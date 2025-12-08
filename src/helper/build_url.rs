use axum::http::{HeaderMap};

#[allow(dead_code)]
pub(crate) fn build_absolute_url(headers: &HeaderMap) -> String {
    let host = headers
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");

    let schema = headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("http");

    format!("{}://{}", schema, host)
}
