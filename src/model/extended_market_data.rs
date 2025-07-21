/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::prelude::Currency;
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Withdrawal priority information
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalPriority {
    /// Priority name (e.g., "very_low", "low", "medium", "high", "very_high")
    pub name: String,
    /// Priority value (fee multiplier)
    pub value: f64,
}

impl WithdrawalPriority {
    /// Create a new withdrawal priority
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }

    /// Create a very low priority
    pub fn very_low() -> Self {
        Self::new("very_low".to_string(), 0.15)
    }

    /// Create a low priority
    pub fn low() -> Self {
        Self::new("low".to_string(), 0.5)
    }

    /// Create a medium priority
    pub fn medium() -> Self {
        Self::new("medium".to_string(), 1.0)
    }

    /// Create a high priority
    pub fn high() -> Self {
        Self::new("high".to_string(), 1.2)
    }

    /// Create a very high priority
    pub fn very_high() -> Self {
        Self::new("very_high".to_string(), 1.5)
    }
}

impl_json_display!(WithdrawalPriority);
impl_json_debug_pretty!(WithdrawalPriority);

/// Currency information and configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyInfo {
    /// Coin type identifier (e.g., "BITCOIN", "ETHEREUM")
    pub coin_type: String,
    /// Currency code
    pub currency: Currency,
    /// Full currency name
    pub currency_long: String,
    /// Fee precision (decimal places)
    pub fee_precision: i32,
    /// Minimum confirmations required
    pub min_confirmations: i32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Standard withdrawal fee
    pub withdrawal_fee: f64,
    /// Available withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// Whether the currency is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Minimum deposit amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_deposit_amount: Option<f64>,
    /// Maximum withdrawal amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_withdrawal_amount: Option<f64>,
}

impl CurrencyInfo {
    /// Create new currency info
    pub fn new(
        coin_type: String,
        currency: Currency,
        currency_long: String,
        fee_precision: i32,
        min_confirmations: i32,
        min_withdrawal_fee: f64,
        withdrawal_fee: f64,
    ) -> Self {
        Self {
            coin_type,
            currency,
            currency_long,
            fee_precision,
            min_confirmations,
            min_withdrawal_fee,
            withdrawal_fee,
            withdrawal_priorities: Vec::new(),
            disabled: None,
            min_deposit_amount: None,
            max_withdrawal_amount: None,
        }
    }

    /// Add withdrawal priority
    pub fn add_priority(&mut self, priority: WithdrawalPriority) {
        self.withdrawal_priorities.push(priority);
    }

    /// Set disabled status
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set deposit limits
    pub fn with_deposit_limit(mut self, min_amount: f64) -> Self {
        self.min_deposit_amount = Some(min_amount);
        self
    }

    /// Set withdrawal limits
    pub fn with_withdrawal_limit(mut self, max_amount: f64) -> Self {
        self.max_withdrawal_amount = Some(max_amount);
        self
    }

    /// Check if currency is enabled
    pub fn is_enabled(&self) -> bool {
        !self.disabled.unwrap_or(false)
    }

    /// Get priority by name
    pub fn get_priority(&self, name: &str) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities.iter().find(|p| p.name == name)
    }

    /// Get highest priority
    pub fn highest_priority(&self) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }

    /// Get lowest priority
    pub fn lowest_priority(&self) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities
            .iter()
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }
}

impl_json_display!(CurrencyInfo);
impl_json_debug_pretty!(CurrencyInfo);

/// Index price information
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexPrice {
    /// Estimated delivery price
    pub estimated_delivery_price: f64,
    /// Current index price
    pub index_price: f64,
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Index name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
}

impl IndexPrice {
    /// Create new index price
    pub fn new(estimated_delivery_price: f64, index_price: f64, timestamp: i64) -> Self {
        Self {
            estimated_delivery_price,
            index_price,
            timestamp,
            index_name: None,
        }
    }

    /// Set index name
    pub fn with_name(mut self, name: String) -> Self {
        self.index_name = Some(name);
        self
    }

    /// Get price difference
    pub fn price_difference(&self) -> f64 {
        self.estimated_delivery_price - self.index_price
    }

    /// Get price difference percentage
    pub fn price_difference_percentage(&self) -> f64 {
        if self.index_price != 0.0 {
            (self.price_difference() / self.index_price) * 100.0
        } else {
            0.0
        }
    }
}

impl_json_display!(IndexPrice);
impl_json_debug_pretty!(IndexPrice);

/// Funding rate information
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingRate {
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Index name
    pub index_name: String,
    /// Interest rate
    pub interest_rate: f64,
    /// 8-hour interest rate
    pub interest_8h: f64,
    /// Current funding rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_funding: Option<f64>,
    /// Next funding timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_funding_timestamp: Option<i64>,
}

