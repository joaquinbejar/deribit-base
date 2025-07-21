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
    /// Order is valid for the current trading day only
    Day,
    /// Order remains active until explicitly cancelled
    GoodTillCancel,
    /// Order must be executed immediately or cancelled
    ImmediateOrCancel,
    /// Order must be filled completely or cancelled
    FillOrKill,
}

/// Order side enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order type enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order - executed immediately at current market price
    Market,
    /// Limit order - executed only at specified price or better
    Limit,
    /// Stop order - becomes market order when stop price is reached
    Stop,
    /// Stop limit order - becomes limit order when stop price is reached
    StopLimit,
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

impl_json_debug_pretty!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus
);
impl_json_display!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus
);
