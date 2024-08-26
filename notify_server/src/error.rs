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
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),
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
            AppError::JwtError(_) => axum::http::StatusCode::FORBIDDEN,
            AppError::IoError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!(ErrorOutput::new(self.to_string())))).into_response()
    }
}
