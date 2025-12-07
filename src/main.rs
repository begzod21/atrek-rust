mod db;
mod app;
mod utils;
mod base;

use db::connection::create_pool;
use utils::config::Config;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await;

    let app = app::app_routes(pool.clone());

    let addr = format!("127.0.0.1:{}", config.port);
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
