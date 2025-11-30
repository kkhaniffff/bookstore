use crate::{AppState, handlers::orders};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/orders", post(orders::create_order))
        .route("/orders", get(orders::get_orders))
}