impl FundingRate {
    /// Create new funding rate
    pub fn new(timestamp: i64, index_name: String, interest_rate: f64, interest_8h: f64) -> Self {
        Self {
            timestamp,
            index_name,
            interest_rate,
            interest_8h,
            current_funding: None,
            next_funding_timestamp: None,
        }
    }

    /// Set current funding
    pub fn with_current_funding(mut self, funding: f64) -> Self {
        self.current_funding = Some(funding);
        self
    }

    /// Set next funding timestamp
    pub fn with_next_funding(mut self, timestamp: i64) -> Self {
        self.next_funding_timestamp = Some(timestamp);
        self
    }

    /// Calculate annualized rate
    pub fn annualized_rate(&self) -> f64 {
        self.interest_rate * 365.0 * 3.0 // 3 times per day (8h intervals)
    }

    /// Check if funding is positive (longs pay shorts)
    pub fn is_positive(&self) -> bool {
        self.current_funding.unwrap_or(0.0) > 0.0
    }

    /// Check if funding is negative (shorts pay longs)
    pub fn is_negative(&self) -> bool {
        self.current_funding.unwrap_or(0.0) < 0.0
    }
}

impl_json_display!(FundingRate);
impl_json_debug_pretty!(FundingRate);

/// Historical volatility information
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalVolatility {
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Volatility value (percentage)
    pub volatility: f64,
    /// Period in days
    pub period_days: i32,
    /// Underlying asset
    pub underlying: String,
}

impl HistoricalVolatility {
    /// Create new historical volatility
    pub fn new(timestamp: i64, volatility: f64, period_days: i32, underlying: String) -> Self {
        Self {
            timestamp,
            volatility,
            period_days,
            underlying,
        }
    }

    /// Convert to decimal form
    pub fn as_decimal(&self) -> f64 {
        self.volatility / 100.0
    }

    /// Annualize volatility if needed
    pub fn annualized(&self) -> f64 {
        if self.period_days == 365 {
            self.volatility
        } else {
            self.volatility * (365.0 / self.period_days as f64).sqrt()
        }
    }
}

impl_json_display!(HistoricalVolatility);
impl_json_debug_pretty!(HistoricalVolatility);

/// Market statistics
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketStatistics {
    /// Currency
    pub currency: Currency,
    /// Total volume 24h
    pub volume_24h: f64,
    /// Total volume 30d
    pub volume_30d: f64,
    /// Volume in USD 24h
    pub volume_usd_24h: f64,
    /// Volume in USD 30d
    pub volume_usd_30d: f64,
    /// Number of trades 24h
    pub trades_count_24h: i64,
    /// Number of trades 30d
    pub trades_count_30d: i64,
    /// Open interest
    pub open_interest: f64,
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
}

impl MarketStatistics {
    /// Create new market statistics
    pub fn new(currency: Currency, timestamp: i64) -> Self {
        Self {
            currency,
            volume_24h: 0.0,
            volume_30d: 0.0,
            volume_usd_24h: 0.0,
            volume_usd_30d: 0.0,
            trades_count_24h: 0,
            trades_count_30d: 0,
            open_interest: 0.0,
            timestamp,
        }
    }

    /// Set volume data
    pub fn with_volume(
        mut self,
        vol_24h: f64,
        vol_30d: f64,
        vol_usd_24h: f64,
        vol_usd_30d: f64,
    ) -> Self {
        self.volume_24h = vol_24h;
        self.volume_30d = vol_30d;
        self.volume_usd_24h = vol_usd_24h;
        self.volume_usd_30d = vol_usd_30d;
        self
    }

    /// Set trade counts
    pub fn with_trades(mut self, trades_24h: i64, trades_30d: i64) -> Self {
        self.trades_count_24h = trades_24h;
        self.trades_count_30d = trades_30d;
        self
    }

    /// Set open interest
    pub fn with_open_interest(mut self, oi: f64) -> Self {
        self.open_interest = oi;
        self
    }

    /// Calculate average trade size 24h
    pub fn avg_trade_size_24h(&self) -> f64 {
        if self.trades_count_24h > 0 {
            self.volume_24h / self.trades_count_24h as f64
        } else {
            0.0
        }
    }

    /// Calculate average trade size 30d
    pub fn avg_trade_size_30d(&self) -> f64 {
        if self.trades_count_30d > 0 {
            self.volume_30d / self.trades_count_30d as f64
        } else {
            0.0
        }
    }

    /// Calculate volume growth (30d vs 24h annualized)
    pub fn volume_growth_rate(&self) -> f64 {
        let daily_30d = self.volume_30d / 30.0;
        if daily_30d > 0.0 {
            ((self.volume_24h / daily_30d) - 1.0) * 100.0
        } else {
            0.0
        }
    }
}

