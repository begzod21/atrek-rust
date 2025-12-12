use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub company_id: i32,
}
