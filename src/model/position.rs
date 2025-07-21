/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Position information
#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    /// Trading symbol/instrument name
    pub symbol: String,
    /// Position quantity (positive for long, negative for short)
    pub quantity: f64,
    /// Average price of the position
    pub average_price: f64,
    /// Unrealized profit and loss
    pub unrealized_pnl: f64,
    /// Realized profit and loss
    pub realized_pnl: f64,
}

impl_json_debug_pretty!(Position);
impl_json_display!(Position);
