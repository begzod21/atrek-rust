use axum::{
    Json,
    Extension,
    extract::{OriginalUri, Query, State},
    http::HeaderMap,
};
use chrono::Utc;
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::load::models::Load;
use crate::base::paginations::{PaginatedResponse, PaginationParams, paginate_query};
use crate::helper::tenant_context::with_tenant_schema;


fn gen_random_coords() -> (f64, f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let pick_lat: f64 = rng.gen_range(-90.0..90.0);
    let pick_lon: f64 = rng.gen_range(-180.0..180.0);
    let deliver_lat: f64 = rng.gen_range(-90.0..90.0);
    let deliver_lon: f64 = rng.gen_range(-180.0..180.0);
    (pick_lon, pick_lat, deliver_lon, deliver_lat)
}

#[axum::debug_handler]
pub async fn list_loads(
    headers: HeaderMap,
    OriginalUri(original_uri): OriginalUri,
    Query(params): Query<PaginationParams>,
    State(pool): State<PgPool>,
    tenant: Extension<String>,
) -> Json<PaginatedResponse<Load>> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();
    print!("Using tenant schema: {}\n", tenant.0);

    with_tenant_schema(&mut tx, &tenant).await.unwrap();

    let sql_count = "SELECT COUNT(*) FROM loads";

    let sql_data = r#"
        SELECT
            *
        FROM loads
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "#;

    let response =
        paginate_query::<Load>(&pool, params, &original_uri, sql_count, sql_data, &headers)
            .await
            .unwrap();

    Json(response)
}

#[axum::debug_handler]
pub async fn create_random_load(State(pool): State<PgPool>) -> Json<Load> {
    let (pick_lon, pick_lat, deliver_lon, deliver_lat) = gen_random_coords();

    let load = Load {
        id: Uuid::new_v4(),
        created_at: Some(Utc::now()),

        is_active: Some(true),
        broker_id: None,
        broker_company_id: None,
        from_address: None,
        to_address: None,
        source_id: 1,
        source_name: None,
        subject: None,
        message_id: None,
        thread_id: None,
        mail_id: None,
        pick_up_at: None,
        pick_up_latitude: Some(pick_lat),
        pick_up_longitude: Some(pick_lon),
        pick_up_state: None,
        pick_up_zip: None,
        deliver_to: None,
        deliver_to_latitude: Some(deliver_lat),
        deliver_to_longitude: Some(deliver_lon),
        deliver_zip: None,
        duration: None,
        distance: None,
        order_number: None,
        contact_name: None,
        contact_address: None,
        contact_phone: None,
        contact_email: None,
        contact_person: None,
        received_date: None,
        pick_up_date: None,
        delivery_date: None,
        pick_up_date_raw: None,
        delivery_date_raw: None,
        expire_date_raw: None,
        expire_date: None,
        notes: None,
        pays: None,
        posted_amount: None,
        miles: None,
        pieces: None,
        stackable: None,
        hazardous: None,
        fast_load: None,
        dock_level: None,
        weight: None,
        dims: None,
        suggested_truck: None,
        vehicle_type: None,
        owner_name: None,
        owner_email: None,
        owner_key: None,
        pick_up_at_geo: None,
        deliver_to_geo: None,
        pick_up_at_state: None,
        deliver_to_state: None,
        market_pays: None,
        raw: None,
        miles_out: None,
        nearest_vehicles_count: None,
        vehicle_team: None,
        count_day: None,
    };

    sqlx::query(
        r#"
        INSERT INTO loads (id, pick_up_location, deliver_to_location, created_at, source_id)
        VALUES ($1,
                $6, $7)
        "#,
    )
    .bind(load.id)
    .bind(load.created_at)
    .bind(load.source_id)
    .execute(&pool)
    .await
    .unwrap();

    Json(load)
}
