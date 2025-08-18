/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::{instrument::InstrumentKind, order::OrderSide};
use serde::{Deserialize, Serialize};

use crate::{impl_json_debug_pretty, impl_json_display};
/// Liquidity type enumeration
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Liquidity {
    /// Maker (provided liquidity)
    #[serde(rename = "M")]
    Maker,
    /// Taker (consumed liquidity)
    #[serde(rename = "T")]
    Taker,
    /// Mixed (both maker and taker in same trade)
    #[serde(rename = "MT")]
    Mixed,
}

/// Trade execution information
#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique trade identifier
    pub trade_id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Trade direction (buy/sell)
    pub direction: OrderSide,
    /// Trade amount
    pub amount: f64,
    /// Execution price
    pub price: f64,
    /// Trade timestamp
    pub timestamp: i64,
    /// Fee amount
    pub fee: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Liquidity type (maker/taker)
    pub liquidity: Liquidity,
    /// Mark price at time of trade
    pub mark_price: f64,
    /// Index price at time of trade
    pub index_price: f64,
    /// Instrument kind
    pub instrument_kind: Option<InstrumentKind>,
    /// Trade sequence number
    pub trade_seq: Option<u64>,
    /// User role in the trade
    pub user_role: Option<String>,
    /// Whether this is a block trade
    pub block_trade: Option<bool>,
    /// Underlying price (for options)
    pub underlying_price: Option<f64>,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Label associated with the order
    pub label: Option<String>,
    /// Profit and loss from this trade
    pub profit_loss: Option<f64>,
    /// Tick direction
    pub tick_direction: Option<i32>,
    /// Whether this trade was self-traded
    pub self_trade: Option<bool>,
}

impl Trade {
    /// Calculate the notional value of the trade
    pub fn notional_value(&self) -> f64 {
        self.amount * self.price
    }

    /// Check if this was a maker trade
    pub fn is_maker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Maker | Liquidity::Mixed)
    }

    /// Check if this was a taker trade
    pub fn is_taker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Taker | Liquidity::Mixed)
    }

    /// Check if this is a buy trade
    pub fn is_buy(&self) -> bool {
        self.direction == OrderSide::Buy
    }

    /// Check if this is a sell trade
    pub fn is_sell(&self) -> bool {
        self.direction == OrderSide::Sell
    }

    /// Get fee as percentage of notional
    pub fn fee_percentage(&self) -> f64 {
        if self.notional_value() != 0.0 {
            (self.fee / self.notional_value()) * 100.0
        } else {
            0.0
        }
    }
}

/// Trade statistics
#[derive(Clone, Serialize, Deserialize)]
pub struct TradeStats {
    /// Total number of trades
    pub count: u64,
    /// Total volume
    pub volume: f64,
    /// Total fees paid
    pub total_fees: f64,
    /// Average price
    pub avg_price: f64,
    /// Profit and loss
    pub pnl: f64,
    /// Number of winning trades
    pub winning_trades: u64,
    /// Number of losing trades
    pub losing_trades: u64,
}

impl TradeStats {
    /// Create empty trade statistics
    pub fn new() -> Self {
        Self {
            count: 0,
            volume: 0.0,
            total_fees: 0.0,
            avg_price: 0.0,
            pnl: 0.0,
            winning_trades: 0,
            losing_trades: 0,
        }
    }

    /// Calculate win rate as percentage
    pub fn win_rate(&self) -> f64 {
        if self.count > 0 {
            (self.winning_trades as f64 / self.count as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for TradeStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Trade execution
#[derive(Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub amount: f64,
    pub direction: String,
    pub fee: f64,
    pub fee_currency: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: String,
    pub liquidity: String,
    pub mark_price: f64,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub self_trade: bool,
    pub state: String,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
    pub underlying_price: Option<f64>,
}

/// User trade information
#[derive(Clone, Serialize, Deserialize)]
pub struct UserTrade {
    pub amount: f64,
    pub direction: String,
    pub fee: f64,
    pub fee_currency: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: String,
    pub liquidity: String,
    pub mark_price: f64,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub self_trade: bool,
    pub state: String,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
    pub underlying_price: Option<f64>,
}

/// Last trade
#[derive(Clone, Serialize, Deserialize)]
pub struct LastTrade {
    pub amount: f64,
    pub direction: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub liquid: Option<String>,
    pub price: f64,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
}

// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(
    LastTrade,
    Liquidity,
    Trade,
    TradeStats,
    TradeExecution,
    UserTrade
);

// Display implementations using compact JSON formatting
impl_json_display!(
    LastTrade,
    Liquidity,
    Trade,
    TradeStats,
    TradeExecution,
    UserTrade
);
