mod db;
mod app;
mod utils;
mod base;
mod middleware;
mod helper;

use axum::{Extension, middleware as axum_middleware};
use db::connection::create_pool;
use middleware::tenant::tenant_middleware;
use tokio::net::TcpListener;
use utils::config::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await;

    let app = app::app_routes(pool.clone())
        .layer(axum_middleware::from_fn(tenant_middleware))
        .layer(Extension(pool.clone())); 

    let addr = format!("127.0.0.1:{}", config.port);
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
