use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Book {
    id: uuid::Uuid,
    title: String,
    author: String,
    publication_date: Option<chrono::NaiveDate>,
    stock_quantity: i32,
    price: i32,
    archived: bool,
}

#[derive(Debug, Deserialize)]
pub struct BookDto {
    title: String,
    author: String,
    publication_date: chrono::NaiveDate,
    stock_quantity: i32,
    price: i32,
}

pub async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<BookDto>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    let id = uuid::Uuid::new_v4();

    let res = sqlx::query!(
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
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(id))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_books(State(pool): State<PgPool>) -> Result<Json<Vec<Book>>, StatusCode> {
    let res = sqlx::query_as!(Book, "SELECT * FROM books")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(books) => Ok(Json(books)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<BookDto>,
) -> StatusCode {
    let res = sqlx::query!(
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
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn archive_book(State(pool): State<PgPool>, Path(id): Path<uuid::Uuid>) -> StatusCode {
    let res = sqlx::query!(
        r#"
        UPDATE books
        SET archived = true
        WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
