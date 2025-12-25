use axum::{
    Extension, Json, extract::{OriginalUri, Query, State}, body::{Bytes}, http::{HeaderMap, StatusCode}
};
use sqlx::PgPool;
use serde_json::json;
use chrono::{Utc, Duration};
use mail_parser::{MessageParser};

use crate::app::load::models::{LoadListResponse};
use crate::app::auth::models::AuthUser;
use crate::app::company::models::TenantCompany;

use crate::base::paginations::{PaginatedResponse, PaginationParams, paginate_query_with_tx};
use crate::helper::tenant_context::with_tenant_schema;

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

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    with_tenant_schema(&mut tx, &tenant.schema_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let team_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT team_id FROM user_user_teams WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sixty_minutes_ago = Utc::now() - Duration::minutes(60);

    let sql_count = "SELECT COUNT(*) FROM load_load ll WHERE ll.is_deleted = FALSE AND ll.is_active = TRUE";

    let vehicles_subquery = if !team_ids.is_empty() {
        format!(
            "SELECT id FROM owner_vehicle WHERE status = 1 AND registration_status = 4 AND team_id IN ({})",
            team_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        )
    } else {
        "SELECT id FROM owner_vehicle WHERE status = 1 AND registration_status = 4".to_string()
    };

    let mut sql = format!(
        r#"
        SELECT
            ll.id, ll.received_date, ll.pick_up_at, ll.deliver_to,
            ll.suggested_truck, ll.miles, ll.contact_name, ll.source_name,
            ll.vehicle_type, ll.pick_up_at_state, ll.pick_up_date,
            ll.pick_up_latitude::float8 AS pick_up_latitude,
            ll.pick_up_longitude::float8 AS pick_up_longitude, ll.deliver_to_state,
            ll.delivery_date, ll.miles_out, ll.nearest_vehicles_count,
            ll.broker_company_id,
            ll.vehicle_team,
            (SELECT array_agg(team_id) FROM load_load_vehicle_teams lvt WHERE lvt.load_id = ll.id) AS vehicle_teams,
            ll.count_day, ll.is_active,
            bc.rating AS broker_rating,
            EXISTS (
                SELECT 1 FROM load_bid AS bid
                WHERE bid.load_id = ll.id
                  AND bid.vehicle_id IN ({vehicles})
            ) AS is_bid,
            EXISTS (
                SELECT 1 FROM load_driverbid AS driver_bid
                WHERE driver_bid.load_id = ll.id
                  AND driver_bid.vehicle_id IN ({vehicles})
                  AND driver_bid.dispatch_bid_date IS NULL
                  AND driver_bid.created_at >= $3
            ) AS is_driver_bid,
            EXISTS (
                SELECT 1 FROM load_load_is_read_users AS load_is_read_users
                WHERE load_is_read_users.load_id = ll.id
                  AND load_is_read_users.user_id = $4
            ) AS is_read
        FROM load_load ll
        LEFT JOIN broker_brokercompany bc ON ll.broker_company_id = bc.id
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
                  AND lvt.team_id IN ({})
            ) ",
            team_ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")
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


#[derive(serde::Serialize)]
pub struct WebhookResponse {
    result: bool
}

#[axum::debug_handler]
pub async fn postal_webhook(
    State(pool): State<PgPool>,
    Extension(tenant): Extension<TenantCompany>,
    body: Bytes
) -> Result<Json<WebhookResponse>, StatusCode> {
    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    with_tenant_schema(&mut tx, &tenant.schema_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let raw_body = String::from_utf8_lossy(&body).to_string();

    let message = MessageParser::default().parse(&raw_body).unwrap();

    if let Some(from) = message.from() {
        if let Some(addr) = from.first() {
            println!("From Name: {:?}", addr.name);
            println!("From Email: {:?}", addr.address);
        }
    }

    if let Some(to) = message.to() {
        if let Some(addr) = to.first() {
            println!("To Name: {:?}", addr.name);
            println!("To Email: {:?}", addr.address);
        }
    }

    if let Some(subject) = message.subject() {
        println!("Subject: {}", subject);
    }

    if let Some(msg_id) = message.message_id() {
        println!("Message-ID: {}", msg_id);
    }

    if let Some(date) = message.date() {
        println!("Date: {}", date.to_rfc3339());
    }

    if let Some(reply_to) = message.reply_to() {
        if let Some(addr) = reply_to.first() {
            println!("Reply-To Email: {:?}", addr.address);
        }
    }



    Ok(Json(WebhookResponse { result: true }))
    
}
