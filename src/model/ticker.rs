use serde::{Deserialize, Serialize};

/// Ticker stats sub-structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// Trading volume
    pub volume: f64,
    /// Price change from previous period
    pub price_change: Option<f64>,
    /// Highest price in the period
    pub high: Option<f64>,
    /// Lowest price in the period
    pub low: Option<f64>,
}

/// Ticker data structure with corrected field types
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Name of the instrument
    pub instrument_name: String,
    /// Last traded price
    pub last_price: Option<f64>,
    /// Current mark price
    pub mark_price: f64,
    /// Best bid price available
    pub best_bid_price: Option<f64>,
    /// Best ask price available
    pub best_ask_price: Option<f64>,
    /// Amount available at best bid price
    pub best_bid_amount: f64,
    /// Amount available at best ask price
    pub best_ask_amount: f64,
    /// Trading volume in base currency
    pub volume: Option<f64>,
    /// Trading volume in USD
    pub volume_usd: Option<f64>,
    /// Open interest for the instrument
    pub open_interest: Option<f64>,
    /// Highest price in 24h period
    pub high: Option<f64>,
    /// Lowest price in 24h period
    pub low: Option<f64>,
    /// Absolute price change in 24h
    pub price_change: Option<f64>,
    /// Percentage price change in 24h
    pub price_change_percentage: Option<f64>,
    /// Implied volatility at best bid
    pub bid_iv: Option<f64>,
    /// Implied volatility at best ask
    pub ask_iv: Option<f64>,
    /// Mark implied volatility
    pub mark_iv: Option<f64>,
    /// Timestamp of the ticker data
    pub timestamp: u64,
    /// Current state of the instrument
    pub state: String,
    /// Settlement price (for expired instruments)
    pub settlement_price: Option<f64>,
    /// Additional ticker statistics
    pub stats: TickerStats,
}

crate::impl_json_display!(TickerData);
crate::impl_json_debug_pretty!(TickerData);

crate::impl_json_display!(TickerStats);
crate::impl_json_debug_pretty!(TickerStats);
