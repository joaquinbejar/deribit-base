/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Time in force enumeration
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order type enum
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
    /// Maximum amount to show in order book (optional)
    pub max_show: Option<f64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_in_force_as_str() {
        assert_eq!(TimeInForce::GoodTilCancelled.as_str(), "good_til_cancelled");
        assert_eq!(TimeInForce::GoodTilDay.as_str(), "good_til_day");
        assert_eq!(TimeInForce::FillOrKill.as_str(), "fill_or_kill");
        assert_eq!(
            TimeInForce::ImmediateOrCancel.as_str(),
            "immediate_or_cancel"
        );
    }

    #[test]
    fn test_time_in_force_serialization() {
        let tif = TimeInForce::GoodTilCancelled;
        let json = serde_json::to_string(&tif).unwrap();
        assert_eq!(json, "\"good_til_cancelled\"");

        let deserialized: TimeInForce = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_order_side_serialization() {
        let buy_side = OrderSide::Buy;
        let sell_side = OrderSide::Sell;

        let buy_json = serde_json::to_string(&buy_side).unwrap();
        let sell_json = serde_json::to_string(&sell_side).unwrap();

        assert_eq!(buy_json, "\"Buy\"");
        assert_eq!(sell_json, "\"Sell\"");

        let buy_deserialized: OrderSide = serde_json::from_str(&buy_json).unwrap();
        let sell_deserialized: OrderSide = serde_json::from_str(&sell_json).unwrap();

        assert_eq!(buy_deserialized, OrderSide::Buy);
        assert_eq!(sell_deserialized, OrderSide::Sell);
    }

    #[test]
    fn test_order_type_as_str() {
        assert_eq!(OrderType::Limit.as_str(), "limit");
        assert_eq!(OrderType::Market.as_str(), "market");
        assert_eq!(OrderType::StopLimit.as_str(), "stop_limit");
        assert_eq!(OrderType::StopMarket.as_str(), "stop_market");
        assert_eq!(OrderType::TakeLimit.as_str(), "take_limit");
        assert_eq!(OrderType::TakeMarket.as_str(), "take_market");
        assert_eq!(OrderType::MarketLimit.as_str(), "market_limit");
        assert_eq!(OrderType::TrailingStop.as_str(), "trailing_stop");
    }

    #[test]
    fn test_order_type_serialization() {
        let order_type = OrderType::Limit;
        let json = serde_json::to_string(&order_type).unwrap();
        assert_eq!(json, "\"limit\"");

        let deserialized: OrderType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, OrderType::Limit);
    }

    #[test]
    fn test_new_order_request_creation() {
        let order = NewOrderRequest {
            symbol: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity: 1.0,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            client_order_id: Some("CLIENT_ORDER_123".to_string()),
        };

        assert_eq!(order.symbol, "BTC-PERPETUAL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 1.0);
        assert_eq!(order.price, Some(50000.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
        assert_eq!(order.client_order_id, Some("CLIENT_ORDER_123".to_string()));
    }

    #[test]
    fn test_new_order_request_market_order() {
        let market_order = NewOrderRequest {
            symbol: "ETH-PERPETUAL".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            quantity: 2.0,
            price: None, // Market orders don't have price
            time_in_force: TimeInForce::ImmediateOrCancel,
            client_order_id: None,
        };

        assert_eq!(market_order.order_type, OrderType::Market);
        assert_eq!(market_order.price, None);
        assert_eq!(market_order.client_order_id, None);
    }

    #[test]
    fn test_order_status_variants() {
        let statuses = vec![
            OrderStatus::New,
            OrderStatus::PartiallyFilled,
            OrderStatus::Filled,
            OrderStatus::DoneForDay,
            OrderStatus::Canceled,
            OrderStatus::Replaced,
            OrderStatus::PendingCancel,
            OrderStatus::Stopped,
            OrderStatus::Rejected,
            OrderStatus::Suspended,
            OrderStatus::PendingNew,
            OrderStatus::Calculated,
            OrderStatus::Expired,
            OrderStatus::AcceptedForBidding,
            OrderStatus::PendingReplace,
        ];

        // Test that all variants can be created and compared
        for status in statuses {
            let cloned = status;
            assert_eq!(status, cloned);
        }
    }

    #[test]
    fn test_order_info_creation() {
        let order_info = OrderInfo {
            amount: 1.0,
            api: true,
            average_price: 50000.0,
            creation_timestamp: 1640995200000,
            direction: "buy".to_string(),
            filled_amount: 0.5,
            instrument_name: "BTC-PERPETUAL".to_string(),
            is_liquidation: false,
            label: "test_order".to_string(),
            last_update_timestamp: 1640995300000,
            max_show: Some(0.8),
            order_id: "ORDER_123".to_string(),
            order_state: "open".to_string(),
            order_type: "limit".to_string(),
            original_order_type: None,
            post_only: false,
            price: 50000.0,
            profit_loss: Some(100.0),
            reduce_only: false,
            replaced: false,
            risk_reducing: false,
            time_in_force: "good_til_cancelled".to_string(),
            triggered: Some(false),
            trigger: None,
            usd: Some(50000.0),
            web: false,
        };

        assert_eq!(order_info.amount, 1.0);
        assert_eq!(order_info.instrument_name, "BTC-PERPETUAL");
        assert_eq!(order_info.filled_amount, 0.5);
        assert_eq!(order_info.max_show, Some(0.8));
        assert_eq!(order_info.profit_loss, Some(100.0));
        assert!(order_info.api);
        assert!(!order_info.is_liquidation);
    }

    #[test]
    fn test_order_info_optional_fields() {
        let minimal_order_info = OrderInfo {
            amount: 1.0,
            api: true,
            average_price: 0.0,
            creation_timestamp: 1640995200000,
            direction: "buy".to_string(),
            filled_amount: 0.0,
            instrument_name: "BTC-PERPETUAL".to_string(),
            is_liquidation: false,
            label: "".to_string(),
            last_update_timestamp: 1640995200000,
            max_show: None,
            order_id: "ORDER_123".to_string(),
            order_state: "new".to_string(),
            order_type: "limit".to_string(),
            original_order_type: None,
            post_only: false,
            price: 50000.0,
            profit_loss: None,
            reduce_only: false,
            replaced: false,
            risk_reducing: false,
            time_in_force: "good_til_cancelled".to_string(),
            triggered: None,
            trigger: None,
            usd: None,
            web: false,
        };

        assert_eq!(minimal_order_info.max_show, None);
        assert_eq!(minimal_order_info.profit_loss, None);
        assert_eq!(minimal_order_info.triggered, None);
        assert_eq!(minimal_order_info.trigger, None);
        assert_eq!(minimal_order_info.usd, None);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let order = NewOrderRequest {
            symbol: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity: 1.0,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            client_order_id: Some("CLIENT_ORDER_123".to_string()),
        };

        let json = serde_json::to_string(&order).unwrap();
        let deserialized: NewOrderRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(order.symbol, deserialized.symbol);
        assert_eq!(order.side, deserialized.side);
        assert_eq!(order.order_type, deserialized.order_type);
        assert_eq!(order.quantity, deserialized.quantity);
        assert_eq!(order.price, deserialized.price);
        assert_eq!(order.time_in_force, deserialized.time_in_force);
        assert_eq!(order.client_order_id, deserialized.client_order_id);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let order = NewOrderRequest {
            symbol: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity: 1.0,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            client_order_id: Some("CLIENT_ORDER_123".to_string()),
        };

        let debug_str = format!("{:?}", order);
        let display_str = format!("{}", order);

        assert!(debug_str.contains("BTC-PERPETUAL"));
        assert!(display_str.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_enum_equality_and_cloning() {
        let tif1 = TimeInForce::GoodTilCancelled;
        let tif2 = tif1;
        assert_eq!(tif1, tif2);

        let side1 = OrderSide::Buy;
        let side2 = side1;
        assert_eq!(side1, side2);

        let type1 = OrderType::Limit;
        let type2 = type1;
        assert_eq!(type1, type2);

        let status1 = OrderStatus::New;
        let status2 = status1;
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_order_type_variants_coverage() {
        // Test all order type variants
        let types = vec![
            OrderType::Limit,
            OrderType::Market,
            OrderType::StopLimit,
            OrderType::StopMarket,
            OrderType::TakeLimit,
            OrderType::TakeMarket,
            OrderType::MarketLimit,
            OrderType::TrailingStop,
        ];

        for order_type in types {
            // Test as_str method
            let str_repr = order_type.as_str();
            assert!(!str_repr.is_empty());

            // Test serialization
            let json = serde_json::to_string(&order_type).unwrap();
            let deserialized: OrderType = serde_json::from_str(&json).unwrap();
            assert_eq!(order_type, deserialized);
        }
    }

    #[test]
    fn test_time_in_force_variants_coverage() {
        let tifs = vec![
            TimeInForce::GoodTilCancelled,
            TimeInForce::GoodTilDay,
            TimeInForce::FillOrKill,
            TimeInForce::ImmediateOrCancel,
        ];

        for tif in tifs {
            // Test as_str method
            let str_repr = tif.as_str();
            assert!(!str_repr.is_empty());

            // Test serialization
            let json = serde_json::to_string(&tif).unwrap();
            let deserialized: TimeInForce = serde_json::from_str(&json).unwrap();
            assert_eq!(tif, deserialized);
        }
    }
}
