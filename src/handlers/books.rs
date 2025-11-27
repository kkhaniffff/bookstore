use crate::{
    dtos::books::{BookDto, BookFilterDto},
    error::AppError,
    models::books::Book,
    repositories::books as repo,
};
use axum::{Json, extract::Path, extract::Query, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<BookDto>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let id = repo::insert(&pool, &payload)
        .await
        .map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn get_books(
    State(pool): State<PgPool>,
    Query(filter): Query<BookFilterDto>,
) -> Result<Json<Vec<Book>>, AppError> {
    let books = repo::fetch_all(&pool, filter)
        .await
        .map_err(AppError::from)?;

    Ok(Json(books))
}

pub async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<BookDto>,
) -> Result<(), AppError> {
    let rows_affected = repo::update(&pool, id, &payload)
        .await
        .map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(()),
    }
}

pub async fn archive_book(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(), AppError> {
    let rows_affected = repo::archive(&pool, id).await.map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(()),
    }
}
