use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::market::{Market, MarketStatus};

pub async fn create_market(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    description: &str,
    closed_at: Option<DateTime<Utc>>,
) -> Result<Market, Error> {
    let rec = sqlx::query_as::<_, Market>(
        r#"INSERT INTO markets (id, title, description, closed_at) VALUES ($1, $2, $3, $4) 
        RETURNING id, title, description, status, closed_at, created_at"#,
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .bind(closed_at)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_market_by_id(pool: &PgPool, id: Uuid) -> Result<Market, Error> {
    let rec = sqlx::query_as::<_, Market>(
        r#"SELECT id, title, description, status, closed_at, created_at FROM markets WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn list_open_markets(pool: &PgPool) -> Result<Vec<Market>, Error> {
    let recs = sqlx::query_as::<_, Market>(
        r#"SELECT id, title, description, status, closed_at, created_at FROM markets WHERE status = 'open'"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(recs)
}

pub async fn update_market_resolution(
    pool: &PgPool,
    id: Uuid,
    status: MarketStatus,
) -> Result<Market, Error> {
    let rec = sqlx::query_as::<_, Market>(
        r#"UPDATE markets SET status = $1 WHERE id = $2 
        RETURNING id, title, description, status, closed_at, created_at"#,
    )
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}
