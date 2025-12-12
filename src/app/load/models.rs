use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Load {
    pub id: i32,
    pub created_at: Option<DateTime<Utc>>,
}
