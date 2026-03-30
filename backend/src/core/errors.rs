use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BrowserError(String),
    NetworkError(String),
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::BrowserError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NetworkError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
            "type": match &self {
                AppError::BrowserError(_) => "browser_error",
                AppError::NetworkError(_) => "network_error",
                AppError::ValidationError(_) => "validation_error",
                AppError::NotFound(_) => "not_found",
                AppError::InternalError(_) => "internal_error",
            }
        }));

        (status, body).into_response()
    }
}

impl From<headless_chrome::error::Error> for AppError {
    fn from(err: headless_chrome::error::Error) -> Self {
        AppError::BrowserError(format!("Browser automation error: {}", err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(format!("Network error: {}", err))
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(format!("Validation failed: {:?}", err))
    }
}
