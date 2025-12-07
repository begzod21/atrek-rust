use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use super::handlers::*;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/list", get(list_loads))
        .route("/random", post(create_random_load))
        .with_state(pool)
}
