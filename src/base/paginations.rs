use serde::Serialize;
use sqlx::{Transaction, Postgres, FromRow};
use axum::http::{HeaderMap, Uri};
use crate::helper::build_url::build_absolute_url;

#[derive(Serialize, Debug)]
pub struct PaginatedResponse<T> {
    pub count: i64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn paginate_query_with_tx<T>(
    tx: &mut Transaction<'_, Postgres>,
    params: PaginationParams,
    base_uri: &Uri,
    sql_count: &str,
    sql_data: &str,
    headers: &HeaderMap,
) -> Result<PaginatedResponse<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(50) as i64;

    if page == 0 {
        return Err(sqlx::Error::Protocol("Page number must be greater than 0".into()));
    }

    let offset = (page as i64 - 1) * page_size;

    // Count query transaction ichida
    let count: i64 = sqlx::query_scalar(sql_count)
        .fetch_one(&mut **tx)
        .await
        .unwrap_or(0);

    // Data query transaction ichida
    let results = sqlx::query_as::<_, T>(sql_data)
        .bind(page_size)
        .fetch_all(&mut **tx)
        .await?;

    let next = if offset + page_size < count {
        Some(format!(
            "{}{}?page={}",
            build_absolute_url(headers),
            base_uri,
            page + 1
        ))
    } else {
        None
    };

    let previous = if page > 1 {
        Some(format!(
            "{}{}?page={}",
            build_absolute_url(headers),
            base_uri,
            page - 1
        ))
    } else {
        None
    };

    Ok(PaginatedResponse {
        count,
        next,
        previous,
        results,
    })
}
