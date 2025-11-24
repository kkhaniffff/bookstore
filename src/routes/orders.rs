use crate::handlers::orders;
use axum::Router;
use axum::routing::{get, post};
use sqlx::{Pool, Postgres};

pub fn router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/orders", post(orders::create_order))
        .route("/orders", get(orders::get_orders))
}
