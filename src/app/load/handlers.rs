use axum::{
    Extension, Json, extract::{OriginalUri, Query, State}, http::{HeaderMap, StatusCode}
};
use sqlx::PgPool;
use serde_json::json;

use crate::app::load::models::{Load, LoadListResponse};
use crate::app::auth::models::AuthUser;
use crate::app::company::models::TenantCompany;

use crate::base::paginations::{PaginatedResponse, PaginationParams, paginate_query_with_tx};
use crate::helper::tenant_context::with_tenant_schema;

// #[axum::debug_handler]
// pub async fn list_loads(
//     headers: HeaderMap,
//     OriginalUri(original_uri): OriginalUri,
//     Query(params): Query<PaginationParams>,
//     State(pool): State<PgPool>,
//     Extension(tenant): Extension<TenantCompany>,
//     Extension(_user): Extension<AuthUser>
// ) -> Json<PaginatedResponse<Load>> {
//     let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();
//     println!("Using tenant schema: {}", tenant.schema_name);

//     with_tenant_schema(&mut tx, &tenant.schema_name).await.unwrap();

//     let sql_count = "SELECT COUNT(*) FROM load_load";

//     let sql_data = r#"
//         SELECT
//             *
//         FROM load_load
//         WHERE is_deleted = FALSE and is_active = TRUE
//         ORDER BY received_date DESC
//         LIMIT $1 OFFSET $2
//     "#;

//     let response =
//         paginate_query_with_tx::<Load>(&mut tx, params, &original_uri, sql_count, sql_data, &headers)
//             .await
//             .unwrap();

//     Json(response)
// }

#[axum::debug_handler]
pub async fn test() -> Json<serde_json::Value> {
    let load = json!({
        "test": "value",
        "id": "12345",
    });

    Json(load)
}

#[axum::debug_handler]
pub async fn loads(
    headers: HeaderMap,
    OriginalUri(original_uri): OriginalUri,
    Query(params): Query<PaginationParams>,
    State(pool): State<PgPool>,
    Extension(tenant): Extension<TenantCompany>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<PaginatedResponse<LoadListResponse>>, StatusCode> {
    println!("user_id: {}", user.id);

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    with_tenant_schema(&mut tx, &tenant.schema_name).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let team_ids: Vec<i64> = match sqlx::query_scalar::<_, i64>(
        "SELECT team_id FROM user_user_teams WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_all(&mut *tx)
    .await
    {
        Ok(ids) => ids,
        Err(e) => {
            eprintln!("Failed to fetch team_ids: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("User {} team IDs: {:?}", user.id, team_ids);

    let sixty_minutes_ago =
        chrono::Utc::now() - chrono::Duration::minutes(60);

    // COUNT QUERY
    let sql_count = "SELECT COUNT(*) FROM load_load";

    let vehicles_subquery = if !team_ids.is_empty() {
        format!(
            "SELECT id FROM owner_vehicle 
             WHERE status = 1 AND registration_status = 4
               AND team_id IN ({})",
            team_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        )
    } else {
        "SELECT id FROM vehicle WHERE status = 1 AND registration_status = 4".to_string()
    };

    let mut sql = format!(
        r#"
        SELECT
            ll.id, ll.received_date, ll.pick_up_at, ll.deliver_to,
            ll.suggested_truck, ll.miles, ll.contact_name, ll.source_name,
            ll.vehicle_type, ll.pick_up_at_state, ll.pick_up_date,
            ll.pick_up_latitude, ll.pick_up_longitude, ll.deliver_to_state,
            ll.delivery_date, ll.miles_out, ll.nearest_vehicles_count,
            ll.broker_company, ll.vehicle_team, ll.vehicle_teams,
            ll.count_day, ll.is_active,
            bc.rating AS broker_rating,

            EXISTS (
                SELECT 1 FROM load_bid 
                WHERE bid.load_id = ll.id
                  AND bid.vehicle_id IN ({vehicles})
            ) AS is_bid,

            EXISTS (
                SELECT 1 FROM load_driver_bid 
                WHERE driver_bid.load_id = ll.id
                  AND driver_bid.vehicle_id IN ({vehicles})
                  AND driver_bid.dispatch_bid_date IS NULL
                  AND driver_bid.created_at >= $3
            ) AS is_driver_bid,

            EXISTS (
                SELECT 1 FROM load_load_is_read_users
                WHERE load_is_read_users.load_id = ll.id
                  AND load_is_read_users.user_id = $4
            ) AS is_read

        FROM load_load ll
        LEFT JOIN broker_company bc ON ll.broker_company = bc.id
        WHERE ll.is_deleted = FALSE
          AND ll.is_active = TRUE
        "#,
        vehicles = vehicles_subquery,
    );

    if tenant.cargo_distance != Some(-1.0) {
        sql.push_str(" AND ll.nearest_vehicles_count > 0 ");
    }

    if !team_ids.is_empty() {
        sql.push_str(&format!(
            " AND EXISTS (
                SELECT 1 FROM load_load_vehicle_teams lvt
                WHERE lvt.load_id = ll.id
                  AND lvt.vehicle_team_id IN ({})
            ) ",
            team_ids
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }

    sql.push_str(" ORDER BY ll.received_date DESC LIMIT $1 OFFSET $2 ");
    
    let res = paginate_query_with_tx::<LoadListResponse>(
        &mut tx,
        params,
        &original_uri,
        sql_count,
        &sql,
        &headers,
        sixty_minutes_ago,
        user.id as i64,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(res))
}