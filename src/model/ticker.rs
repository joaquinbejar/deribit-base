use serde::{Deserialize, Serialize};

/// Greeks sub-structure for options
#[derive(Clone, Serialize, Deserialize)]
pub struct Greeks {
    /// Delta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Gamma value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Vega value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    /// Theta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Rho value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
}

/// Ticker stats sub-structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// Trading volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    /// Trading volume in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usd: Option<f64>,
    /// Price change from previous period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change: Option<f64>,
    /// Highest price in the period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    /// Lowest price in the period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
}

/// Ticker data structure with corrected field types
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Name of the instrument
    pub instrument_name: String,
    /// Last traded price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    /// Current mark price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    /// Best bid price available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_bid_price: Option<f64>,
    /// Best ask price available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_ask_price: Option<f64>,
    /// Amount available at best bid price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_bid_amount: Option<f64>,
    /// Amount available at best ask price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_ask_amount: Option<f64>,
    /// Trading volume in base currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    /// Trading volume in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usd: Option<f64>,
    /// Open interest for the instrument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    /// Highest price in 24h period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    /// Lowest price in 24h period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    /// Absolute price change in 24h
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change: Option<f64>,
    /// Percentage price change in 24h
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change_percentage: Option<f64>,
    /// Implied volatility at best bid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_iv: Option<f64>,
    /// Implied volatility at best ask
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_iv: Option<f64>,
    /// Mark implied volatility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_iv: Option<f64>,
    /// Timestamp of the ticker data
    pub timestamp: u64,
    /// Current state of the instrument
    pub state: String,
    /// Settlement price (for expired instruments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<f64>,
    /// Additional ticker statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<TickerStats>,
    /// Greeks for options (delta, gamma, vega, theta, rho)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeks: Option<Greeks>,
    /// Index price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<f64>,
    /// Minimum price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<f64>,
    /// Maximum price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    /// Interest rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
    /// Underlying price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// Underlying index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_index: Option<String>,
    /// Estimated delivery price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_price: Option<f64>,
}

crate::impl_json_display!(TickerData);
crate::impl_json_debug_pretty!(TickerData);

crate::impl_json_display!(TickerStats);
crate::impl_json_debug_pretty!(TickerStats);

crate::impl_json_display!(Greeks);
crate::impl_json_debug_pretty!(Greeks);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ticker_data_serialization() {
        let json_str = r#"{
            "timestamp": 1757433676689,
            "state": "open",
            "stats": {
                "high": 0.0002,
                "low": 0.0001,
                "price_change": 100.0,
                "volume": 70.7,
                "volume_usd": 974.07
            },
            "greeks": {
                "delta": -0.01874,
                "gamma": 2.0e-5,
                "vega": 2.16672,
                "theta": -15.99927,
                "rho": -0.03815
            },
            "index_price": 110881.2,
            "instrument_name": "BTC-10SEP25-106000-P",
            "last_price": 0.0002,
            "settlement_price": 2.533e-4,
            "min_price": 0.0001,
            "max_price": 0.0175,
            "open_interest": 18.6,
            "mark_price": 0.0001,
            "best_bid_price": 0.0001,
            "best_ask_price": 0.0002,
            "interest_rate": 0.0,
            "mark_iv": 49.22,
            "bid_iv": 46.67,
            "ask_iv": 51.77,
            "underlying_price": 110714.7602,
            "underlying_index": "SYN.BTC-10SEP25",
            "estimated_delivery_price": 110881.2,
            "best_ask_amount": 4.1,
            "best_bid_amount": 2.2
        }"#;

        // Test deserialization
        let ticker_data: TickerData =
            serde_json::from_str(json_str).expect("Failed to deserialize ticker data");

        // Verify some key fields
        assert_eq!(ticker_data.instrument_name, "BTC-10SEP25-106000-P");
        assert_eq!(ticker_data.timestamp, 1757433676689);
        assert_eq!(ticker_data.state, "open");
        assert_eq!(ticker_data.last_price, Some(0.0002));
        assert_eq!(ticker_data.mark_price, Some(0.0001));
        assert_eq!(ticker_data.best_bid_price, Some(0.0001));
        assert_eq!(ticker_data.best_ask_price, Some(0.0002));
        assert_eq!(ticker_data.best_bid_amount, Some(2.2));
        assert_eq!(ticker_data.best_ask_amount, Some(4.1));
        assert_eq!(ticker_data.open_interest, Some(18.6));
        assert_eq!(ticker_data.settlement_price, Some(2.533e-4));
        assert_eq!(ticker_data.min_price, Some(0.0001));
        assert_eq!(ticker_data.max_price, Some(0.0175));
        assert_eq!(ticker_data.interest_rate, Some(0.0));
        assert_eq!(ticker_data.mark_iv, Some(49.22));
        assert_eq!(ticker_data.bid_iv, Some(46.67));
        assert_eq!(ticker_data.ask_iv, Some(51.77));
        assert_eq!(ticker_data.underlying_price, Some(110714.7602));
        assert_eq!(
            ticker_data.underlying_index,
            Some("SYN.BTC-10SEP25".to_string())
        );
        assert_eq!(ticker_data.estimated_delivery_price, Some(110881.2));
        assert_eq!(ticker_data.index_price, Some(110881.2));

        // Verify stats
        let stats = ticker_data.stats.as_ref().expect("Stats should be present");
        assert_eq!(stats.high, Some(0.0002));
        assert_eq!(stats.low, Some(0.0001));
        assert_eq!(stats.price_change, Some(100.0));
        assert_eq!(stats.volume, Some(70.7));
        assert_eq!(stats.volume_usd, Some(974.07));

        // Verify greeks
        let greeks = ticker_data
            .greeks
            .as_ref()
            .expect("Greeks should be present");
        assert_eq!(greeks.delta, Some(-0.01874));
        assert_eq!(greeks.gamma, Some(2.0e-5));
        assert_eq!(greeks.vega, Some(2.16672));
        assert_eq!(greeks.theta, Some(-15.99927));
        assert_eq!(greeks.rho, Some(-0.03815));

        // Test serialization back to JSON
        let serialized =
            serde_json::to_string(&ticker_data).expect("Failed to serialize ticker data");

        // Verify we can deserialize it again
        let _: TickerData = serde_json::from_str(&serialized)
            .expect("Failed to deserialize serialized ticker data");
    }
}
