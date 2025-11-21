use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "share_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ShareType {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    PartiallyFilled,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CloseOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub  market_id: Uuid,
    #[sqlx(rename = "type")]
    pub share_type: ShareType,
    pub price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub status: OrderStatus,
    pub closed_at: DateTime<Utc>,
}