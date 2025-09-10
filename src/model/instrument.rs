/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Instrument kind enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentKind {
    /// Future contract
    Future,
    /// Option contract
    Option,
    /// Spot trading
    Spot,
    /// Future combo
    #[serde(rename = "future_combo")]
    FutureCombo,
    /// Option combo
    #[serde(rename = "option_combo")]
    OptionCombo,
}

/// Option type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    /// Call option
    Call,
    /// Put option
    Put,
}

/// Instrument type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentType {
    /// Linear instrument
    Linear,
    /// Reversed instrument
    Reversed,
}

/// Instrument information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name (e.g., "BTC-PERPETUAL", "ETH-25JUL25-3000-C")
    pub instrument_name: String,
    /// Price index used for mark price calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_index: Option<String>,
    /// Instrument kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,
    /// Base currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Whether the instrument is active for trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Expiration timestamp (None for perpetuals)
    pub expiration_timestamp: Option<i64>,
    /// Strike price (for options)
    pub strike: Option<f64>,
    /// Option type (call/put, for options only)
    pub option_type: Option<OptionType>,
    /// Minimum price movement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<f64>,
    /// Minimum trade amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_trade_amount: Option<f64>,
    /// Contract size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_size: Option<f64>,
    /// Settlement period
    pub settlement_period: Option<String>,
    /// Instrument type (linear/reversed)
    pub instrument_type: Option<InstrumentType>,
    /// Quote currency
    pub quote_currency: Option<String>,
    /// Settlement currency
    pub settlement_currency: Option<String>,
    /// Creation timestamp
    pub creation_timestamp: Option<i64>,
    /// Maximum leverage
    pub max_leverage: Option<f64>,
    /// Maker commission rate
    pub maker_commission: Option<f64>,
    /// Taker commission rate
    pub taker_commission: Option<f64>,
    /// Unique instrument identifier
    pub instrument_id: Option<u32>,
    /// Base currency for the instrument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
    /// Counter currency for the instrument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_currency: Option<String>,
}

impl Instrument {
    /// Check if the instrument is a perpetual contract
    pub fn is_perpetual(&self) -> bool {
        self.expiration_timestamp.is_none()
            && self
                .kind
                .as_ref()
                .is_some_and(|k| matches!(k, InstrumentKind::Future))
    }

    /// Check if the instrument is an option
    pub fn is_option(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Option | InstrumentKind::OptionCombo))
    }

    /// Check if the instrument is a future
    pub fn is_future(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Future | InstrumentKind::FutureCombo))
    }

    /// Check if the instrument is a spot
    pub fn is_spot(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Spot))
    }
}

/// Index data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct IndexData {
    /// BTC component (optional)
    pub btc: Option<f64>,
    /// ETH component (optional)
    pub eth: Option<f64>,
    /// USDC component (optional)
    pub usdc: Option<f64>,
    /// USDT component (optional)
    pub usdt: Option<f64>,
    /// EURR component (optional)
    pub eurr: Option<f64>,
    /// EDP (Estimated Delivery Price)
    pub edp: f64,
}

