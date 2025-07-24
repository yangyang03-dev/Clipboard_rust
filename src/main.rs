mod models;
mod routes;

use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use std::net::SocketAddr;
use routes::message::{message_routes, MessageStore};
use std::sync::{Arc, Mutex};
use routes::taglist::{taglist_routes, TagListStore};
use routes::file::{file_routes, FileStore};
use sqlx::postgres::PgPoolOptions;
use axum::Router;
#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("DB connect failed");

    let app = Router::new()
        .route("/taglists", get(get_all_taglists))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to the Clipboard API!"
}