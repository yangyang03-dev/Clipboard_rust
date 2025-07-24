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
#[tokio::main]
async fn main() {
    let message_store: MessageStore = Arc::new(Mutex::new(Vec::new()));
    let taglist_store: TagListStore = Arc::new(Mutex::new(Vec::new()));
    let file_store: FileStore = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
     .merge(message_routes(message_store.clone()))
     .merge(taglist_routes(taglist_store.clone()))
     .merge(file_routes(file_store.clone()))
     .route("/", get(root_handler))
     .layer(
        CorsLayer::new()
            .allow_origin(Any) // OR .allow_origin("http://localhost:9000".parse().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE])
            .allow_headers(Any),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to the Clipboard API!"
}