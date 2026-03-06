/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::instrument::InstrumentKind;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Ticker information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MarketStats {
    /// Currency
    pub currency: String,
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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

/// Mark price history data point
///
/// Represents a single mark price value at a specific timestamp,
/// returned by `/public/get_mark_price_history`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkPricePoint {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: i64,
    /// Mark price value
    pub mark_price: f64,
}

impl MarkPricePoint {
    /// Create a new mark price point
    #[must_use]
    pub fn new(timestamp: i64, mark_price: f64) -> Self {
        Self {
            timestamp,
            mark_price,
        }
    }

    /// Create from raw API response tuple [timestamp, mark_price]
    #[must_use]
    pub fn from_tuple(data: (i64, f64)) -> Self {
        Self {
            timestamp: data.0,
            mark_price: data.1,
        }
    }
}

/// Mark price history collection
///
/// Collection of mark price history points for an instrument,
/// returned by `/public/get_mark_price_history`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkPriceHistory {
    /// Instrument name
    pub instrument_name: String,
    /// Collection of mark price points
    pub points: Vec<MarkPricePoint>,
}

impl MarkPriceHistory {
    /// Create a new empty mark price history
    #[must_use]
    pub fn new(instrument_name: String) -> Self {
        Self {
            instrument_name,
            points: Vec::new(),
        }
    }

    /// Create from raw API response data
    #[must_use]
    pub fn from_raw(instrument_name: String, data: Vec<(i64, f64)>) -> Self {
        Self {
            instrument_name,
            points: data.into_iter().map(MarkPricePoint::from_tuple).collect(),
        }
    }

    /// Add a mark price point
    pub fn add_point(&mut self, point: MarkPricePoint) {
        self.points.push(point);
    }

    /// Get the latest mark price point
    #[must_use]
    pub fn latest(&self) -> Option<&MarkPricePoint> {
        self.points.iter().max_by_key(|p| p.timestamp)
    }

    /// Get the earliest mark price point
    #[must_use]
    pub fn earliest(&self) -> Option<&MarkPricePoint> {
        self.points.iter().min_by_key(|p| p.timestamp)
    }

    /// Get the number of points
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the history is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// Trade volume data for a currency
///
/// Aggregated 24h trade volumes for different instrument types,
/// returned by `/public/get_trade_volumes`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeVolume {
    /// Currency code (e.g., "BTC", "ETH")
    pub currency: String,
    /// 24h trade volume for put options
    pub puts_volume: f64,
    /// 24h trade volume for call options
    pub calls_volume: f64,
    /// 24h trade volume for futures
    pub futures_volume: f64,
    /// 24h trade volume for spot
    #[serde(default)]
    pub spot_volume: f64,
    /// 7-day trade volume for put options (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puts_volume_7d: Option<f64>,
    /// 30-day trade volume for put options (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puts_volume_30d: Option<f64>,
    /// 7-day trade volume for call options (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calls_volume_7d: Option<f64>,
    /// 30-day trade volume for call options (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calls_volume_30d: Option<f64>,
    /// 7-day trade volume for futures (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_volume_7d: Option<f64>,
    /// 30-day trade volume for futures (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_volume_30d: Option<f64>,
    /// 7-day trade volume for spot (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_volume_7d: Option<f64>,
    /// 30-day trade volume for spot (extended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_volume_30d: Option<f64>,
}

impl TradeVolume {
    /// Create a new trade volume with basic 24h data
    #[must_use]
    pub fn new(
        currency: String,
        puts_volume: f64,
        calls_volume: f64,
        futures_volume: f64,
        spot_volume: f64,
    ) -> Self {
        Self {
            currency,
            puts_volume,
            calls_volume,
            futures_volume,
            spot_volume,
            puts_volume_7d: None,
            puts_volume_30d: None,
            calls_volume_7d: None,
            calls_volume_30d: None,
            futures_volume_7d: None,
            futures_volume_30d: None,
            spot_volume_7d: None,
            spot_volume_30d: None,
        }
    }

