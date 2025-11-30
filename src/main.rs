mod error;
mod handlers;
mod models;
mod payloads;
mod repositories;
mod routes;

use axum::{Router, serve};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::books::router())
        .merge(routes::orders::router())
        .with_state(Arc::new(AppState { db: pool }));

    let addr = "0.0.0.0:3000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind a listener");

    println!("Started successfully on {addr}");
    serve(listener, app).await.unwrap();
}
