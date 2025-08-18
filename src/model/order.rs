/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};
use crate::model::trade::TradeExecution;

/// Time in force enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "good_til_cancelled")]
    GoodTilCancelled,
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

/// Order side enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order type enum
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "stop_market")]
    StopMarket,
    #[serde(rename = "take_limit")]
    TakeLimit,
    #[serde(rename = "take_market")]
    TakeMarket,
    #[serde(rename = "market_limit")]
    MarketLimit,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::StopLimit => "stop_limit",
            OrderType::StopMarket => "stop_market",
            OrderType::TakeLimit => "take_limit",
            OrderType::TakeMarket => "take_market",
            OrderType::MarketLimit => "market_limit",
            OrderType::TrailingStop => "trailing_stop",
        }
    }
}

/// New order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct NewOrderRequest {
    /// Trading symbol/instrument name
    pub symbol: String,
    /// Order side (buy or sell)
    pub side: OrderSide,
    /// Type of order
    pub order_type: OrderType,
    /// Order quantity
    pub quantity: f64,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// Time in force specification
    pub time_in_force: TimeInForce,
    /// Client-specified order identifier
    pub client_order_id: Option<String>,
}

/// Order status enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order has been accepted by the system
    New,
    /// Order has been partially filled
    PartiallyFilled,
    /// Order has been completely filled
    Filled,
    /// Order is done for the day
    DoneForDay,
    /// Order has been cancelled
    Canceled,
    /// Order has been replaced
    Replaced,
    /// Order cancellation is pending
    PendingCancel,
    /// Order has been stopped
    Stopped,
    /// Order has been rejected
    Rejected,
    /// Order has been suspended
    Suspended,
    /// Order is pending acceptance
    PendingNew,
    /// Order has been calculated
    Calculated,
    /// Order has expired
    Expired,
    /// Order has been accepted for bidding
    AcceptedForBidding,
    /// Order replacement is pending
    PendingReplace,
}



/// Order information
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    pub amount: f64,
    pub api: bool,
    pub average_price: f64,
    pub creation_timestamp: u64,
    pub direction: String,
    pub filled_amount: f64,
    pub instrument_name: String,
    pub is_liquidation: bool,
    pub label: String,
    pub last_update_timestamp: u64,
    pub max_show: f64,
    pub order_id: String,
    pub order_state: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub post_only: bool,
    pub price: f64,
    pub profit_loss: Option<f64>,
    pub reduce_only: bool,
    pub replaced: bool,
    pub risk_reducing: bool,
    pub time_in_force: String,
    pub triggered: Option<bool>,
    pub trigger: Option<String>,
    pub usd: Option<f64>,
    pub web: bool,
}


impl_json_debug_pretty!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus,
    OrderInfo
);
impl_json_display!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus,
    OrderInfo
);
