use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Clone, Deserialize, FromRow)]
pub struct TenantCompany {
    pub id: i32,
    pub domain_url: String,
    pub schema_name: String,
}
