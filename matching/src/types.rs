use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Clone, Serialize, Debug)]
pub struct OrderEntry {
    pub id: Uuid,
    pub user_address: String,
    pub market_id: u64,
    pub side: Side,
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub market_id: u64,
    pub buyer_address: String,
    pub seller_address: String,
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

#[derive(Clone, Serialize, Debug)]
pub struct OpenOrder {
    pub id: Uuid,          // Unique Order ID (UUID or On-chain ID)
    pub market_id: String,   // To verify context
    pub outcome: String,     // "Yes" or "No"
    pub side: Side,        // "Bid" (Buy) or "Ask" (Sell)
    pub price: Decimal,      // Limit Price (e.g., 0.55)
    pub quantity: Decimal,   // Quantity of shares
}