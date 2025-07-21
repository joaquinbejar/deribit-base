/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::currency::Currency;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name (e.g., "BTC-PERPETUAL", "ETH-25JUL25-3000-C")
    pub instrument_name: String,
    /// Instrument kind
    pub kind: InstrumentKind,
    /// Base currency
    pub currency: Currency,
    /// Whether the instrument is active for trading
    pub is_active: bool,
    /// Expiration timestamp (None for perpetuals)
    pub expiration_timestamp: Option<i64>,
    /// Strike price (for options)
    pub strike: Option<f64>,
    /// Option type (call/put, for options only)
    pub option_type: Option<OptionType>,
    /// Minimum price movement
    pub tick_size: f64,
    /// Minimum trade amount
    pub min_trade_amount: f64,
    /// Contract size
    pub contract_size: f64,
    /// Settlement period
    pub settlement_period: Option<String>,
    /// Instrument type (linear/reversed)
    pub instrument_type: Option<InstrumentType>,
    /// Quote currency
    pub quote_currency: Option<Currency>,
    /// Settlement currency
    pub settlement_currency: Option<Currency>,
    /// Creation timestamp
    pub creation_timestamp: Option<i64>,
    /// Maximum leverage
    pub max_leverage: Option<f64>,
    /// Maker commission rate
    pub maker_commission: Option<f64>,
    /// Taker commission rate
    pub taker_commission: Option<f64>,
}

impl Instrument {
    /// Check if the instrument is a perpetual contract
    pub fn is_perpetual(&self) -> bool {
        self.expiration_timestamp.is_none() && self.kind == InstrumentKind::Future
    }

    /// Check if the instrument is an option
    pub fn is_option(&self) -> bool {
        self.kind == InstrumentKind::Option
    }

    /// Check if the instrument is a future
    pub fn is_future(&self) -> bool {
        matches!(
            self.kind,
            InstrumentKind::Future | InstrumentKind::FutureCombo
        )
    }

    /// Check if the instrument is a spot
    pub fn is_spot(&self) -> bool {
        self.kind == InstrumentKind::Spot
    }

    /// Get the base currency as string
    pub fn currency_str(&self) -> &'static str {
        self.currency.as_str()
    }
}
