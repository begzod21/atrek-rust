mod db;
mod app;
mod utils;
mod base;
mod middleware;
mod helper;

use axum::{http::{Method, HeaderValue}, Extension, middleware as axum_middleware};
use db::connection::create_pool;
use tokio::net::TcpListener;
use utils::config::Config;

use middleware::tenant::tenant_middleware;
// use middleware::auth::auth_middleware;

use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await;

    let cors = CorsLayer::new()
        .allow_origin([
            "https://test.atrek.icu".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8088".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let app = app::app_routes(pool.clone())
        .layer(axum_middleware::from_fn(tenant_middleware))
        // .layer(axum_middleware::from_fn(auth_middleware))
        .layer(Extension(pool.clone()))
        .layer(cors);

    let addr = format!("127.0.0.1:{}", config.port);
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
