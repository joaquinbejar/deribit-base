/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::order::{OrderSide, OrderType, TimeInForce};
use serde::{Deserialize, Serialize};

/// FIX protocol compatible structures
pub mod fix {
    use super::*;

    /// New order request structure for FIX protocol
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewOrderRequest {
        /// Instrument symbol (e.g., "BTC-PERPETUAL")
        pub symbol: String,
        /// Order side (buy/sell)
        pub side: OrderSide,
        /// Order type
        pub order_type: OrderType,
        /// Order quantity
        pub quantity: f64,
        /// Order price (required for limit orders)
        pub price: Option<f64>,
        /// Time in force
        pub time_in_force: TimeInForce,
        /// Client order ID
        pub client_order_id: Option<String>,
    }

    impl NewOrderRequest {
        /// Create a new market buy order
        pub fn market_buy(symbol: String, quantity: f64) -> Self {
            Self {
                symbol,
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                quantity,
                price: None,
                time_in_force: TimeInForce::ImmediateOrCancel,
                client_order_id: None,
            }
        }

        /// Create a new market sell order
        pub fn market_sell(symbol: String, quantity: f64) -> Self {
            Self {
                symbol,
                side: OrderSide::Sell,
                order_type: OrderType::Market,
                quantity,
                price: None,
                time_in_force: TimeInForce::ImmediateOrCancel,
                client_order_id: None,
            }
        }

        /// Create a new limit buy order
        pub fn limit_buy(symbol: String, quantity: f64, price: f64) -> Self {
            Self {
                symbol,
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                quantity,
                price: Some(price),
                time_in_force: TimeInForce::GoodTillCancel,
                client_order_id: None,
            }
        }

        /// Create a new limit sell order
        pub fn limit_sell(symbol: String, quantity: f64, price: f64) -> Self {
            Self {
                symbol,
                side: OrderSide::Sell,
                order_type: OrderType::Limit,
                quantity,
                price: Some(price),
                time_in_force: TimeInForce::GoodTillCancel,
                client_order_id: None,
            }
        }

        /// Set client order ID
        pub fn with_client_order_id(mut self, client_order_id: String) -> Self {
            self.client_order_id = Some(client_order_id);
            self
        }

        /// Set time in force
        pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
            self.time_in_force = tif;
            self
        }
    }

    /// Convert from REST/WebSocket NewOrderRequest to FIX NewOrderRequest
    impl From<super::NewOrderRequest> for NewOrderRequest {
        fn from(rest_order: super::NewOrderRequest) -> Self {
            Self {
                symbol: rest_order.instrument_name,
                side: rest_order.side,
                order_type: rest_order.order_type,
                quantity: rest_order.amount,
                price: rest_order.price,
                time_in_force: rest_order.time_in_force,
                client_order_id: rest_order.client_order_id,
            }
        }
    }

    /// Convert from FIX NewOrderRequest to REST/WebSocket NewOrderRequest
    impl From<NewOrderRequest> for super::NewOrderRequest {
        fn from(fix_order: NewOrderRequest) -> Self {
            Self {
                instrument_name: fix_order.symbol,
                amount: fix_order.quantity,
                order_type: fix_order.order_type,
                side: fix_order.side,
                price: fix_order.price,
                time_in_force: fix_order.time_in_force,
                post_only: None,
                reduce_only: None,
                label: None,
                stop_price: None,
                trigger: None,
                advanced: None,
                max_show: None,
                reject_post_only: None,
                valid_until: None,
                client_order_id: fix_order.client_order_id,
            }
        }
    }
}

/// Generic request for creating new orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order amount
    pub amount: f64,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: Option<bool>,
    /// Reduce-only flag
    pub reduce_only: Option<bool>,
    /// Order label
    pub label: Option<String>,
    /// Stop price for stop orders
    pub stop_price: Option<f64>,
    /// Trigger type for stop orders
    pub trigger: Option<TriggerType>,
    /// Advanced order type
    pub advanced: Option<AdvancedOrderType>,
    /// Maximum show amount (iceberg orders)
    pub max_show: Option<f64>,
    /// Reject post-only flag
    pub reject_post_only: Option<bool>,
    /// Valid until timestamp
    pub valid_until: Option<i64>,
    /// Client order ID for tracking
    pub client_order_id: Option<String>,
}

impl NewOrderRequest {
    /// Create a new market buy order
    pub fn market_buy(instrument_name: String, amount: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            price: None,
            time_in_force: TimeInForce::ImmediateOrCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new market sell order
    pub fn market_sell(instrument_name: String, amount: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            price: None,
            time_in_force: TimeInForce::ImmediateOrCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new limit buy order
    pub fn limit_buy(instrument_name: String, amount: f64, price: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            price: Some(price),
            time_in_force: TimeInForce::GoodTillCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new limit sell order
    pub fn limit_sell(instrument_name: String, amount: f64, price: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Limit,
            side: OrderSide::Sell,
            price: Some(price),
            time_in_force: TimeInForce::GoodTillCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Set the order as post-only
    pub fn with_post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Set the order as reduce-only
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set order label
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }
}

/// Trigger type for stop orders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    /// Index price trigger
    IndexPrice,
    /// Mark price trigger
    MarkPrice,
    /// Last price trigger
    LastPrice,
}

/// Advanced order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvancedOrderType {
    /// USD denomination
    Usd,
    /// Implied volatility
    Implv,
}

/// Order modification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyOrderRequest {
    /// Order ID to modify
    pub order_id: String,
    /// New amount
    pub amount: Option<f64>,
    /// New price
    pub price: Option<f64>,
    /// New stop price
    pub stop_price: Option<f64>,
    /// New post-only flag
    pub post_only: Option<bool>,
    /// New reduce-only flag
    pub reduce_only: Option<bool>,
    /// New reject post-only flag
    pub reject_post_only: Option<bool>,
    /// New advanced order type
    pub advanced: Option<AdvancedOrderType>,
}

/// Order cancellation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    /// Order ID to cancel
    pub order_id: String,
}

/// Cancel all orders request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    /// Currency filter
    pub currency: Option<String>,
    /// Instrument kind filter
    pub kind: Option<String>,
    /// Instrument type filter
    #[serde(rename = "type")]
    pub instrument_type: Option<String>,
}

/// Position close request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosePositionRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order type for closing
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Price for limit orders
    pub price: Option<f64>,
}

/// Authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Grant type
    pub grant_type: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Refresh token (for refresh grant)
    pub refresh_token: Option<String>,
    /// Scope
    pub scope: Option<String>,
}

impl AuthRequest {
    /// Create a client credentials authentication request
    pub fn client_credentials(client_id: String, client_secret: String) -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            client_id,
            client_secret,
            refresh_token: None,
            scope: None,
        }
    }

    /// Create a refresh token authentication request
    pub fn refresh_token(client_id: String, client_secret: String, refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            client_id,
            client_secret,
            refresh_token: Some(refresh_token),
            scope: None,
        }
    }
}
