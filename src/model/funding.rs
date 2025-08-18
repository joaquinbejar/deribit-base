/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Funding chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingChartData {
    /// Current interest rate
    pub current_interest: f64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// Historical funding data points
    pub data: Vec<FundingDataPoint>,
}

impl FundingChartData {
    /// Create new funding chart data
    pub fn new() -> Self {
        Self {
            current_interest: 0.0,
            interest_8h: 0.0,
            data: Vec::new(),
        }
    }
}

impl Default for FundingChartData {
    fn default() -> Self {
        Self::new()
    }
}

impl_json_display!(FundingChartData);
impl_json_debug_pretty!(FundingChartData);

/// Funding data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingDataPoint {
    /// Index price at the time
    pub index_price: f64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// Timestamp of the data point
    pub timestamp: u64,
}

impl FundingDataPoint {
    /// Create new funding data point
    pub fn new(index_price: f64, interest_8h: f64, timestamp: u64) -> Self {
        Self {
            index_price,
            interest_8h,
            timestamp,
        }
    }
}

impl_json_display!(FundingDataPoint);
impl_json_debug_pretty!(FundingDataPoint);

/// Funding rate data structure for historical funding rates
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingRateData {
    /// Funding rate value
    pub funding_rate: f64,
    /// Index price at the time
    pub index_price: f64,
    /// Interest rate
    pub interest_rate: f64,
    /// Previous index price
    pub prev_index_price: f64,
    /// Timestamp of the funding event
    pub timestamp: u64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// 1h interest rate
    pub interest_1h: f64,
}

impl FundingRateData {
    /// Create new funding rate data
    pub fn new(
        funding_rate: f64,
        index_price: f64,
        interest_rate: f64,
        prev_index_price: f64,
        timestamp: u64,
    ) -> Self {
        Self {
            funding_rate,
            index_price,
            interest_rate,
            prev_index_price,
            timestamp,
            interest_8h: 0.0,
            interest_1h: 0.0,
        }
    }

    /// Set interest rates
    pub fn with_interest_rates(mut self, interest_8h: f64, interest_1h: f64) -> Self {
        self.interest_8h = interest_8h;
        self.interest_1h = interest_1h;
        self
    }
}

impl_json_display!(FundingRateData);
impl_json_debug_pretty!(FundingRateData);

/// TradingView chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TradingViewChartData {
    /// Status of the data
    pub status: String,
    /// Array of timestamps
    pub ticks: Vec<u64>,
    /// Array of open prices
    pub open: Vec<f64>,
    /// Array of high prices
    pub high: Vec<f64>,
    /// Array of low prices
    pub low: Vec<f64>,
    /// Array of close prices
    pub close: Vec<f64>,
    /// Array of volumes
    pub volume: Vec<f64>,
    /// Array of costs
    pub cost: Vec<f64>,
}

impl TradingViewChartData {
    /// Create new TradingView chart data
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
            ticks: Vec::new(),
            open: Vec::new(),
            high: Vec::new(),
            low: Vec::new(),
            close: Vec::new(),
            volume: Vec::new(),
            cost: Vec::new(),
        }
    }

    /// Add a new candle to the data
    pub fn add_candle(
        &mut self,
        timestamp: u64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        cost: f64,
    ) {
        self.ticks.push(timestamp);
        self.open.push(open);
        self.high.push(high);
        self.low.push(low);
        self.close.push(close);
        self.volume.push(volume);
        self.cost.push(cost);
    }
}

impl Default for TradingViewChartData {
    fn default() -> Self {
        Self::new()
    }
}

impl_json_display!(TradingViewChartData);
impl_json_debug_pretty!(TradingViewChartData);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_chart_data_creation() {
        let chart_data = FundingChartData::new();
        assert_eq!(chart_data.current_interest, 0.0);
        assert_eq!(chart_data.interest_8h, 0.0);
        assert!(chart_data.data.is_empty());
    }

    #[test]
    fn test_trading_view_chart_data_creation() {
        let mut chart_data = TradingViewChartData::new();
        chart_data.add_candle(
            1640995200000, // timestamp
            45000.0,       // open
            45500.0,       // high
            44800.0,       // low
            45200.0,       // close
            100.0,         // volume
            4520000.0,     // cost
        );

        assert_eq!(chart_data.ticks.len(), 1);
        assert_eq!(chart_data.open[0], 45000.0);
        assert_eq!(chart_data.high[0], 45500.0);
    }

    #[test]
    fn test_serde() {
        let funding_data = FundingRateData::new(
            0.0001,        // funding_rate
            45000.0,       // index_price
            0.05,          // interest_rate
            44900.0,       // prev_index_price
            1640995200000, // timestamp
        );

        let json = serde_json::to_string(&funding_data).unwrap();
        let deserialized: FundingRateData = serde_json::from_str(&json).unwrap();
        assert_eq!(funding_data.funding_rate, deserialized.funding_rate);
    }
}
