/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::order::{OrderSide, OrderType, TimeInForce};
use pretty_simple_display::{DebugPretty, DisplaySimple};

use serde::{Deserialize, Serialize};

/// Buy order request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    /// Name of the instrument to trade
    pub instrument_name: String,
    /// Amount/quantity to buy
    pub amount: f64,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label for the order
    pub label: Option<String>,
    /// Time in force specification
    pub time_in_force: Option<TimeInForce>,
    /// Whether this order only reduces position
    pub reduce_only: Option<bool>,
    /// Whether this is a post-only order
    pub post_only: Option<bool>,
    /// Type of order to place
    pub type_: Option<OrderType>,
}

/// Sell order request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    /// Name of the instrument to trade
    pub instrument_name: String,
    /// Amount/quantity to sell
    pub amount: f64,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label for the order
    pub label: Option<String>,
    /// Time in force specification
    pub time_in_force: Option<TimeInForce>,
    /// Whether this order only reduces position
    pub reduce_only: Option<bool>,
    /// Whether this is a post-only order
    pub post_only: Option<bool>,
    /// Type of order to place
    pub type_: Option<OrderType>,
}

/// Edit order request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct EditOrderRequest {
    /// Unique identifier of the order to edit
    pub order_id: String,
    /// New order amount/quantity
    pub amount: Option<f64>,
    /// New order price
    pub price: Option<f64>,
    /// Whether this is a post-only order
    pub post_only: Option<bool>,
    /// Whether this order only reduces position
    pub reduce_only: Option<bool>,
    /// Time in force specification
    pub time_in_force: Option<TimeInForce>,
}

/// Mass quote request item
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteItem {
    /// Name of the instrument to quote
    pub instrument_name: String,
    /// Order side (buy or sell)
    pub side: OrderSide,
    /// Quote amount/quantity
    pub amount: f64,
    /// Quote price
    pub price: f64,
}

/// Mass quote request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteRequest {
    /// List of quote items
    pub items: Vec<MassQuoteItem>,
    /// User-defined label for the mass quote
    pub label: Option<String>,
}

/// Transfer result for order-related transfers (e.g., fee rebates)
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transfer identifier
    pub id: String,
    /// Transfer status
    pub status: String,
}

/// Quote result
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct QuoteResult {
    /// Name of the instrument that was quoted
    pub instrument_name: String,
    /// Whether the quote was successful
    pub success: bool,
    /// Error message if quote failed
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_quote_request() {
        let item = MassQuoteItem {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            amount: 10.0,
            price: 50000.0,
        };
        let req = MassQuoteRequest {
            items: vec![item],
            label: Some("test".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        let de: MassQuoteRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(de.items.len(), 1);
    }
}