    /// Calculate total options volume (puts + calls)
    #[must_use]
    pub fn total_options_volume(&self) -> f64 {
        self.puts_volume + self.calls_volume
    }

    /// Calculate total 24h volume across all instrument types
    #[must_use]
    pub fn total_volume(&self) -> f64 {
        self.puts_volume + self.calls_volume + self.futures_volume + self.spot_volume
    }

    /// Calculate put/call ratio
    #[must_use]
    pub fn put_call_ratio(&self) -> Option<f64> {
        if self.calls_volume > 0.0 {
            Some(self.puts_volume / self.calls_volume)
        } else {
            None
        }
    }
}

/// Volatility index OHLC candle
///
/// Represents a single volatility index candle with OHLC data,
/// returned by `/public/get_volatility_index_data`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityIndexCandle {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: i64,
    /// Open volatility value
    pub open: f64,
    /// High volatility value
    pub high: f64,
    /// Low volatility value
    pub low: f64,
    /// Close volatility value
    pub close: f64,
}

impl VolatilityIndexCandle {
    /// Create a new volatility index candle
    #[must_use]
    pub fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            timestamp,
            open,
            high,
            low,
            close,
        }
    }

    /// Create from raw API response tuple [timestamp, open, high, low, close]
    #[must_use]
    pub fn from_tuple(data: (i64, f64, f64, f64, f64)) -> Self {
        Self {
            timestamp: data.0,
            open: data.1,
            high: data.2,
            low: data.3,
            close: data.4,
        }
    }

    /// Calculate the range (high - low)
    #[must_use]
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Check if volatility increased (close > open)
    #[must_use]
    pub fn is_increasing(&self) -> bool {
        self.close > self.open
    }

    /// Check if volatility decreased (close < open)
    #[must_use]
    pub fn is_decreasing(&self) -> bool {
        self.close < self.open
    }
}

/// Volatility index data response
///
/// Collection of volatility index candles with optional continuation token,
/// returned by `/public/get_volatility_index_data`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityIndexData {
    /// Currency for this volatility index
    pub currency: String,
    /// Collection of volatility candles
    pub data: Vec<VolatilityIndexCandle>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

impl VolatilityIndexData {
    /// Create a new empty volatility index data
    #[must_use]
    pub fn new(currency: String) -> Self {
        Self {
            currency,
            data: Vec::new(),
            continuation: None,
        }
    }

    /// Create from raw API response data
    #[must_use]
    pub fn from_raw(
        currency: String,
        data: Vec<(i64, f64, f64, f64, f64)>,
        continuation: Option<String>,
    ) -> Self {
        Self {
            currency,
            data: data
                .into_iter()
                .map(VolatilityIndexCandle::from_tuple)
                .collect(),
            continuation,
        }
    }

    /// Get the latest candle
    #[must_use]
    pub fn latest(&self) -> Option<&VolatilityIndexCandle> {
        self.data.iter().max_by_key(|c| c.timestamp)
    }

    /// Get the earliest candle
    #[must_use]
    pub fn earliest(&self) -> Option<&VolatilityIndexCandle> {
        self.data.iter().min_by_key(|c| c.timestamp)
    }

    /// Check if there are more results available
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }

    /// Get the number of candles
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the data is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Index type filter for supported index names
///
/// Used to filter index names by type in `/public/get_supported_index_names`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    /// All index types
    #[default]
    All,
    /// Spot price indexes
    Spot,
    /// Derivative price indexes
    Derivative,
}

impl IndexType {
    /// Get the string representation for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Spot => "spot",
            Self::Derivative => "derivative",
        }
    }
}

