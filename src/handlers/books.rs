use crate::{
    AppState,
    error::AppError,
    models::books::Book,
    payloads::books::{BookFilterPayload, BookPayload},
    repositories::books as repo,
};
use axum::{Json, extract::Path, extract::Query, extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn create_book(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BookPayload>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let id = repo::insert(&state.db, &payload)
        .await
        .map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn get_books(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<BookFilterPayload>,
) -> Result<Json<Vec<Book>>, AppError> {
    let books = repo::fetch_all(&state.db, filter)
        .await
        .map_err(AppError::from)?;

    Ok(Json(books))
}

pub async fn update_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<BookPayload>,
) -> Result<(), AppError> {
    let rows_affected = repo::update(&state.db, id, &payload)
        .await
        .map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(()),
    }
}

pub async fn archive_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(), AppError> {
    let rows_affected = repo::archive(&state.db, id).await.map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(()),
    }
}
