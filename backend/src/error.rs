use std::env::VarError;
use std::net::AddrParseError;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    BadRequest(String),
    Conflict(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Forbidden(msg) => {(StatusCode::FORBIDDEN, msg)}
        };

        let body = Json(json!({
            "message": message,
            "status": false,
        }));

        (status, body).into_response()
    }
}

impl From<AddrParseError> for AppError{
    fn from(error: AddrParseError) -> Self{
        AppError::Internal(error.to_string())
    }
}

impl From<std::io::Error> for AppError{
    fn from(error: std::io::Error) -> Self{
        AppError::Internal(error.to_string())
    }
}

impl From<VarError> for AppError{
    fn from(error: VarError) -> Self{
        AppError::Internal(error.to_string())
    }
}
impl From<sqlx::Error> for AppError{
    fn from(error: sqlx::Error) -> Self{
        AppError::Internal(error.to_string())
    }
}

impl From<reqwest::Error> for AppError{
    fn from(error: reqwest::Error) -> Self{
        AppError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for AppError{
    fn from(error: serde_json::Error) -> Self{
        AppError::Internal(error.to_string())
    }
}