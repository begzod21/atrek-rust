use crate::app::company::models::TenantCompany;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

pub async fn tenant_middleware(
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, StatusCode> {
    let domain_url = req.headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost")
        .to_string();
    println!("Domain URL: {}", domain_url);

    let pool = req
        .extensions()
        .get::<PgPool>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let tenant = sqlx::query_as::<_, TenantCompany>(
            r#"SELECT id, schema_name, domain_url, cargo_distance 
               FROM company_company 
               WHERE domain_url = $1 OR domain_url ILIKE $2"#
        )
        .bind(domain_url.clone())
        .bind(format!("%{}%", domain_url.trim()))
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    println!("{}", tenant.schema_name);

    req.extensions_mut().insert(tenant);

    Ok(next.run(req).await)
}
