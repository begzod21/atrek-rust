use serde::{Serialize, Deserialize};
use sqlx::FromRow;


use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Load {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct LoadListResponse {
    pub id: i64,
    pub received_date: chrono::DateTime<chrono::Utc>,
    pub pick_up_at: Option<String>,
    pub deliver_to: Option<String>,
    pub suggested_truck: Option<String>,
    pub miles: Option<i32>,
    pub contact_name: Option<String>,
    pub source_name: Option<String>,
    pub vehicle_type: Option<String>,
    pub pick_up_at_state: Option<String>,
    pub pick_up_date: Option<chrono::DateTime<chrono::Utc>>,
    pub pick_up_latitude: Option<f64>,
    pub pick_up_longitude: Option<f64>,
    pub deliver_to_state: Option<String>,
    pub delivery_date: Option<chrono::DateTime<chrono::Utc>>,
    pub miles_out: Option<i32>,
    pub nearest_vehicles_count: Option<i32>,
    pub broker_company_id: Option<i64>,
    pub vehicle_team: Option<sqlx::types::Json<Vec<i64>>>,
    pub vehicle_teams: Option<sqlx::types::Json<Vec<i64>>>,
    pub count_day: Option<i32>,
    pub is_active: bool,
    #[sqlx(rename = "broker_rating")]
    pub broker_rating: Option<f64>,
    pub is_bid: bool,
    pub is_driver_bid: bool,
    pub is_read: bool,
}
