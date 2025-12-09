use matching::types::{Side, Trade};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenOrder {
    pub id: String,          // Unique Order ID (UUID or On-chain ID)
    // pub market_id: String,   // To verify context
    pub outcome: String,     // "Yes" or "No"
    pub side: String,        // "Bid" (Buy) or "Ask" (Sell)
    pub price: Decimal,      // Limit Price (e.g., 0.55)
    pub quantity: Decimal,   // Quantity of shares
    // pub original_amount: Decimal, // Total size requested
    // pub filled_amount: Decimal,   // How much has matched so far
    // pub remaining_amount: Decimal,// Convenience field (optional)
    // pub timestamp: i64,      // Creation time (for sorting)
}

pub struct OpenOrdersResponse {
    pub orders: Vec<OpenOrder>,
}