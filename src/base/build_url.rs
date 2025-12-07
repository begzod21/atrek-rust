use axum::http::{Uri, HeaderMap};

#[allow(dead_code)]
pub(crate) fn build_absolute_url(headers: &HeaderMap, uri: &Uri, page: u32) -> String {
    let host = headers
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");

    let schema = headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("http");

    format!(
        "{}://{}{}?page={}",
        schema,
        host,
        uri.path(),
        page
    )


}
