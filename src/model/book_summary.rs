/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::prelude::Currencies;
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Book summary information for an instrument
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BookSummary {
    /// Instrument name
    pub instrument_name: String,
    /// Base currency
    pub base_currency: Currencies,
    /// Quote currency (usually USD)
    pub quote_currency: String,
    /// 24h trading volume
    pub volume: f64,
    /// 24h trading volume in USD
    pub volume_usd: f64,
    /// Open interest
    pub open_interest: f64,
    /// 24h price change percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change: Option<f64>,
    /// Current mark price
    pub mark_price: f64,
    /// Mark implied volatility (options only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_iv: Option<f64>,
    /// Best bid price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// Best ask price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Mid price (bid + ask) / 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_price: Option<f64>,
    /// Last trade price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// 24h high price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    /// 24h low price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    /// Estimated delivery price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_price: Option<f64>,
    /// Current funding rate (perpetuals only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_funding: Option<f64>,
    /// 8h funding rate (perpetuals only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_8h: Option<f64>,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub creation_timestamp: i64,
}

impl BookSummary {
    /// Create a new book summary
    pub fn new(
        instrument_name: String,
        base_currency: Currencies,
        quote_currency: String,
        mark_price: f64,
        creation_timestamp: i64,
    ) -> Self {
        Self {
            instrument_name,
            base_currency,
            quote_currency,
            volume: 0.0,
            volume_usd: 0.0,
            open_interest: 0.0,
            price_change: None,
            mark_price,
            mark_iv: None,
            bid_price: None,
            ask_price: None,
            mid_price: None,
            last: None,
            high: None,
            low: None,
            estimated_delivery_price: None,
            current_funding: None,
            funding_8h: None,
            creation_timestamp,
        }
    }

    /// Set volume information
    pub fn with_volume(mut self, volume: f64, volume_usd: f64) -> Self {
        self.volume = volume;
        self.volume_usd = volume_usd;
        self
    }

    /// Set price information
    pub fn with_prices(
        mut self,
        bid: Option<f64>,
        ask: Option<f64>,
        last: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
    ) -> Self {
        self.bid_price = bid;
        self.ask_price = ask;
        self.last = last;
        self.high = high;
        self.low = low;

        // Calculate mid price if both bid and ask are available
        if let (Some(bid), Some(ask)) = (bid, ask) {
            self.mid_price = Some((bid + ask) / 2.0);
        }

        self
    }

    /// Set open interest
    pub fn with_open_interest(mut self, open_interest: f64) -> Self {
        self.open_interest = open_interest;
        self
    }

    /// Set price change percentage
    pub fn with_price_change(mut self, price_change: f64) -> Self {
        self.price_change = Some(price_change);
        self
    }

    /// Set implied volatility (for options)
    pub fn with_iv(mut self, mark_iv: f64) -> Self {
        self.mark_iv = Some(mark_iv);
        self
    }

    /// Set funding rates (for perpetuals)
    pub fn with_funding(mut self, current: f64, funding_8h: f64) -> Self {
        self.current_funding = Some(current);
        self.funding_8h = Some(funding_8h);
        self
    }

    /// Set estimated delivery price
    pub fn with_delivery_price(mut self, price: f64) -> Self {
        self.estimated_delivery_price = Some(price);
        self
    }

    /// Get spread (ask - bid)
    pub fn spread(&self) -> Option<f64> {
        match (self.bid_price, self.ask_price) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price) {
            (Some(spread), Some(mid)) if mid > 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Check if this is a perpetual contract
    pub fn is_perpetual(&self) -> bool {
        self.instrument_name.contains("PERPETUAL")
    }

    /// Check if this is an option
    pub fn is_option(&self) -> bool {
        // Options end with -C or -P (call/put) but not PERPETUAL
        !self.is_perpetual()
            && (self.instrument_name.ends_with("-C") || self.instrument_name.ends_with("-P"))
    }

    /// Check if this is a future
    pub fn is_future(&self) -> bool {
        !self.is_perpetual() && !self.is_option()
    }

    /// Get 24h price change in absolute terms
    pub fn price_change_absolute(&self) -> Option<f64> {
        self.price_change.map(|change| {
            if let Some(last) = self.last {
                last * (change / 100.0)
            } else {
                self.mark_price * (change / 100.0)
            }
        })
    }
}

impl_json_display!(BookSummary);
impl_json_debug_pretty!(BookSummary);

/// Collection of book summaries
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BookSummaries {
    /// List of book summaries
    pub summaries: Vec<BookSummary>,
}

impl BookSummaries {
    /// Create a new collection
    pub fn new() -> Self {
        Self {
            summaries: Vec::new(),
        }
    }

