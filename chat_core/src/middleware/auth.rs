use super::TokenVerify;
use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::warn;

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await;
    let req = match token {
        Ok(TypedHeader(Authorization(bearer))) => {
            let token = bearer.token();
            match state.verify(token) {
                Ok(user) => {
                    let mut req = Request::from_parts(parts, body);
                    req.extensions_mut().insert(user);
                    req
                }
                Err(e) => {
                    let msg = format!("verify token failed: {:?}", e);
                    warn!(msg);
                    return (StatusCode::UNAUTHORIZED, msg).into_response();
                }
            }
        }
        Err(e) => {
            let msg = format!("parse Authorization header failed: {}", e);
            warn!(msg);
            return (StatusCode::UNAUTHORIZED, msg).into_response();
        }
    };
    next.run(req).await
}
