//! Common data types for Deribit API

use serde::{Deserialize, Serialize};

/// Currency type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "BTC")]
    Bitcoin,
    #[serde(rename = "ETH")]
    Ethereum,
    #[serde(rename = "SOL")]
    Solana,
    #[serde(rename = "USDC")]
    UsdCoin,
    #[serde(rename = "USDT")]
    Tether,
}

/// Instrument kind
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentKind {
    Future,
    Option,
    Spot,
    FutureCombo,
    OptionCombo,
}

/// Order side
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TrailingStop,
}

/// Order state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderState {
    Open,
    Filled,
    Rejected,
    Cancelled,
    Untriggered,
    Triggered,
}

/// Time in force
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    GoodTillCancelled,
    GoodTillDay,
    FillOrKill,
    ImmediateOrCancel,
}

/// Instrument information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_name: String,
    pub kind: InstrumentKind,
    pub currency: Currency,
    pub is_active: bool,
    pub expiration_timestamp: Option<i64>,
    pub strike: Option<f64>,
    pub option_type: Option<String>,
    pub tick_size: f64,
    pub min_trade_amount: f64,
    pub contract_size: f64,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub instrument_name: String,
    pub direction: OrderSide,
    pub amount: f64,
    pub price: Option<f64>,
    pub order_type: OrderType,
    pub order_state: OrderState,
    pub time_in_force: TimeInForce,
    pub filled_amount: f64,
    pub average_price: Option<f64>,
    pub creation_timestamp: i64,
    pub last_update_timestamp: i64,
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub instrument_name: String,
    pub order_id: String,
    pub direction: OrderSide,
    pub amount: f64,
    pub price: f64,
    pub timestamp: i64,
    pub fee: f64,
    pub fee_currency: Currency,
}

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub instrument_name: String,
    pub size: f64,
    pub direction: OrderSide,
    pub average_price: f64,
    pub mark_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
}

/// Account summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub currency: Currency,
    pub balance: f64,
    pub equity: f64,
    pub available_funds: f64,
    pub margin_balance: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub total_pl: f64,
}

/// Market data tick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub instrument_name: String,
    pub timestamp: i64,
    pub best_bid_price: Option<f64>,
    pub best_bid_amount: Option<f64>,
    pub best_ask_price: Option<f64>,
    pub best_ask_amount: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub index_price: Option<f64>,
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<T>,
    pub error: Option<ApiError>,
}

/// API error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
