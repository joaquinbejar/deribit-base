/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Time in force enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Order remains active until explicitly cancelled
    #[serde(rename = "good_til_cancelled")]
    GoodTilCancelled,
    /// Order expires at the end of the trading day
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    /// Order must be filled immediately and completely or cancelled
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    /// Order must be filled immediately, partial fills allowed, remaining cancelled
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    /// Returns the string representation of the time in force value
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
    /// Limit order - executes at specified price or better
    #[serde(rename = "limit")]
    Limit,
    /// Market order - executes immediately at best available price
    #[serde(rename = "market")]
    Market,
    /// Stop limit order - becomes limit order when stop price is reached
    #[serde(rename = "stop_limit")]
    StopLimit,
    /// Stop market order - becomes market order when stop price is reached
    #[serde(rename = "stop_market")]
    StopMarket,
    /// Take limit order - limit order to take profit
    #[serde(rename = "take_limit")]
    TakeLimit,
    /// Take market order - market order to take profit
    #[serde(rename = "take_market")]
    TakeMarket,
    /// Market limit order - market order with limit price protection
    #[serde(rename = "market_limit")]
    MarketLimit,
    /// Trailing stop order - stop order that trails the market price
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
    /// Returns the string representation of the order type
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
    /// Order amount
    pub amount: f64,
    /// Whether order was placed via API
    pub api: bool,
    /// Average execution price
    pub average_price: f64,
    /// Order creation timestamp
    pub creation_timestamp: u64,
    /// Order direction (buy/sell)
    pub direction: String,
    /// Amount that has been filled
    pub filled_amount: f64,
    /// Instrument name
    pub instrument_name: String,
    /// Whether this is a liquidation order
    pub is_liquidation: bool,
    /// Order label
    pub label: String,
    /// Last update timestamp
    pub last_update_timestamp: u64,
    /// Maximum amount to show in order book
    pub max_show: f64,
    /// Unique order identifier
    pub order_id: String,
    /// Current order state
    pub order_state: String,
    /// Type of order
    pub order_type: String,
    /// Original order type before any modifications
    pub original_order_type: Option<String>,
    /// Whether this is a post-only order
    pub post_only: bool,
    /// Order price
    pub price: f64,
    /// Current profit/loss on the order
    pub profit_loss: Option<f64>,
    /// Whether this order only reduces position
    pub reduce_only: bool,
    /// Whether this order has been replaced
    pub replaced: bool,
    /// Whether this order reduces risk
    pub risk_reducing: bool,
    /// Time in force specification
    pub time_in_force: String,
    /// Whether the order has been triggered
    pub triggered: Option<bool>,
    /// Trigger condition for the order
    pub trigger: Option<String>,
    /// USD value of the order
    pub usd: Option<f64>,
    /// Whether order was placed via web interface
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
