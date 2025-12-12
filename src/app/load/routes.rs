use axum::{Router, routing::{get}};
use sqlx::PgPool;
use super::handlers::*;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        // .route("/list", get(list_loads))
        .route("/test", get(test))
        .route("/", get(loads))
        .with_state(pool)
}
