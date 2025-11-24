use crate::handlers::books;
use axum::Router;
use axum::routing::{delete, get, post, put};
use sqlx::{Pool, Postgres};

pub fn router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/books", post(books::create_book))
        .route("/books", get(books::get_books))
        .route("/books/{id}", put(books::update_book))
        .route("/books/{id}", delete(books::archive_book))
}