    /// Add a book summary
    pub fn add(&mut self, summary: BookSummary) {
        self.summaries.push(summary);
    }

    /// Get summaries by currency
    pub fn by_currency(&self, currency: Currencies) -> Vec<&BookSummary> {
        self.summaries
            .iter()
            .filter(|s| s.base_currency == currency)
            .collect()
    }

    /// Get summaries by instrument type
    pub fn perpetuals(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_perpetual()).collect()
    }

    /// Get option summaries
    pub fn options(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_option()).collect()
    }

    /// Get future summaries
    pub fn futures(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_future()).collect()
    }

    /// Sort by volume (descending)
    pub fn sort_by_volume(&mut self) {
        self.summaries
            .sort_by(|a, b| b.volume_usd.partial_cmp(&a.volume_usd).unwrap());
    }

    /// Sort by open interest (descending)
    pub fn sort_by_open_interest(&mut self) {
        self.summaries
            .sort_by(|a, b| b.open_interest.partial_cmp(&a.open_interest).unwrap());
    }
}

impl Default for BookSummaries {
    fn default() -> Self {
        Self::new()
    }
}

impl_json_display!(BookSummaries);
impl_json_debug_pretty!(BookSummaries);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_summary_creation() {
        let summary = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            45000.0,
            1640995200000,
        );

        assert_eq!(summary.instrument_name, "BTC-PERPETUAL");
        assert_eq!(summary.base_currency, Currencies::Bitcoin);
        assert_eq!(summary.mark_price, 45000.0);
        assert!(summary.is_perpetual());
    }

    #[test]
    fn test_book_summary_builder() {
        let summary = BookSummary::new(
            "BTC-25MAR23-50000-C".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            2500.0,
            1640995200000,
        )
        .with_volume(100.0, 4500000.0)
        .with_prices(
            Some(2480.0),
            Some(2520.0),
            Some(2500.0),
            Some(2600.0),
            Some(2400.0),
        )
        .with_iv(85.5);

        assert_eq!(summary.volume, 100.0);
        assert_eq!(summary.volume_usd, 4500000.0);
        assert_eq!(summary.bid_price, Some(2480.0));
        assert_eq!(summary.ask_price, Some(2520.0));
        assert_eq!(summary.mid_price, Some(2500.0));
        assert_eq!(summary.mark_iv, Some(85.5));
        assert!(summary.is_option());
    }

    #[test]
    fn test_spread_calculation() {
        let summary = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            45000.0,
            1640995200000,
        )
        .with_prices(Some(44950.0), Some(45050.0), None, None, None);

        assert_eq!(summary.spread(), Some(100.0));
        assert_eq!(summary.mid_price, Some(45000.0));

        let spread_pct = summary.spread_percentage().unwrap();
        assert!((spread_pct - 0.2222).abs() < 0.001); // ~0.22%
    }

    #[test]
    fn test_instrument_type_detection() {
        let perpetual = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            45000.0,
            0,
        );
        assert!(perpetual.is_perpetual());
        assert!(!perpetual.is_option());
        assert!(!perpetual.is_future());

        let option = BookSummary::new(
            "BTC-25MAR23-50000-C".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            2500.0,
            0,
        );
        assert!(!option.is_perpetual());
        assert!(option.is_option());
        assert!(!option.is_future());

        let future = BookSummary::new(
            "BTC-25MAR23".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            45000.0,
            0,
        );
        assert!(!future.is_perpetual());
        assert!(!future.is_option());
        assert!(future.is_future());
    }

    #[test]
    fn test_book_summaries_collection() {
        let mut summaries = BookSummaries::new();

        summaries.add(
            BookSummary::new(
                "BTC-PERPETUAL".to_string(),
                Currencies::Bitcoin,
                "USD".to_string(),
                45000.0,
                0,
            )
            .with_volume(1000.0, 45000000.0),
        );

        summaries.add(
            BookSummary::new(
                "ETH-PERPETUAL".to_string(),
                Currencies::Ethereum,
                "USD".to_string(),
                3000.0,
                0,
            )
            .with_volume(500.0, 1500000.0),
        );

        assert_eq!(summaries.summaries.len(), 2);
        assert_eq!(summaries.by_currency(Currencies::Bitcoin).len(), 1);
        assert_eq!(summaries.perpetuals().len(), 2);

        summaries.sort_by_volume();
        assert_eq!(summaries.summaries[0].base_currency, Currencies::Bitcoin);
    }

    #[test]
    fn test_serde() {
        let summary = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            Currencies::Bitcoin,
            "USD".to_string(),
            45000.0,
            1640995200000,
        )
        .with_funding(0.0001, 0.0008);

        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: BookSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, deserialized);
    }
}
