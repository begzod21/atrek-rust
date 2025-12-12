use axum::{
    Json,
    Extension,
    extract::{OriginalUri, Query, State},
    http::HeaderMap,
};
use sqlx::PgPool;

use crate::app::load::models::Load;
use crate::app::auth::models::AuthUser;
use crate::app::company::models::TenantCompany;

use crate::base::paginations::{PaginatedResponse, PaginationParams, paginate_query};
use crate::helper::tenant_context::with_tenant_schema;

#[axum::debug_handler]
pub async fn list_loads(
    headers: HeaderMap,
    OriginalUri(original_uri): OriginalUri,
    Query(params): Query<PaginationParams>,
    State(pool): State<PgPool>,
    Extension(tenant): Extension<TenantCompany>,
    Extension(_user): Extension<AuthUser>
) -> Json<PaginatedResponse<Load>> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();
    println!("Using tenant schema: {}", tenant.schema_name);

    with_tenant_schema(&mut tx, &tenant.schema_name).await.unwrap();

    let sql_count = "SELECT COUNT(*) FROM load_load";

    let sql_data = r#"
        SELECT
            *
        FROM load_load
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "#;

    let response =
        paginate_query::<Load>(&pool, params, &original_uri, sql_count, sql_data, &headers)
            .await
            .unwrap();

    Json(response)
}