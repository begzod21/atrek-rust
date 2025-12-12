use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Load {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}
