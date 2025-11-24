use crate::handlers::health;
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn router() -> Router<Pool<Postgres>> {
    Router::new().route("/health", get(health::get_health))
}
