/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Buy direction
    Buy,
    /// Sell direction
    Sell,
}

/// Position structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    pub average_price: f64,
    pub average_price_usd: Option<f64>,
    pub delta: Option<f64>,
    pub direction: Direction,
    pub estimated_liquidation_price: Option<f64>,
    pub floating_profit_loss: Option<f64>,
    pub floating_profit_loss_usd: Option<f64>,
    pub gamma: Option<f64>,
    pub index_price: Option<f64>,
    pub initial_margin: Option<f64>,
    pub instrument_name: String,
    pub interest_value: Option<f64>,
    pub kind: Option<String>,
    pub leverage: Option<i32>,
    pub maintenance_margin: Option<f64>,
    pub mark_price: Option<f64>,
    pub open_orders_margin: Option<f64>,
    pub realized_funding: Option<f64>,
    pub realized_profit_loss: Option<f64>,
    pub settlement_price: Option<f64>,
    pub size: f64,
    pub size_currency: Option<f64>,
    pub theta: Option<f64>,
    pub total_profit_loss: Option<f64>,
    pub vega: Option<f64>,
    pub unrealized_profit_loss: Option<f64>,
}

impl_json_debug_pretty!(Position);
impl_json_display!(Position);
