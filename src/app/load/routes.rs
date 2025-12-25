use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use super::handlers::*;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/test", get(test))
        .route("/list", get(loads))
        .route("/postal-webhook", post(postal_webhook))
        .with_state(pool)
}
