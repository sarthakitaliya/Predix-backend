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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Market{
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: MarketStatus,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}