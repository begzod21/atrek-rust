use crate::app::company::models::TenantCompany;
use crate::helper::build_url::build_absolute_url;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

pub async fn tenant_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let domain_url = build_absolute_url(req.headers());

    let pool = req
        .extensions()
        .get::<PgPool>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let tenant = sqlx::query_as!(
        TenantCompany,
        r#"SELECT id, schema_name, domain_url FROM company WHERE domain_url = $1"#,
        domain_url
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    req.extensions_mut().insert(tenant);

    Ok(next.run(req).await)
}
