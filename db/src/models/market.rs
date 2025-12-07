use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "market_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "market_outcome", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MarketOutcome {
    Yes,
    No,
    #[sqlx(rename = "not_decided")]
    NotDecided,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Market {
    pub id: Uuid,
    pub market_id: String,
    pub market_pda: String,
    pub metadata_url: String,
    pub yes_mint: String,
    pub no_mint: String,
    pub usdc_vault: String,
    pub status: MarketStatus,
    pub outcome: MarketOutcome,
    pub close_time: DateTime<Utc>,
    pub resolve_time: Option<DateTime<Utc>>,

    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub image_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
