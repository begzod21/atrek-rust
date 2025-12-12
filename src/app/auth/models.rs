use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthUser {
    pub user_id: i64,
    pub username: String,
    pub company_id: i64,
}
