use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Book {
    pub id: uuid::Uuid,
    pub title: String,
    pub author: String,
    pub publication_date: chrono::NaiveDate,
    pub stock_quantity: i32,
    pub price: i32,
    pub archived: bool,
}

#[derive(FromRow)]
pub struct BookStock {
    pub id: uuid::Uuid,
    pub stock_quantity: i32,
    pub price: i32,
}
