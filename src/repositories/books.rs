use crate::{
    dtos::books::{BookDto, BookFilterDto},
    models::books::{Book, BookStock},
};
use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

pub async fn insert(pool: &PgPool, dto: &BookDto) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO books (id, title, author, publication_date, stock_quantity, price)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        dto.title,
        dto.author,
        dto.publication_date,
        dto.stock_quantity,
        dto.price
    )
    .execute(pool)
    .await
    .map(|_| id)
}

pub async fn fetch_all(pool: &PgPool, filter: BookFilterDto) -> Result<Vec<Book>, Error> {
    sqlx::query_as!(
        Book,
        r#"
            SELECT 
                    id,
                    title,
                    author,
                    publication_date,
                    stock_quantity,
                    price,
                    archived
            FROM books
            WHERE title LIKE $1 AND author LIKE $2
            OFFSET $3 LIMIT $4
        "#,
        filter.title(),
        filter.author(),
        filter.offset(),
        filter.limit(),
    )
    .fetch_all(pool)
    .await
}

pub async fn update(pool: &PgPool, id: Uuid, dto: &BookDto) -> Result<u64, Error> {
    sqlx::query!(
        r#"
            UPDATE books
            SET title = $1,
                author = $2,
                publication_date = $3,
                stock_quantity = $4,
                price = $5
            WHERE id = $6
        "#,
        dto.title,
        dto.author,
        dto.publication_date,
        dto.stock_quantity,
        dto.price,
        id
    )
    .execute(pool)
    .await
    .map(|res| res.rows_affected())
}

pub async fn archive(pool: &PgPool, id: Uuid) -> Result<u64, Error> {
    sqlx::query!("UPDATE books SET archived = true WHERE id = $1", id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

pub async fn get_stock(
    tx: &mut Transaction<'_, Postgres>,
    book_ids: &[Uuid],
) -> Result<Vec<BookStock>, Error> {
    sqlx::query_as!(
        BookStock,
        r#"
            SELECT 
                    id,
                    stock_quantity,
                    price
            FROM books
            WHERE archived IS FALSE
            AND id = ANY($1)
        "#,
        book_ids
    )
    .fetch_all(&mut **tx)
    .await
}

pub async fn decrement_stock(
    tx: &mut Transaction<'_, Postgres>,
    book_id: &Uuid,
    amount: i32,
) -> Result<u64, Error> {
    sqlx::query!(
        r#"
            UPDATE books
            SET stock_quantity = stock_quantity - $1
            WHERE id = $2
        "#,
        amount,
        book_id
    )
    .execute(&mut **tx)
    .await
    .map(|res| res.rows_affected())
}
