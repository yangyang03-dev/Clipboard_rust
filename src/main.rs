use axum::{
    http::Method,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use tower::ServiceBuilder;

mod routes;
mod models;

use routes::taglist::taglist_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:admin@localhost:5432/clipboard".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // ✅ 直接传递数组到 allow_methods
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]);

    // ✅ 使用 ServiceBuilder 创建中间件栈
    let middleware_stack = ServiceBuilder::new()
        .layer(cors);

    // ✅ 创建应用并应用中间件
    let app = Router::new()
        .merge(taglist_routes(pool))
        .layer(middleware_stack);

    // ✅ 绑定地址
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    println!("🚀 Server running at http://{}", addr);

    // ✅ 启动服务器
    axum::serve(
        TcpListener::bind(addr).await?,
        app.into_make_service()
    )
    .await?;

    Ok(())
}