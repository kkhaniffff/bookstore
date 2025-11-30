use crate::{AppState, handlers::auth};
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
}
