/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::order::{OrderSide, OrderType, TimeInForce};
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Buy order request
#[derive(Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub price: Option<f64>,
    pub label: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<bool>,
    pub type_: Option<OrderType>,
}

impl_json_display!(BuyOrderRequest);
impl_json_debug_pretty!(BuyOrderRequest);

/// Sell order request
#[derive(Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub price: Option<f64>,
    pub label: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<bool>,
    pub type_: Option<OrderType>,
}

impl_json_display!(SellOrderRequest);
impl_json_debug_pretty!(SellOrderRequest);

/// Edit order request
#[derive(Clone, Serialize, Deserialize)]
pub struct EditOrderRequest {
    pub order_id: String,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub time_in_force: Option<TimeInForce>,
}

impl_json_display!(EditOrderRequest);
impl_json_debug_pretty!(EditOrderRequest);

/// Mass quote request item
#[derive(Clone, Serialize, Deserialize)]
pub struct MassQuoteItem {
    pub instrument_name: String,
    pub side: OrderSide,
    pub amount: f64,
    pub price: f64,
}

/// Mass quote request
#[derive(Clone, Serialize, Deserialize)]
pub struct MassQuoteRequest {
    pub items: Vec<MassQuoteItem>,
    pub label: Option<String>,
}

impl_json_display!(MassQuoteRequest);
impl_json_debug_pretty!(MassQuoteRequest);

/// Transfer result for order-related transfers (e.g., fee rebates)
#[derive(Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub id: String,
    pub status: String,
}

impl_json_display!(TransferResult);
impl_json_debug_pretty!(TransferResult);


/// Quote result
#[derive(Clone, Serialize, Deserialize)]
pub struct QuoteResult {
    pub instrument_name: String,
    pub success: bool,
    pub error: Option<String>,
}

impl_json_display!(QuoteResult);
impl_json_debug_pretty!(QuoteResult);

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
