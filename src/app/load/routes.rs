use axum::{Router, routing::{get}};
use sqlx::PgPool;
use super::handlers::*;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/test", get(test))
        .route("/list", get(loads))
        .with_state(pool)
}
