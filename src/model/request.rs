/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::order::{OrderSide, OrderType, TimeInForce};
use pretty_simple_display::{DebugPretty, DisplaySimple};

use serde::{Deserialize, Serialize};

/// FIX protocol compatible structures
pub mod fix {
    use super::*;

    /// New order request structure for FIX protocol
    #[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
                time_in_force: TimeInForce::GoodTilCancelled,
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
                time_in_force: TimeInForce::GoodTilCancelled,
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
            time_in_force: TimeInForce::GoodTilCancelled,
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
            time_in_force: TimeInForce::GoodTilCancelled,
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
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    /// Order ID to cancel
    pub order_id: String,
}

/// Cancel all orders request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_new_order_request_market_buy() {
        let order = fix::NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0);
        assert_eq!(order.symbol, "BTC-PERPETUAL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.quantity, 1.0);
        assert_eq!(order.price, None);
        assert_eq!(order.time_in_force, TimeInForce::ImmediateOrCancel);
        assert_eq!(order.client_order_id, None);
    }

    #[test]
    fn test_fix_new_order_request_market_sell() {
        let order = fix::NewOrderRequest::market_sell("ETH-PERPETUAL".to_string(), 2.0);
        assert_eq!(order.symbol, "ETH-PERPETUAL");
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.quantity, 2.0);
        assert_eq!(order.price, None);
        assert_eq!(order.time_in_force, TimeInForce::ImmediateOrCancel);
    }

    #[test]
    fn test_fix_new_order_request_limit_buy() {
        let order = fix::NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0);
        assert_eq!(order.symbol, "BTC-PERPETUAL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 1.0);
        assert_eq!(order.price, Some(50000.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_fix_new_order_request_limit_sell() {
        let order = fix::NewOrderRequest::limit_sell("ETH-PERPETUAL".to_string(), 2.0, 3500.0);
        assert_eq!(order.symbol, "ETH-PERPETUAL");
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 2.0);
        assert_eq!(order.price, Some(3500.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_fix_new_order_request_with_client_order_id() {
        let order = fix::NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0)
            .with_client_order_id("CLIENT_ORDER_123".to_string());
        assert_eq!(order.client_order_id, Some("CLIENT_ORDER_123".to_string()));
    }

    #[test]
    fn test_fix_new_order_request_with_time_in_force() {
        let order = fix::NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_time_in_force(TimeInForce::FillOrKill);
        assert_eq!(order.time_in_force, TimeInForce::FillOrKill);
    }

    #[test]
    fn test_new_order_request_market_buy() {
        let order = NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0);
        assert_eq!(order.instrument_name, "BTC-PERPETUAL");
        assert_eq!(order.amount, 1.0);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.price, None);
        assert_eq!(order.time_in_force, TimeInForce::ImmediateOrCancel);
        assert_eq!(order.post_only, None);
        assert_eq!(order.reduce_only, None);
    }

    #[test]
    fn test_new_order_request_market_sell() {
        let order = NewOrderRequest::market_sell("ETH-PERPETUAL".to_string(), 2.0);
        assert_eq!(order.instrument_name, "ETH-PERPETUAL");
        assert_eq!(order.amount, 2.0);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.price, None);
        assert_eq!(order.time_in_force, TimeInForce::ImmediateOrCancel);
    }

    #[test]
    fn test_new_order_request_limit_buy() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0);
        assert_eq!(order.instrument_name, "BTC-PERPETUAL");
        assert_eq!(order.amount, 1.0);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.price, Some(50000.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_new_order_request_limit_sell() {
        let order = NewOrderRequest::limit_sell("ETH-PERPETUAL".to_string(), 2.0, 3500.0);
        assert_eq!(order.instrument_name, "ETH-PERPETUAL");
        assert_eq!(order.amount, 2.0);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.price, Some(3500.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_new_order_request_with_post_only() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_post_only(true);
        assert_eq!(order.post_only, Some(true));
    }

    #[test]
    fn test_new_order_request_with_reduce_only() {
        let order = NewOrderRequest::limit_sell("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_reduce_only(true);
        assert_eq!(order.reduce_only, Some(true));
    }

    #[test]
    fn test_new_order_request_with_label() {
        let order = NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0)
            .with_label("test_order".to_string());
        assert_eq!(order.label, Some("test_order".to_string()));
    }

    #[test]
    fn test_new_order_request_with_time_in_force() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_time_in_force(TimeInForce::FillOrKill);
        assert_eq!(order.time_in_force, TimeInForce::FillOrKill);
    }

    #[test]
    fn test_trigger_type_serialization() {
        let trigger_types = vec![
            TriggerType::IndexPrice,
            TriggerType::MarkPrice,
            TriggerType::LastPrice,
        ];

        for trigger_type in trigger_types {
            let json = serde_json::to_string(&trigger_type).unwrap();
            let deserialized: TriggerType = serde_json::from_str(&json).unwrap();
            assert_eq!(trigger_type, deserialized);
        }
    }

    #[test]
    fn test_advanced_order_type_serialization() {
        let advanced_types = vec![AdvancedOrderType::Usd, AdvancedOrderType::Implv];

        for advanced_type in advanced_types {
            let json = serde_json::to_string(&advanced_type).unwrap();
            let deserialized: AdvancedOrderType = serde_json::from_str(&json).unwrap();
            assert_eq!(advanced_type, deserialized);
        }
    }

    #[test]
    fn test_modify_order_request() {
        let modify_request = ModifyOrderRequest {
            order_id: "ORDER_123".to_string(),
            amount: Some(2.0),
            price: Some(51000.0),
            stop_price: None,
            post_only: Some(true),
            reduce_only: Some(false),
            reject_post_only: None,
            advanced: Some(AdvancedOrderType::Usd),
        };

        assert_eq!(modify_request.order_id, "ORDER_123");
        assert_eq!(modify_request.amount, Some(2.0));
        assert_eq!(modify_request.price, Some(51000.0));
        assert_eq!(modify_request.post_only, Some(true));
        assert_eq!(modify_request.advanced, Some(AdvancedOrderType::Usd));
    }

    #[test]
    fn test_cancel_order_request() {
        let cancel_request = CancelOrderRequest {
            order_id: "ORDER_123".to_string(),
        };
        assert_eq!(cancel_request.order_id, "ORDER_123");
    }

    #[test]
    fn test_cancel_all_orders_request() {
        let cancel_all_request = CancelAllOrdersRequest {
            currency: Some("BTC".to_string()),
            kind: Some("future".to_string()),
            instrument_type: Some("perpetual".to_string()),
        };

        assert_eq!(cancel_all_request.currency, Some("BTC".to_string()));
        assert_eq!(cancel_all_request.kind, Some("future".to_string()));
        assert_eq!(
            cancel_all_request.instrument_type,
            Some("perpetual".to_string())
        );
    }

    #[test]
    fn test_cancel_all_orders_request_empty() {
        let cancel_all_request = CancelAllOrdersRequest {
            currency: None,
            kind: None,
            instrument_type: None,
        };

        assert_eq!(cancel_all_request.currency, None);
        assert_eq!(cancel_all_request.kind, None);
        assert_eq!(cancel_all_request.instrument_type, None);
    }

    #[test]
    fn test_close_position_request() {
        let close_request = ClosePositionRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_type: OrderType::Market,
            price: None,
        };

        assert_eq!(close_request.instrument_name, "BTC-PERPETUAL");
        assert_eq!(close_request.order_type, OrderType::Market);
        assert_eq!(close_request.price, None);

        let close_limit_request = ClosePositionRequest {
            instrument_name: "ETH-PERPETUAL".to_string(),
            order_type: OrderType::Limit,
            price: Some(3500.0),
        };

        assert_eq!(close_limit_request.price, Some(3500.0));
    }

    #[test]
    fn test_auth_request_client_credentials() {
        let auth_request = AuthRequest::client_credentials(
            "client_id_123".to_string(),
            "client_secret_456".to_string(),
        );

        assert_eq!(auth_request.grant_type, "client_credentials");
        assert_eq!(auth_request.client_id, "client_id_123");
        assert_eq!(auth_request.client_secret, "client_secret_456");
        assert_eq!(auth_request.refresh_token, None);
        assert_eq!(auth_request.scope, None);
    }

    #[test]
    fn test_auth_request_refresh_token() {
        let auth_request = AuthRequest::refresh_token(
            "client_id_123".to_string(),
            "client_secret_456".to_string(),
            "refresh_token_789".to_string(),
        );

        assert_eq!(auth_request.grant_type, "refresh_token");
        assert_eq!(auth_request.client_id, "client_id_123");
        assert_eq!(auth_request.client_secret, "client_secret_456");
        assert_eq!(
            auth_request.refresh_token,
            Some("refresh_token_789".to_string())
        );
        assert_eq!(auth_request.scope, None);
    }

    #[test]
    fn test_fix_to_rest_conversion() {
        let fix_order = fix::NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_client_order_id("CLIENT_ORDER_123".to_string());

        let rest_order: NewOrderRequest = fix_order.into();

        assert_eq!(rest_order.instrument_name, "BTC-PERPETUAL");
        assert_eq!(rest_order.amount, 1.0);
        assert_eq!(rest_order.order_type, OrderType::Limit);
        assert_eq!(rest_order.side, OrderSide::Buy);
        assert_eq!(rest_order.price, Some(50000.0));
        assert_eq!(
            rest_order.client_order_id,
            Some("CLIENT_ORDER_123".to_string())
        );
        assert_eq!(rest_order.post_only, None);
        assert_eq!(rest_order.reduce_only, None);
    }

    #[test]
    fn test_rest_to_fix_conversion() {
        let rest_order = NewOrderRequest::limit_sell("ETH-PERPETUAL".to_string(), 2.0, 3500.0)
            .with_label("test_order".to_string())
            .with_post_only(true);

        let fix_order: fix::NewOrderRequest = rest_order.into();

        assert_eq!(fix_order.symbol, "ETH-PERPETUAL");
        assert_eq!(fix_order.quantity, 2.0);
        assert_eq!(fix_order.order_type, OrderType::Limit);
        assert_eq!(fix_order.side, OrderSide::Sell);
        assert_eq!(fix_order.price, Some(3500.0));
        assert_eq!(fix_order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_post_only(true)
            .with_label("test_order".to_string());

        let json = serde_json::to_string(&order).unwrap();
        let deserialized: NewOrderRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(order.instrument_name, deserialized.instrument_name);
        assert_eq!(order.amount, deserialized.amount);
        assert_eq!(order.order_type, deserialized.order_type);
        assert_eq!(order.side, deserialized.side);
        assert_eq!(order.price, deserialized.price);
        assert_eq!(order.post_only, deserialized.post_only);
        assert_eq!(order.label, deserialized.label);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let order = NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0);
        let debug_str = format!("{:?}", order);
        let display_str = format!("{}", order);

        assert!(debug_str.contains("BTC-PERPETUAL"));
        assert!(display_str.contains("BTC-PERPETUAL"));

        let auth_request =
            AuthRequest::client_credentials("client_id".to_string(), "client_secret".to_string());
        let auth_debug = format!("{:?}", auth_request);
        let auth_display = format!("{}", auth_request);

        assert!(auth_debug.contains("client_credentials"));
        assert!(auth_display.contains("client_credentials"));
    }

    #[test]
    fn test_enum_equality_and_cloning() {
        let trigger1 = TriggerType::IndexPrice;
        let trigger2 = trigger1.clone();
        assert_eq!(trigger1, trigger2);

        let advanced1 = AdvancedOrderType::Usd;
        let advanced2 = advanced1.clone();
        assert_eq!(advanced1, advanced2);
    }

    #[test]
    fn test_complex_order_with_all_fields() {
        let order = NewOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: 1.5,
            order_type: OrderType::StopLimit,
            side: OrderSide::Buy,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilDay,
            post_only: Some(true),
            reduce_only: Some(false),
            label: Some("complex_order".to_string()),
            stop_price: Some(49000.0),
            trigger: Some(TriggerType::MarkPrice),
            advanced: Some(AdvancedOrderType::Implv),
            max_show: Some(0.5),
            reject_post_only: Some(false),
            valid_until: Some(1640995200000),
            client_order_id: Some("CLIENT_ORDER_COMPLEX".to_string()),
        };

        assert_eq!(order.instrument_name, "BTC-PERPETUAL");
        assert_eq!(order.amount, 1.5);
        assert_eq!(order.order_type, OrderType::StopLimit);
        assert_eq!(order.stop_price, Some(49000.0));
        assert_eq!(order.trigger, Some(TriggerType::MarkPrice));
        assert_eq!(order.advanced, Some(AdvancedOrderType::Implv));
        assert_eq!(order.max_show, Some(0.5));
        assert_eq!(order.valid_until, Some(1640995200000));
    }
}
