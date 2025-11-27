use axum::{Json, extract::Path, extract::Query, extract::State, http::StatusCode};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::books::Book;
use crate::dtos::books::{BookDto, BookFilterDto};

pub async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<BookDto>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, publication_date, stock_quantity, price)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        &id,
        &payload.title,
        &payload.author,
        &payload.publication_date,
        &payload.stock_quantity,
        &payload.price
    )
    .execute(&pool)
    .await
    .map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn get_books(
    State(pool): State<PgPool>,
    Query(filter): Query<BookFilterDto>,
) -> Result<Json<Vec<Book>>, AppError> {
    let books = sqlx::query_as!(
        Book,
        r#"
            SELECT * FROM books 
            WHERE title LIKE $1 AND author LIKE $2 
            OFFSET $3 LIMIT $4
            "#,
        filter.title(),
        filter.author(),
        filter.offset(),
        filter.limit(),
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::from)?;

    Ok(Json(books))
}

pub async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<BookDto>,
) -> Result<(), AppError> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE books
        SET title = $1, author = $2, publication_date = $3, stock_quantity = $4, price = $5
        WHERE id = $6
        "#,
        &payload.title,
        &payload.author,
        &payload.publication_date,
        &payload.stock_quantity,
        &payload.price,
        id
    )
    .execute(&pool)
    .await
    .map(|res| res.rows_affected())
    .map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(())
    }

}

pub async fn archive_book(State(pool): State<PgPool>, Path(id): Path<uuid::Uuid>) -> Result<(), AppError> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE books
        SET archived = true
        WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await
    .map(|res| res.rows_affected())
    .map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(id.to_string())),
        _ => Ok(())
    }
}
