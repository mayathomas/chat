use axum::{response::IntoResponse, Extension};
use tracing::info;

use crate::User;

pub(crate) async fn list_chat_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    info!("user: {:?}", user);
    "Hello, World!"
}

pub(crate) async fn create_chat_handler() -> &'static str {
    "Hello, World!"
}

pub(crate) async fn update_chat_handler() -> &'static str {
    "Hello, World!"
}

pub(crate) async fn delete_chat_handler() -> &'static str {
    "Hello, World!"
}
