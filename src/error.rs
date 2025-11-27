use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: String,
}

#[derive(Debug)]
pub enum AppError {
    Database(String),
    NotFound(String),
    BadRequest(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {msg}"),
            ),
            AppError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Entity with id={id} not found"),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("Bad request: {msg}")),
        };

        let body = Json(ErrorBody { error: message });

        (status, body).into_response()
    }
}
