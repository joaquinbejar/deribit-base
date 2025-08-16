/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::{currency::Currency, instrument::InstrumentKind};
use serde::{Deserialize, Serialize};

use crate::{impl_json_debug_pretty, impl_json_display};
/// Ticker information
#[derive(Clone, Serialize, Deserialize)] 
pub struct Ticker {
    /// Instrument name
    pub instrument_name: String,
    /// Timestamp of the ticker data
    pub timestamp: i64,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Last trade price
    pub last_price: Option<f64>,
    /// Mark price
    pub mark_price: Option<f64>,
    /// Index price
    pub index_price: Option<f64>,
    /// Open interest
    pub open_interest: f64,
    /// 24h volume
    pub volume_24h: f64,
    /// 24h volume in USD
    pub volume_usd_24h: f64,
    /// 24h price change
    pub price_change_24h: f64,
    /// High price in 24h
    pub high_24h: Option<f64>,
    /// Low price in 24h
    pub low_24h: Option<f64>,
    /// Underlying price (for derivatives)
    pub underlying_price: Option<f64>,
    /// Underlying index
    pub underlying_index: Option<String>,
    /// Instrument kind
    pub instrument_kind: Option<InstrumentKind>,
    /// Current funding rate (for perpetuals)
    pub current_funding: Option<f64>,
    /// Funding 8h rate
    pub funding_8h: Option<f64>,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Greeks (for options)
    pub greeks: Option<Greeks>,
    /// Interest rate
    pub interest_rate: Option<f64>,
}

impl Ticker {
    /// Calculate bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask_price, self.best_bid_price) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Calculate mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask_price, self.best_bid_price) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Calculate spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid != 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Check if there's a valid bid-ask spread
    pub fn has_valid_spread(&self) -> bool {
        self.best_bid_price.is_some() && self.best_ask_price.is_some()
    }
}

/// Order book entry
#[derive(Clone, Serialize, Deserialize)] 
pub struct OrderBookEntry {
    /// Price level
    pub price: f64,
    /// Amount at this price level
    pub amount: f64,
}

impl OrderBookEntry {
    /// Create a new order book entry
    pub fn new(price: f64, amount: f64) -> Self {
        Self { price, amount }
    }

    /// Calculate notional value
    pub fn notional(&self) -> f64 {
        self.price * self.amount
    }
}

/// Order book data
#[derive(Clone, Serialize, Deserialize)] 
pub struct OrderBook {
    /// Instrument name
    pub instrument_name: String,
    /// Timestamp of the order book
    pub timestamp: i64,
    /// Bid levels (sorted by price descending)
    pub bids: Vec<OrderBookEntry>,
    /// Ask levels (sorted by price ascending)
    pub asks: Vec<OrderBookEntry>,
    /// Change ID for incremental updates
    pub change_id: u64,
    /// Previous change ID
    pub prev_change_id: Option<u64>,
}

impl OrderBook {
    /// Create a new empty order book
    pub fn new(instrument_name: String, timestamp: i64, change_id: u64) -> Self {
        Self {
            instrument_name,
            timestamp,
            bids: Vec::new(),
            asks: Vec::new(),
            change_id,
            prev_change_id: None,
        }
    }

    /// Get best bid price
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|entry| entry.price)
    }

    /// Get best ask price
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|entry| entry.price)
    }

    /// Get bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Calculate total bid volume
    pub fn total_bid_volume(&self) -> f64 {
        self.bids.iter().map(|entry| entry.amount).sum()
    }

    /// Calculate total ask volume
    pub fn total_ask_volume(&self) -> f64 {
        self.asks.iter().map(|entry| entry.amount).sum()
    }

    /// Get volume at specific price level
    pub fn volume_at_price(&self, price: f64, is_bid: bool) -> f64 {
        let levels = if is_bid { &self.bids } else { &self.asks };
        levels
            .iter()
            .find(|entry| (entry.price - price).abs() < f64::EPSILON)
            .map(|entry| entry.amount)
            .unwrap_or(0.0)
    }
}

/// Greeks for options
#[derive(Clone, Serialize, Deserialize)] 
pub struct Greeks {
    /// Delta - sensitivity to underlying price changes
    pub delta: f64,
    /// Gamma - rate of change of delta
    pub gamma: f64,
    /// Theta - time decay
    pub theta: f64,
    /// Vega - sensitivity to volatility changes
    pub vega: f64,
    /// Rho - sensitivity to interest rate changes
    pub rho: Option<f64>,
}

/// Market statistics
#[derive(Clone, Serialize, Deserialize)] 
pub struct MarketStats {
    /// Currency
    pub currency: Currency,
    /// Total volume in 24h
    pub volume_24h: f64,
    /// Volume change in 24h
    pub volume_change_24h: f64,
    /// Price change in 24h
    pub price_change_24h: f64,
    /// High price in 24h
    pub high_24h: f64,
    /// Low price in 24h
    pub low_24h: f64,
    /// Number of active instruments
    pub active_instruments: u32,
    /// Total open interest
    pub total_open_interest: f64,
}

/// Candlestick/OHLCV data
#[derive(Clone, Serialize, Deserialize)] 
pub struct Candle {
    /// Timestamp
    pub timestamp: i64,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume
    pub volume: f64,
    /// Number of trades
    pub trades: Option<u64>,
}

impl Candle {
    /// Check if this is a bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Check if this is a bearish candle
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// Calculate the body size
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Calculate the upper shadow
    pub fn upper_shadow(&self) -> f64 {
        self.high - self.close.max(self.open)
    }

    /// Calculate the lower shadow
    pub fn lower_shadow(&self) -> f64 {
        self.close.min(self.open) - self.low
    }

    /// Calculate the range (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
}

// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(
    Ticker,    OrderBookEntry,    OrderBook,    Greeks,    MarketStats,    Candle
);

// Display implementations using compact JSON formatting
impl_json_display!(
    Ticker,    OrderBookEntry,    OrderBook,    Greeks,    MarketStats,    Candle
);