impl std::fmt::Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ticker() -> Ticker {
        Ticker {
            instrument_name: "BTC-PERPETUAL".to_string(),
            timestamp: 1640995200000,
            best_bid_price: Some(50000.0),
            best_bid_amount: Some(1.5),
            best_ask_price: Some(50100.0),
            best_ask_amount: Some(2.0),
            last_price: Some(50050.0),
            mark_price: Some(50025.0),
            index_price: Some(50000.0),
            open_interest: 1000.0,
            volume_24h: 500.0,
            volume_usd_24h: 25000000.0,
            price_change_24h: 2.5,
            high_24h: Some(51000.0),
            low_24h: Some(49000.0),
            underlying_price: Some(50000.0),
            underlying_index: Some("btc_usd".to_string()),
            instrument_kind: Some(InstrumentKind::Future),
            current_funding: Some(0.0001),
            funding_8h: Some(0.0008),
            iv: None,
            greeks: None,
            interest_rate: Some(0.05),
        }
    }

    #[test]
    fn test_ticker_spread() {
        let ticker = create_test_ticker();
        assert_eq!(ticker.spread(), Some(100.0)); // 50100 - 50000

        let mut ticker_no_bid = ticker.clone();
        ticker_no_bid.best_bid_price = None;
        assert_eq!(ticker_no_bid.spread(), None);
    }

    #[test]
    fn test_ticker_mid_price() {
        let ticker = create_test_ticker();
        assert_eq!(ticker.mid_price(), Some(50050.0)); // (50100 + 50000) / 2

        let mut ticker_no_ask = ticker.clone();
        ticker_no_ask.best_ask_price = None;
        assert_eq!(ticker_no_ask.mid_price(), None);
    }

    #[test]
    fn test_ticker_spread_percentage() {
        let ticker = create_test_ticker();
        let expected = (100.0 / 50050.0) * 100.0;
        assert!((ticker.spread_percentage().unwrap() - expected).abs() < 0.001);

        let mut ticker_no_spread = ticker.clone();
        ticker_no_spread.best_bid_price = None;
        assert_eq!(ticker_no_spread.spread_percentage(), None);
    }

    #[test]
    fn test_ticker_has_valid_spread() {
        let ticker = create_test_ticker();
        assert!(ticker.has_valid_spread());

        let mut ticker_no_bid = ticker.clone();
        ticker_no_bid.best_bid_price = None;
        assert!(!ticker_no_bid.has_valid_spread());
    }

    #[test]
    fn test_order_book_entry_new() {
        let entry = OrderBookEntry::new(50000.0, 1.5);
        assert_eq!(entry.price, 50000.0);
        assert_eq!(entry.amount, 1.5);
    }

    #[test]
    fn test_order_book_entry_notional() {
        let entry = OrderBookEntry::new(50000.0, 1.5);
        assert_eq!(entry.notional(), 75000.0);
    }

    #[test]
    fn test_order_book_new() {
        let book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        assert_eq!(book.instrument_name, "BTC-PERPETUAL");
        assert_eq!(book.timestamp, 1640995200000);
        assert_eq!(book.change_id, 12345);
        assert!(book.bids.is_empty());
        assert!(book.asks.is_empty());
        assert_eq!(book.prev_change_id, None);
    }

    #[test]
    fn test_order_book_best_prices() {
        let mut book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        book.bids.push(OrderBookEntry::new(50000.0, 1.0));
        book.bids.push(OrderBookEntry::new(49900.0, 2.0));
        book.asks.push(OrderBookEntry::new(50100.0, 1.5));
        book.asks.push(OrderBookEntry::new(50200.0, 2.5));

        assert_eq!(book.best_bid(), Some(50000.0));
        assert_eq!(book.best_ask(), Some(50100.0));
    }

    #[test]
    fn test_order_book_spread() {
        let mut book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        book.bids.push(OrderBookEntry::new(50000.0, 1.0));
        book.asks.push(OrderBookEntry::new(50100.0, 1.5));

        assert_eq!(book.spread(), Some(100.0));
    }

    #[test]
    fn test_order_book_mid_price() {
        let mut book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        book.bids.push(OrderBookEntry::new(50000.0, 1.0));
        book.asks.push(OrderBookEntry::new(50100.0, 1.5));

        assert_eq!(book.mid_price(), Some(50050.0));
    }

    #[test]
    fn test_order_book_total_volumes() {
        let mut book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        book.bids.push(OrderBookEntry::new(50000.0, 1.0));
        book.bids.push(OrderBookEntry::new(49900.0, 2.0));
        book.asks.push(OrderBookEntry::new(50100.0, 1.5));
        book.asks.push(OrderBookEntry::new(50200.0, 2.5));

        assert_eq!(book.total_bid_volume(), 3.0);
        assert_eq!(book.total_ask_volume(), 4.0);
    }

    #[test]
    fn test_order_book_volume_at_price() {
        let mut book = OrderBook::new("BTC-PERPETUAL".to_string(), 1640995200000, 12345);
        book.bids.push(OrderBookEntry::new(50000.0, 1.0));
        book.asks.push(OrderBookEntry::new(50100.0, 1.5));

        assert_eq!(book.volume_at_price(50000.0, true), 1.0);
        assert_eq!(book.volume_at_price(50100.0, false), 1.5);
        assert_eq!(book.volume_at_price(49000.0, true), 0.0);
    }

    #[test]
    fn test_candle_is_bullish() {
        let bullish_candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49500.0,
            close: 50500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert!(bullish_candle.is_bullish());
        assert!(!bullish_candle.is_bearish());
    }

    #[test]
    fn test_candle_is_bearish() {
        let bearish_candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 50200.0,
            low: 49000.0,
            close: 49500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert!(bearish_candle.is_bearish());
        assert!(!bearish_candle.is_bullish());
    }

    #[test]
    fn test_candle_body_size() {
        let candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert_eq!(candle.body_size(), 500.0);
    }

    #[test]
    fn test_candle_upper_shadow() {
        let candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert_eq!(candle.upper_shadow(), 500.0); // 51000 - 50500
    }

    #[test]
    fn test_candle_lower_shadow() {
        let candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert_eq!(candle.lower_shadow(), 1000.0); // 50000 - 49000
    }

    #[test]
    fn test_candle_range() {
        let candle = Candle {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
            trades: Some(50),
        };
        assert_eq!(candle.range(), 2000.0); // 51000 - 49000
    }

    #[test]
    fn test_greeks_creation() {
        let greeks = Greeks {
            delta: 0.5,
            gamma: 0.01,
            theta: -0.05,
            vega: 0.1,
            rho: Some(0.02),
        };
        assert_eq!(greeks.delta, 0.5);
        assert_eq!(greeks.rho, Some(0.02));
    }

    #[test]
    fn test_market_stats_creation() {
        let stats = MarketStats {
            currency: "BTC".to_string(),
            volume_24h: 1000.0,
            volume_change_24h: 5.0,
            price_change_24h: 2.5,
            high_24h: 51000.0,
            low_24h: 49000.0,
            active_instruments: 50,
            total_open_interest: 10000.0,
        };
        assert_eq!(stats.currency, "BTC");
        assert_eq!(stats.active_instruments, 50);
    }

    #[test]
    fn test_serialization() {
        let ticker = create_test_ticker();
        let json = serde_json::to_string(&ticker).unwrap();
        let deserialized: Ticker = serde_json::from_str(&json).unwrap();
        assert_eq!(ticker.instrument_name, deserialized.instrument_name);
        assert_eq!(ticker.best_bid_price, deserialized.best_bid_price);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let ticker = create_test_ticker();
        let debug_str = format!("{:?}", ticker);
        let display_str = format!("{}", ticker);

        assert!(debug_str.contains("BTC-PERPETUAL"));
        assert!(display_str.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_mark_price_point_new() {
        let point = MarkPricePoint::new(1640995200000, 50000.0);
        assert_eq!(point.timestamp, 1640995200000);
        assert!((point.mark_price - 50000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mark_price_point_from_tuple() {
        let point = MarkPricePoint::from_tuple((1640995200000, 50000.0));
        assert_eq!(point.timestamp, 1640995200000);
        assert!((point.mark_price - 50000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mark_price_history_new() {
        let history = MarkPriceHistory::new("BTC-25JUN21-50000-C".to_string());
        assert_eq!(history.instrument_name, "BTC-25JUN21-50000-C");
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_mark_price_history_from_raw() {
        let data = vec![
            (1640995200000, 0.5165),
            (1640995201000, 0.5166),
            (1640995202000, 0.5167),
        ];
        let history = MarkPriceHistory::from_raw("BTC-25JUN21-50000-C".to_string(), data);
        assert_eq!(history.len(), 3);
        assert!(!history.is_empty());
    }

    #[test]
    fn test_mark_price_history_latest_earliest() {
        let data = vec![
            (1640995200000, 0.5165),
            (1640995202000, 0.5167),
            (1640995201000, 0.5166),
        ];
        let history = MarkPriceHistory::from_raw("BTC-25JUN21-50000-C".to_string(), data);

        let latest = history.latest();
        assert!(latest.is_some());
        assert_eq!(latest.map(|p| p.timestamp), Some(1640995202000));

        let earliest = history.earliest();
        assert!(earliest.is_some());
        assert_eq!(earliest.map(|p| p.timestamp), Some(1640995200000));
    }

    #[test]
    fn test_mark_price_history_serialization() {
        let history = MarkPriceHistory::from_raw(
            "BTC-25JUN21-50000-C".to_string(),
            vec![(1640995200000, 0.5165)],
        );
        let json = serde_json::to_string(&history).unwrap();
        let deserialized: MarkPriceHistory = serde_json::from_str(&json).unwrap();
        assert_eq!(history, deserialized);
    }

    #[test]
    fn test_trade_volume_new() {
        let volume = TradeVolume::new("BTC".to_string(), 48.0, 145.0, 6.25, 11.1);
        assert_eq!(volume.currency, "BTC");
        assert!((volume.puts_volume - 48.0).abs() < f64::EPSILON);
        assert!((volume.calls_volume - 145.0).abs() < f64::EPSILON);
        assert!((volume.futures_volume - 6.25).abs() < f64::EPSILON);
        assert!((volume.spot_volume - 11.1).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trade_volume_total_options() {
        let volume = TradeVolume::new("BTC".to_string(), 48.0, 145.0, 6.25, 11.1);
        assert!((volume.total_options_volume() - 193.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trade_volume_total() {
        let volume = TradeVolume::new("BTC".to_string(), 48.0, 145.0, 6.25, 11.1);
        let expected = 48.0 + 145.0 + 6.25 + 11.1;
        assert!((volume.total_volume() - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trade_volume_put_call_ratio() {
        let volume = TradeVolume::new("BTC".to_string(), 48.0, 145.0, 6.25, 11.1);
        let ratio = volume.put_call_ratio();
        assert!(ratio.is_some());
        assert!((ratio.unwrap() - (48.0 / 145.0)).abs() < 0.001);

        let volume_zero_calls = TradeVolume::new("BTC".to_string(), 48.0, 0.0, 6.25, 11.1);
        assert!(volume_zero_calls.put_call_ratio().is_none());
    }

    #[test]
    fn test_trade_volume_serialization() {
        let volume = TradeVolume::new("BTC".to_string(), 48.0, 145.0, 6.25, 11.1);
        let json = serde_json::to_string(&volume).unwrap();
        let deserialized: TradeVolume = serde_json::from_str(&json).unwrap();
        assert_eq!(volume.currency, deserialized.currency);
        assert!((volume.puts_volume - deserialized.puts_volume).abs() < f64::EPSILON);
    }

    #[test]
    fn test_volatility_index_candle_new() {
        let candle = VolatilityIndexCandle::new(1640995200000, 0.21, 0.22, 0.20, 0.215);
        assert_eq!(candle.timestamp, 1640995200000);
        assert!((candle.open - 0.21).abs() < f64::EPSILON);
        assert!((candle.high - 0.22).abs() < f64::EPSILON);
        assert!((candle.low - 0.20).abs() < f64::EPSILON);
        assert!((candle.close - 0.215).abs() < f64::EPSILON);
    }

    #[test]
    fn test_volatility_index_candle_from_tuple() {
        let candle = VolatilityIndexCandle::from_tuple((1640995200000, 0.21, 0.22, 0.20, 0.215));
        assert_eq!(candle.timestamp, 1640995200000);
        assert!((candle.range() - 0.02).abs() < f64::EPSILON);
    }

    #[test]
    fn test_volatility_index_candle_increasing_decreasing() {
        let increasing = VolatilityIndexCandle::new(1640995200000, 0.20, 0.22, 0.19, 0.21);
        assert!(increasing.is_increasing());
        assert!(!increasing.is_decreasing());

        let decreasing = VolatilityIndexCandle::new(1640995200000, 0.21, 0.22, 0.19, 0.20);
        assert!(decreasing.is_decreasing());
        assert!(!decreasing.is_increasing());
    }

    #[test]
    fn test_volatility_index_data_new() {
        let data = VolatilityIndexData::new("BTC".to_string());
        assert_eq!(data.currency, "BTC");
        assert!(data.is_empty());
        assert_eq!(data.len(), 0);
        assert!(!data.has_more());
    }

    #[test]
    fn test_volatility_index_data_from_raw() {
        let raw_data = vec![
            (1640995200000, 0.21, 0.22, 0.20, 0.215),
            (1640995260000, 0.215, 0.23, 0.21, 0.22),
        ];
        let data = VolatilityIndexData::from_raw("BTC".to_string(), raw_data, None);
        assert_eq!(data.len(), 2);
        assert!(!data.has_more());
    }

    #[test]
    fn test_volatility_index_data_with_continuation() {
        let raw_data = vec![(1640995200000, 0.21, 0.22, 0.20, 0.215)];
        let data = VolatilityIndexData::from_raw(
            "BTC".to_string(),
            raw_data,
            Some("next_page_token".to_string()),
        );
        assert!(data.has_more());
        assert_eq!(data.continuation, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_volatility_index_data_latest_earliest() {
        let raw_data = vec![
            (1640995200000, 0.21, 0.22, 0.20, 0.215),
            (1640995320000, 0.22, 0.24, 0.21, 0.23),
            (1640995260000, 0.215, 0.23, 0.21, 0.22),
        ];
        let data = VolatilityIndexData::from_raw("BTC".to_string(), raw_data, None);

        let latest = data.latest();
        assert!(latest.is_some());
        assert_eq!(latest.map(|c| c.timestamp), Some(1640995320000));

        let earliest = data.earliest();
        assert!(earliest.is_some());
        assert_eq!(earliest.map(|c| c.timestamp), Some(1640995200000));
    }

    #[test]
    fn test_volatility_index_data_serialization() {
        let data = VolatilityIndexData::from_raw(
            "BTC".to_string(),
            vec![(1640995200000, 0.21, 0.22, 0.20, 0.215)],
            None,
        );
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: VolatilityIndexData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_index_type_default() {
        let index_type = IndexType::default();
        assert_eq!(index_type, IndexType::All);
    }

    #[test]
    fn test_index_type_as_str() {
        assert_eq!(IndexType::All.as_str(), "all");
        assert_eq!(IndexType::Spot.as_str(), "spot");
        assert_eq!(IndexType::Derivative.as_str(), "derivative");
    }

    #[test]
    fn test_index_type_display() {
        assert_eq!(format!("{}", IndexType::All), "all");
        assert_eq!(format!("{}", IndexType::Spot), "spot");
        assert_eq!(format!("{}", IndexType::Derivative), "derivative");
    }

    #[test]
    fn test_index_type_serialization() {
        let index_type = IndexType::Spot;
        let json = serde_json::to_string(&index_type).unwrap();
        assert_eq!(json, "\"spot\"");

        let deserialized: IndexType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, IndexType::Spot);
    }
}
