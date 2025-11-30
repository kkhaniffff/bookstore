mod auth;
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
    pub jwt_secret: String,
    pub jwt_ttl: i64,
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

    let jwt_secret = std::env::var("JWT_SECRET").expect("Missing JWT_SECRET env variable");
    let jwt_ttl = std::env::var("JWT_TTL")
        .expect("Missing JWT_TTL env variable")
        .parse()
        .expect("JWT_TTL must be an integer (in seconds)");

    let state = AppState {
        db: pool,
        jwt_secret,
        jwt_ttl,
    };

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::books::router())
        .merge(routes::orders::router())
        .merge(routes::auth::router())
        .with_state(Arc::new(state));

    let addr = "0.0.0.0:3000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind a listener");

    println!("Started successfully on {addr}");
    serve(listener, app).await.unwrap();
}
