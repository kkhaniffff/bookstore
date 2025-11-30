use crate::{AppState, handlers::users};
use axum::Router;
use axum::routing::{get, put};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users/me", get(users::current_user))
        .route("/users/password", put(users::change_password))
}
