use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use serde::Deserialize;


use crate::app::auth::models::AuthUser;


#[derive(Deserialize, Debug)]
struct JWTClaims {
    user_id: i32,
    username: String,
    company_id: i32,
}

pub async fn auth_middleware(
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_str.trim_start_matches("Bearer ").to_string();

    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let payload_b64 = parts[1];

    let payload_json = STANDARD_NO_PAD
        .decode(payload_b64)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims: JWTClaims =
        serde_json::from_slice(&payload_json).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let auth_user = AuthUser {
        id: claims.user_id
    };

    req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)


}
