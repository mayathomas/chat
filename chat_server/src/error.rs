use axum::{
    response::{IntoResponse, Response},
    Json,
};
use jwt_simple::reexports::serde_json::json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("http header parser error: {0}")]
    InvalidHeaderValue(#[from] axum::http::header::InvalidHeaderValue),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::SqlxError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => axum::http::StatusCode::FORBIDDEN,
            AppError::InvalidHeaderValue(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EmailAlreadyExists(_) => axum::http::StatusCode::CONFLICT,
        };
        (status, Json(json!(ErrorOutput::new(self.to_string())))).into_response()
    }
}
