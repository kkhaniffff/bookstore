use serde::Serialize;
use sqlx::{FromRow, Type};

#[derive(Serialize, FromRow)]
pub struct Order {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub total_price: i32,
    pub items: Vec<OrderItem>,
}

#[derive(Serialize, FromRow, Type)]
pub struct OrderItem {
    pub id: uuid::Uuid,
    pub price: i32,
    pub amount: i32,
    pub book_id: uuid::Uuid,
    pub book_title: String,
    pub book_author: String,
    pub book_publication_date: chrono::NaiveDate,
}
