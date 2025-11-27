use axum::{Json, extract::Query, extract::State, http::StatusCode};
use sqlx::{PgPool, Postgres, Transaction};
use crate::error::AppError;
use crate::models::{orders::{Order, OrderItem}, books::BookStock};
use crate::dtos::orders::{OrderItemDto, Pagination};

pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<Vec<OrderItemDto>>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let book_ids: Vec<uuid::Uuid> = payload.iter().map(|i| i.book_id).collect();
    let mut tx = pool
        .begin()
        .await
        .map_err(AppError::from)?;

    let books = fetch_books(&mut tx, &book_ids).await?;

    validate_stock(&books, &payload)?;

    let order_id = uuid::Uuid::new_v4();
    let total_price = payload
        .iter()
        .map(|item| {
            let book = books.iter().find(|b| b.id == item.book_id).unwrap();
            book.price * item.amount
        })
        .sum();

    create_order_record(&mut tx, &order_id, total_price).await?;

    for item in payload {
        let item_id = uuid::Uuid::new_v4();
        let book = books.iter().find(|b| b.id == item.book_id).unwrap();

        create_order_item(
            &mut tx,
            &order_id,
            &item_id,
            &item.book_id,
            book.price,
            item.amount,
        )
        .await?;

        update_stock(&mut tx, &item.book_id, item.amount).await?;
    }

    tx.commit()
        .await
        .map_err(AppError::from)?;
    Ok((StatusCode::CREATED, Json(order_id)))
}

pub async fn get_orders(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Order>>, AppError> {
    let res = sqlx::query_as!(
        Order,
        r#"
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
    .await
    .map_err(AppError::from)?;

    Ok(Json(res))
}

async fn fetch_books(
    tx: &mut Transaction<'_, Postgres>,
    book_ids: &Vec<uuid::Uuid>,
) -> Result<Vec<BookStock>, AppError> {
    sqlx::query_as!(
        BookStock,
        r#"
        SELECT id, price, stock_quantity
        FROM books
        WHERE archived IS FALSE AND id IN (SELECT unnest($1::uuid[]))
        "#,
        book_ids
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(AppError::from)
}

fn validate_stock(books: &Vec<BookStock>, payload: &Vec<OrderItemDto>) -> Result<(), AppError> {
    for item in payload {
        let book = books
            .iter()
            .find(|b| b.id == item.book_id)
            .ok_or(AppError::NotFound(item.book_id.to_string()))?;

        if item.amount > book.stock_quantity {
            return Err(AppError::BadRequest(format!(
                        "Not enough stock for book {}",
                        book.id
            )));
        }
    }
    Ok(())
}

async fn create_order_record(
    tx: &mut Transaction<'_, Postgres>,
    order_id: &uuid::Uuid,
    total_price: i32,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO orders (id, created_at, total_price)
        VALUES ($1, $2, $3)
        "#,
        order_id,
        &chrono::Utc::now(),
        total_price
    )
    .execute(&mut **tx)
    .await
    .map_err(AppError::from)?;

    Ok(())
}

async fn create_order_item(
    tx: &mut Transaction<'_, Postgres>,
    order_id: &uuid::Uuid,
    item_id: &uuid::Uuid,
    book_id: &uuid::Uuid,
    price: i32,
    amount: i32,
) -> Result<(), AppError> {
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
    .execute(&mut **tx)
    .await
    .map_err(AppError::from)?;

    Ok(())
}

async fn update_stock(
    tx: &mut Transaction<'_, Postgres>,
    book_id: &uuid::Uuid,
    amount: i32,
) -> Result<(), AppError> {
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
    .map_err(AppError::from)?;

    Ok(())
}
