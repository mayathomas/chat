mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod utils;

use std::{fmt, ops::Deref, sync::Arc};

use anyhow::Context;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
pub use config::AppConfig;
pub use error::{AppError, ErrorOutput};
use handlers::{
    create_chat_handler, delete_chat_handler, file_handler, get_chat_handler, index_handler,
    list_chat_handler, list_chat_users_handler, list_message_handler, send_message_handler,
    signin_handler, signup_handler, update_chat_handler, upload_handler,
};
use middleware::{set_layer, verify_token};
pub use models::User;
pub use models::*;
use sqlx::PgPool;

use tokio::fs;
use utils::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
    pub(crate) pool: PgPool,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;

    let api = Router::new()
        .route("/users", get(list_chat_users_handler))
        .route("/chats", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chats/:id",
            get(get_chat_handler)
                .patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chats/:id/messages", get(list_message_handler))
        .route("/upload", post(upload_handler))
        .route("/files/:ws_id/*path", get(file_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        // routes doesn't need token verification
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    let app = set_layer(app);
    Ok(app)
}

#[cfg(test)]
mod test_util {
    use std::path::Path;

    use super::*;
    use sqlx::{Executor, PgPool};
    use sqlx_db_tester::TestPg;

    use crate::AppState;

    pub async fn get_test_pool(url: Option<&str>) -> (TestPg, PgPool) {
        let url = match url {
            Some(url) => url.to_string(),
            None => "postgres://postgres:postgres@localhost:5432".to_string(),
        };
        let tdb = TestPg::new(url, Path::new("../migrations"));
        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/test.sql").split(';');
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");
        (tdb, pool)
    }

    impl AppState {
        pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
            let config = AppConfig::load()?;
            let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
            let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
            let pos = config.server.db_url.rfind('/').expect("Invalid db_url");
            let server_url = &config.server.db_url[..pos];
            let (tdb, pool) = get_test_pool(Some(server_url)).await;
            let state = Self {
                inner: Arc::new(AppStateInner {
                    config,
                    ek,
                    dk,
                    pool,
                }),
            };
            Ok((tdb, state))
        }
    }
}
