use crate::{AppState, handlers::books};
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/books", post(books::create_book))
        .route("/books", get(books::get_books))
        .route("/books/{id}", put(books::update_book))
        .route("/books/{id}", delete(books::archive_book))
}
