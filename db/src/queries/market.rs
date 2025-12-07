use chrono::{Date, DateTime, NaiveDateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    models::market::{self, Market, MarketOutcome, MarketStatus},
    utils::fetch_metadata::fetch_market_metadata,
};

pub async fn create_market(
    pool: &PgPool,
    market_id: &str,
    market_pda: &str,
    metadata_url: &str,
    yes_mint: &str,
    no_mint: &str,
    usdc_vault: &str,
    status: MarketStatus,
    outcome: MarketOutcome,
    close_time: DateTime<Utc>,
    updated_at: DateTime<Utc>,
) -> Result<Market, Error> {
    let metadata = fetch_market_metadata(metadata_url).await.map_err(|e| {
        sqlx::Error::Protocol(format!("Failed to fetch market metadata: {}", e).into())
    })?;
    let rec = sqlx::query_as::<_, Market>(
        r#"INSERT INTO markets (market_id, market_pda, metadata_url, yes_mint, no_mint, usdc_vault, status, outcome, close_time, title, description, category, image_url, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) 
        RETURNING id, market_id, market_pda, metadata_url, yes_mint, no_mint, usdc_vault, status, outcome, close_time, resolve_time, title, description, category, image_url, created_at, updated_at"#,
    )
    .bind(market_id)
    .bind(market_pda)
    .bind(metadata_url)
    .bind(yes_mint)
    .bind(no_mint)
    .bind(usdc_vault)
    .bind(status)
    .bind(outcome)
    .bind(close_time)
    .bind(metadata.title)
    .bind(metadata.description)
    .bind(metadata.category)
    .bind(metadata.image_url)
    .bind(updated_at)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_market_by_id(pool: &PgPool, market_id: String) -> Result<Market, Error> {
    let rec = sqlx::query_as::<_, Market>(
        r#"SELECT * FROM markets WHERE market_id = $1"#,
    )
    .bind(market_id)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn list_markets_by_status(pool: &PgPool, status: MarketStatus) -> Result<Vec<Market>, Error> {
    let recs = sqlx::query_as::<_, Market>(
        r#"SELECT * FROM markets WHERE status = $1"#,
    )
    .bind(status)
    .fetch_all(pool)
    .await?;

    Ok(recs)
}

pub async fn list_all_markets(pool: &PgPool) -> Result<Vec<Market>, Error> {
    let recs = sqlx::query_as::<_, Market>(
        r#"SELECT * FROM markets"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(recs)
}

pub async fn update_market_resolution(
    pool: &PgPool,
    market_id: String,
    status: MarketStatus,
    outcome: MarketOutcome,
    resolve_time: DateTime<Utc>,
) -> Result<(), Error> {
    let rec = sqlx::query(
        r#"UPDATE markets SET status = $1, outcome = $2, resolve_time = $3 WHERE market_id = $4"#,
    )
    .bind(status)
    .bind(outcome)
    .bind(resolve_time)
    .bind(market_id)
    .execute(pool)
    .await?;    

    Ok(())

}
