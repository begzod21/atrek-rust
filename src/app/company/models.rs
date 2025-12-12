use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Clone, Deserialize, FromRow)]
pub struct TenantCompany {
    pub id: i32,
    pub schema_name: String,
    pub domain_url: String,
}
