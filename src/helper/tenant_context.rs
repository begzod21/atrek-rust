use sqlx::{Transaction, Postgres};

pub async fn with_tenant_schema<'a>(
    tx: &mut Transaction<'a, Postgres>,
    schema: &str,
) -> Result<(), sqlx::Error> {
    let query = format!(r#"SET search_path TO "{}""#, schema);
    sqlx::query(&query).execute(&mut **tx).await?;
    Ok(())
}
