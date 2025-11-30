use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

pub enum AppError {
    Database(String),
    NotFound(String),
    BadRequest(String),

    // Auth:
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
    Forbidden(String),
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
            AppError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())
            }
            AppError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            AppError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };

        let body = Json(ErrorBody { error: message });

        (status, body).into_response()
    }
}
