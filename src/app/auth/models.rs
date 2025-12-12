use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthUser {
    pub id: i32,
}
