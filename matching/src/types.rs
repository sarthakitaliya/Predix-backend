use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Clone, Serialize, Debug)]
pub struct OrderEntry {
    pub id: Uuid,
    pub user_id: String,
    pub market_id: String,
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub market_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Serialize, Debug)]
pub struct SnapshotData {
    pub price: Decimal,
    pub quantity: Decimal,
    pub total: Decimal,
}
#[derive(Serialize)]
pub struct MarketSnapshot {
    pub yes: (Vec<SnapshotData>, Vec<SnapshotData>),
    pub no: (Vec<SnapshotData>, Vec<SnapshotData>),
}
