use crate::{
    dtos::orders::{OrderItemDto, Pagination},
    error::AppError,
    models::{books::BookStock, orders::Order},
    repositories::{books as book_repo, orders as order_repo},
};
use axum::{Json, extract::Query, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<Vec<OrderItemDto>>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let mut tx = pool.begin().await.map_err(AppError::from)?;

    let book_ids: Vec<uuid::Uuid> = payload.iter().map(|i| i.book_id).collect();
    let books = book_repo::get_stock(&mut tx, &book_ids)
        .await
        .map_err(AppError::from)?;

    validate_stock(&books, &payload)?;

    let total_price = payload
        .iter()
        .map(|i| {
            let book = books.iter().find(|b| b.id == i.book_id).unwrap(); // already validated
            book.price * i.amount
        })
        .sum();

    let order_id = order_repo::insert(&mut tx, total_price)
        .await
        .map_err(AppError::from)?;

    for item in payload {
        let book = books.iter().find(|b| b.id == item.book_id).unwrap(); // already validated

        order_repo::insert_item(&mut tx, &order_id, &book.id, book.price, item.amount)
            .await
            .map_err(AppError::from)?;

        book_repo::decrement_stock(&mut tx, &item.book_id, item.amount)
            .await
            .map_err(AppError::from)?;
    }

    tx.commit().await.map_err(AppError::from)?;
    Ok((StatusCode::CREATED, Json(order_id)))
}

pub async fn get_orders(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Order>>, AppError> {
    let orders = order_repo::fetch_all(&pool, pagination)
        .await
        .map_err(AppError::from)?;

    Ok(Json(orders))
}

fn validate_stock(books: &[BookStock], payload: &Vec<OrderItemDto>) -> Result<(), AppError> {
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
