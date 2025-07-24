use axum::{
    Router,
    http::StatusCode,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod routes;
mod models;

use routes::taglist::taglist_routes;
use tower_http::cors::{CorsLayer, Any};
use axum::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from `.env` if running locally
    dotenv().ok();

    // Connect to the PostgreSQL database
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // CORS config (allow all origins)
    let cors = CorsLayer::new().allow_origin(Any);

    // Compose the application routes
    let app = Router::new()
        .merge(taglist_routes(pool.clone()))
        .layer(cors);

    // Start the Axum server
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    println!("🚀 Running at http://{addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}