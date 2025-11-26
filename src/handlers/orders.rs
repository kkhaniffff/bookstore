use axum::{Json, extract::Query, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Type};

#[derive(Serialize, FromRow)]
pub struct Order {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    total_price: i32,
    items: Vec<OrderItem>,
}

#[derive(Serialize, FromRow, Type)]
pub struct OrderItem {
    id: uuid::Uuid,
    price: i32,
    amount: i32,
    book_id: uuid::Uuid,
    book_title: String,
    book_author: String,
    book_publication_date: Option<chrono::NaiveDate>,
}

#[derive(FromRow)]
pub struct Book {
    id: uuid::Uuid,
    price: i32,
    stock_quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemDto {
    book_id: uuid::Uuid,
    amount: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    offset: Option<i64>,
    limit: Option<i64>,
}

impl Pagination {
    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    fn limit(&self) -> i64 {
        self.limit.unwrap_or(100)
    }
}

pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<Vec<OrderItemDto>>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    let book_ids: Vec<uuid::Uuid> = payload.iter().map(|i| i.book_id).collect();
    let books = fetch_books(&pool, &book_ids).await?;

    validate_stock(&books, &payload)?;

    let transaction = pool.begin().await.unwrap();

    let order_id = uuid::Uuid::new_v4();
    let total_price = payload
        .iter()
        .map(|item| {
            let book = books.iter().find(|b| b.id == item.book_id).unwrap(); // already validated
            book.price * item.amount
        })
        .sum();

    create_order_record(&pool, &order_id, total_price).await?;

    for item in payload {
        let item_id = uuid::Uuid::new_v4();
        let book = books.iter().find(|b| b.id == item.book_id).unwrap(); // already validated

        create_order_item(
            &pool,
            &order_id,
            &item_id,
            &item.book_id,
            book.price,
            item.amount,
        )
        .await?;

        update_stock(&pool, &item.book_id, item.amount).await?;
    }

    match transaction.commit().await {
        Ok(_) => Ok((StatusCode::CREATED, Json(order_id))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_orders(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    let res = sqlx::query_as!(Order, r#"
            SELECT 
                o.id, 
                o.created_at, 
                o.total_price,
                ARRAY_AGG(ROW(i.id, i.price, i.amount, b.id, b.title, b.author, b.publication_date)) AS "items!: Vec<OrderItem>"
                FROM orders o
                JOIN order_items i ON o.id = i.order_id
                JOIN books b on b.id = i.book_id
                GROUP BY o.id
                ORDER BY o.created_at DESC
                OFFSET $1 LIMIT $2
            "#,
            pagination.offset(),
            pagination.limit(),
        )
        .fetch_all(&pool)
        .await;

    match res {
        Ok(orders) => Ok(Json(orders)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn fetch_books(pool: &PgPool, book_ids: &Vec<uuid::Uuid>) -> Result<Vec<Book>, StatusCode> {
    sqlx::query_as!(
        Book,
        r#"
        SELECT id, price, stock_quantity
        FROM books
        WHERE archived IS FALSE AND id IN (SELECT unnest($1::uuid[]))
        "#,
        book_ids
    )
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn validate_stock(books: &Vec<Book>, payload: &Vec<OrderItemDto>) -> Result<(), StatusCode> {
    for item in payload {
        let book = books
            .iter()
            .find(|b| b.id == item.book_id)
            .ok_or(StatusCode::NOT_FOUND)?;

        if item.amount > book.stock_quantity {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    Ok(())
}

async fn create_order_record(
    pool: &PgPool,
    order_id: &uuid::Uuid,
    total_price: i32,
) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO orders (id, created_at, total_price)
        VALUES ($1, $2, $3)
        "#,
        order_id,
        &chrono::Utc::now(),
        total_price
    )
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn create_order_item(
    pool: &PgPool,
    order_id: &uuid::Uuid,
    item_id: &uuid::Uuid,
    book_id: &uuid::Uuid,
    price: i32,
    amount: i32,
) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO order_items (id, order_id, book_id, price, amount)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        item_id,
        order_id,
        book_id,
        price,
        amount
    )
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn update_stock(pool: &PgPool, book_id: &uuid::Uuid, amount: i32) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        UPDATE books
        SET stock_quantity = stock_quantity - $1
        WHERE id = $2
        "#,
        amount,
        book_id
    )
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
