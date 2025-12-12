pub mod load;
pub mod company;
pub mod auth;

use axum::Router;
use sqlx::PgPool;
use load::routes as load_routes;

pub fn app_routes(pool: PgPool) -> Router {
    Router::new()
        .nest("/app/api/load", load_routes::routes(pool))
}
