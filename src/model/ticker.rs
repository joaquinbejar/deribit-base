use serde::{Deserialize, Serialize};

/// Ticker stats sub-structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    pub volume: f64,
    pub price_change: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
}

/// Ticker data structure with corrected field types
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    pub instrument_name: String,
    pub last_price: Option<f64>,
    pub mark_price: f64,
    pub best_bid_price: Option<f64>,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_ask_amount: f64,
    pub volume: Option<f64>,
    pub volume_usd: Option<f64>,
    pub open_interest: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub price_change: Option<f64>,
    pub price_change_percentage: Option<f64>,
    pub bid_iv: Option<f64>,
    pub ask_iv: Option<f64>,
    pub mark_iv: Option<f64>,
    pub timestamp: u64,
    pub state: String,
    pub settlement_price: Option<f64>,
    pub stats: TickerStats,
}

crate::impl_json_display!(TickerData);
crate::impl_json_debug_pretty!(TickerData);

crate::impl_json_display!(TickerStats);
crate::impl_json_debug_pretty!(TickerStats);
