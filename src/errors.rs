use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrMessage {
    msg: String,
}

impl ErrMessage {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),
    #[error("invalid arguments: {0}")]
    InvalidArgument(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let status = match &self {
            Self::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
        };

        (status, Json(ErrMessage::new(self.to_string()))).into_response()
    }
}
