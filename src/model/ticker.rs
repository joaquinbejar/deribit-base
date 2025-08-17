use serde::{Deserialize, Serialize};

/// User trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Instrument name
    pub instrument_name: String,
    /// Last trade price
    pub last_price: f64,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Current funding rate
    pub current_funding: Option<f64>,
    /// Estimated delivery price
    pub estimated_delivery_price: Option<f64>,
    /// 8h funding rate
    pub funding_8h: Option<f64>,
    /// Index price
    pub index_price: Option<f64>,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Mark price
    pub mark_price: Option<f64>,
    /// Maximum price
    pub max_price: Option<f64>,
    /// Minimum price
    pub min_price: Option<f64>,
    /// Open interest
    pub open_interest: Option<f64>,
    /// Settlement price
    pub settlement_price: Option<f64>,
    /// Instrument state
    pub state: Option<String>,
    /// Statistics
    pub stats: Option<TickerStats>,
    /// Timestamp
    pub timestamp: u64,
}

/// Ticker statistics structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// Volume in base currency
    pub volume: f64,
    /// Price change percentage
    pub price_change: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
    /// 24h high price
    pub high: Option<f64>,
    /// Volume in USD
    pub volume_usd: Option<f64>,
    /// Notional volume
    pub volume_notional: Option<f64>,
}

crate::impl_json_display!(TickerData);
crate::impl_json_debug_pretty!(TickerData);

crate::impl_json_display!(TickerStats);
crate::impl_json_debug_pretty!(TickerStats);