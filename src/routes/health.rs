use crate::{AppState, handlers::health};
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health::get_health))
}