/// Index price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct IndexPriceData {
    /// Current index price
    pub index_price: f64,
    /// Estimated delivery price
    pub estimated_delivery_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_instrument() -> Instrument {
        Instrument {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price_index: Some("btc_usd".to_string()),
            kind: Some(InstrumentKind::Future),
            currency: Some("BTC".to_string()),
            is_active: Some(true),
            expiration_timestamp: None, // Perpetual
            strike: None,
            option_type: None,
            tick_size: Some(0.5),
            min_trade_amount: Some(10.0),
            contract_size: Some(1.0),
            settlement_period: Some("perpetual".to_string()),
            instrument_type: Some(InstrumentType::Linear),
            quote_currency: Some("USD".to_string()),
            settlement_currency: Some("BTC".to_string()),
            creation_timestamp: Some(1640995200000),
            max_leverage: Some(100.0),
            maker_commission: Some(0.0001),
            taker_commission: Some(0.0005),
            instrument_id: Some(12345),
            base_currency: Some("BTC".to_string()),
            counter_currency: Some("USD".to_string()),
        }
    }

    fn create_test_option() -> Instrument {
        Instrument {
            instrument_name: "BTC-25DEC25-50000-C".to_string(),
            price_index: Some("btc_usd".to_string()),
            kind: Some(InstrumentKind::Option),
            currency: Some("BTC".to_string()),
            is_active: Some(true),
            expiration_timestamp: Some(1735084800000),
            strike: Some(50000.0),
            option_type: Some(OptionType::Call),
            tick_size: Some(0.0005),
            min_trade_amount: Some(0.1),
            contract_size: Some(1.0),
            settlement_period: Some("week".to_string()),
            instrument_type: Some(InstrumentType::Linear),
            quote_currency: Some("USD".to_string()),
            settlement_currency: Some("BTC".to_string()),
            creation_timestamp: Some(1640995200000),
            max_leverage: Some(10.0),
            maker_commission: Some(0.0003),
            taker_commission: Some(0.0003),
            instrument_id: Some(67890),
            base_currency: Some("BTC".to_string()),
            counter_currency: Some("USD".to_string()),
        }
    }

    #[test]
    fn test_instrument_is_perpetual() {
        let perpetual = create_test_instrument();
        assert!(perpetual.is_perpetual());

        let option = create_test_option();
        assert!(!option.is_perpetual());

        let mut future_with_expiry = create_test_instrument();
        future_with_expiry.expiration_timestamp = Some(1735084800000);
        assert!(!future_with_expiry.is_perpetual());
    }

    #[test]
    fn test_instrument_is_option() {
        let option = create_test_option();
        assert!(option.is_option());

        let perpetual = create_test_instrument();
        assert!(!perpetual.is_option());

        let mut option_combo = create_test_option();
        option_combo.kind = Some(InstrumentKind::OptionCombo);
        assert!(option_combo.is_option());
    }

    #[test]
    fn test_instrument_is_future() {
        let future = create_test_instrument();
        assert!(future.is_future());

        let option = create_test_option();
        assert!(!option.is_future());

        let mut future_combo = create_test_instrument();
        future_combo.kind = Some(InstrumentKind::FutureCombo);
        assert!(future_combo.is_future());
    }

    #[test]
    fn test_instrument_is_spot() {
        let mut spot = create_test_instrument();
        spot.kind = Some(InstrumentKind::Spot);
        assert!(spot.is_spot());

        let future = create_test_instrument();
        assert!(!future.is_spot());

        let option = create_test_option();
        assert!(!option.is_spot());
    }

    #[test]
    fn test_instrument_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&InstrumentKind::Future).unwrap(),
            "\"future\""
        );
        assert_eq!(
            serde_json::to_string(&InstrumentKind::Option).unwrap(),
            "\"option\""
        );
        assert_eq!(
            serde_json::to_string(&InstrumentKind::Spot).unwrap(),
            "\"spot\""
        );
        assert_eq!(
            serde_json::to_string(&InstrumentKind::FutureCombo).unwrap(),
            "\"future_combo\""
        );
        assert_eq!(
            serde_json::to_string(&InstrumentKind::OptionCombo).unwrap(),
            "\"option_combo\""
        );
    }

    #[test]
    fn test_option_type_serialization() {
        assert_eq!(
            serde_json::to_string(&OptionType::Call).unwrap(),
            "\"call\""
        );
        assert_eq!(serde_json::to_string(&OptionType::Put).unwrap(), "\"put\"");
    }

    #[test]
    fn test_instrument_type_serialization() {
        assert_eq!(
            serde_json::to_string(&InstrumentType::Linear).unwrap(),
            "\"linear\""
        );
        assert_eq!(
            serde_json::to_string(&InstrumentType::Reversed).unwrap(),
            "\"reversed\""
        );
    }

    #[test]
    fn test_instrument_serialization() {
        let instrument = create_test_instrument();
        let json = serde_json::to_string(&instrument).unwrap();
        let deserialized: Instrument = serde_json::from_str(&json).unwrap();
        assert_eq!(instrument.instrument_name, deserialized.instrument_name);
        assert_eq!(instrument.kind, deserialized.kind);
    }

    #[test]
    fn test_index_data_creation() {
        let index_data = IndexData {
            btc: Some(0.5),
            eth: Some(0.3),
            usdc: Some(0.1),
            usdt: Some(0.05),
            eurr: Some(0.05),
            edp: 50000.0,
        };

        assert_eq!(index_data.btc, Some(0.5));
        assert_eq!(index_data.edp, 50000.0);
    }

    #[test]
    fn test_index_price_data_creation() {
        let index_price_data = IndexPriceData {
            index_price: 50000.0,
            estimated_delivery_price: 50100.0,
        };

        assert_eq!(index_price_data.index_price, 50000.0);
        assert_eq!(index_price_data.estimated_delivery_price, 50100.0);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let instrument = create_test_instrument();
        let debug_str = format!("{:?}", instrument);
        let display_str = format!("{}", instrument);

        assert!(debug_str.contains("BTC-PERPETUAL"));
        assert!(display_str.contains("BTC-PERPETUAL"));
    }
}
