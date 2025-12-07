use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Load {
    pub id: Uuid,

    pub is_active: Option<bool>,

    pub broker_id: Option<i32>,
    pub broker_company_id: Option<i32>,

    pub from_address: Option<String>,
    pub to_address: Option<String>,

    pub source_id: i32,
    pub source_name: Option<String>,

    pub subject: Option<String>,
    pub message_id: Option<String>,
    pub thread_id: Option<String>,
    pub mail_id: Option<String>,

    pub pick_up_at: Option<String>,
    pub pick_up_latitude: Option<f64>,
    pub pick_up_longitude: Option<f64>,

    pub pick_up_state: Option<String>,
    pub pick_up_zip: Option<String>,

    pub deliver_to: Option<String>,
    pub deliver_to_latitude: Option<f64>,
    pub deliver_to_longitude: Option<f64>,
    pub deliver_zip: Option<String>,

    pub duration: Option<i64>,
    pub distance: Option<f64>,

    pub order_number: Option<String>,

    pub contact_name: Option<String>,
    pub contact_address: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_person: Option<String>,

    pub received_date: Option<DateTime<Utc>>,
    pub pick_up_date: Option<DateTime<Utc>>,
    pub delivery_date: Option<DateTime<Utc>>,

    pub pick_up_date_raw: Option<String>,
    pub delivery_date_raw: Option<String>,
    pub expire_date_raw: Option<String>,
    pub expire_date: Option<DateTime<Utc>>,

    pub notes: Option<String>,
    pub pays: Option<f64>,
    pub posted_amount: Option<String>,
    pub miles: Option<i32>,
    pub pieces: Option<i32>,
    pub stackable: Option<bool>,
    pub hazardous: Option<bool>,
    pub fast_load: Option<bool>,
    pub dock_level: Option<bool>,
    pub weight: Option<i32>,

    pub dims: Option<serde_json::Value>,

    pub suggested_truck: Option<String>,
    pub vehicle_type: Option<String>,

    pub owner_name: Option<String>,
    pub owner_email: Option<String>,
    pub owner_key: Option<i32>,

    pub pick_up_at_geo: Option<serde_json::Value>,
    pub deliver_to_geo: Option<serde_json::Value>,

    pub pick_up_at_state: Option<String>,
    pub deliver_to_state: Option<String>,

    pub market_pays: Option<f64>,
    pub raw: Option<String>,

    pub miles_out: Option<i32>,
    pub nearest_vehicles_count: Option<i32>,

    pub vehicle_team: Option<Vec<i32>>,

    pub count_day: Option<i32>,

    pub created_at: Option<DateTime<Utc>>,
    pub pick_up_location: Option<String>,
    pub deliver_to_location: Option<String>,
}