impl_json_display!(MarketStatistics);
impl_json_debug_pretty!(MarketStatistics);

/// Collection of currency information
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyInfoCollection {
    /// List of currency information
    pub currencies: Vec<CurrencyInfo>,
}

impl CurrencyInfoCollection {
    /// Create new collection
    pub fn new() -> Self {
        Self {
            currencies: Vec::new(),
        }
    }

    /// Add currency info
    pub fn add(&mut self, info: CurrencyInfo) {
        self.currencies.push(info);
    }

    /// Get currency info by currency
    pub fn get(&self, currency: Currency) -> Option<&CurrencyInfo> {
        self.currencies.iter().find(|c| c.currency == currency)
    }

    /// Get enabled currencies
    pub fn enabled(&self) -> Vec<&CurrencyInfo> {
        self.currencies.iter().filter(|c| c.is_enabled()).collect()
    }

    /// Get currencies with withdrawal support
    pub fn with_withdrawal(&self) -> Vec<&CurrencyInfo> {
        self.currencies
            .iter()
            .filter(|c| !c.withdrawal_priorities.is_empty())
            .collect()
    }
}

impl Default for CurrencyInfoCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl_json_display!(CurrencyInfoCollection);
impl_json_debug_pretty!(CurrencyInfoCollection);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_priority() {
        let priority = WithdrawalPriority::very_high();
        assert_eq!(priority.name, "very_high");
        assert_eq!(priority.value, 1.5);
    }

    #[test]
    fn test_currency_info() {
        let mut info = CurrencyInfo::new(
            "BITCOIN".to_string(),
            Currency::Bitcoin,
            "Bitcoin".to_string(),
            4,
            1,
            0.0001,
            0.0005,
        );

        info.add_priority(WithdrawalPriority::low());
        info.add_priority(WithdrawalPriority::high());

        assert!(info.is_enabled());
        assert_eq!(info.withdrawal_priorities.len(), 2);
        assert!(info.get_priority("low").is_some());
        assert_eq!(info.highest_priority().unwrap().name, "high");
        assert_eq!(info.lowest_priority().unwrap().name, "low");
    }

    #[test]
    fn test_index_price() {
        let index =
            IndexPrice::new(45000.0, 44950.0, 1640995200000).with_name("BTC-USD".to_string());

        assert_eq!(index.price_difference(), 50.0);
        assert!((index.price_difference_percentage() - 0.1112).abs() < 0.001);
    }

    #[test]
    fn test_funding_rate() {
        let funding = FundingRate::new(1640995200000, "BTC-PERPETUAL".to_string(), 0.0001, 0.0008)
            .with_current_funding(0.0002);

        assert!(funding.is_positive());
        assert!(!funding.is_negative());
        assert_eq!(funding.annualized_rate(), 0.0001 * 365.0 * 3.0);
    }

    #[test]
    fn test_historical_volatility() {
        let vol = HistoricalVolatility::new(1640995200000, 80.0, 30, "BTC".to_string());

        assert_eq!(vol.as_decimal(), 0.8);
        let annualized = vol.annualized();
        assert!((annualized - 80.0 * (365.0f64 / 30.0f64).sqrt()).abs() < 0.001);
    }

    #[test]
    fn test_market_statistics() {
        let stats = MarketStatistics::new(Currency::Bitcoin, 1640995200000)
            .with_volume(1000.0, 30000.0, 45000000.0, 1350000000.0)
            .with_trades(500, 15000)
            .with_open_interest(5000000.0);

        assert_eq!(stats.avg_trade_size_24h(), 2.0);
        assert_eq!(stats.avg_trade_size_30d(), 2.0);

        let growth = stats.volume_growth_rate();
        assert!(growth.abs() < 0.001); // Should be close to 0% since volumes are proportional
    }

    #[test]
    fn test_currency_info_collection() {
        let mut collection = CurrencyInfoCollection::new();

        let btc_info = CurrencyInfo::new(
            "BITCOIN".to_string(),
            Currency::Bitcoin,
            "Bitcoin".to_string(),
            4,
            1,
            0.0001,
            0.0005,
        );

        collection.add(btc_info);

        assert_eq!(collection.currencies.len(), 1);
        assert!(collection.get(Currency::Bitcoin).is_some());
        assert_eq!(collection.enabled().len(), 1);
    }

    #[test]
    fn test_serde() {
        let info = CurrencyInfo::new(
            "BITCOIN".to_string(),
            Currency::Bitcoin,
            "Bitcoin".to_string(),
            4,
            1,
            0.0001,
            0.0005,
        );

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: CurrencyInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }
}
