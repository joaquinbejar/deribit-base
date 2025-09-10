/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

use crate::prelude::WithdrawalPriority;

/// Currency structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency symbol (BTC, ETH, etc.)
    pub currency: String,
    /// Long currency name
    pub currency_long: String,
    /// Withdrawal fee
    pub fee_precision: u32,
    /// Minimum withdrawal amount
    pub min_confirmations: u32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Withdrawal precision
    pub withdrawal_fee: f64,
    /// Withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// APR for yield-generating tokens
    pub apr: Option<f64>,
}
