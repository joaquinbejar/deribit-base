/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Combo Books data structures and types
//!
//! This module contains types for Deribit combo instruments,
//! which are multi-leg instruments combining multiple options or futures.
//!
//! Combos allow trading of spread strategies (e.g., call spreads, straddles)
//! as a single instrument with a unified order book.

use crate::model::order::OrderSide;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Combo state enumeration
///
/// Indicates the current state of a combo instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ComboState {
    /// Request for quote state - combo is being requested
    Rfq,
    /// Active state - combo has an active order book
    #[default]
    Active,
    /// Inactive state - combo is no longer tradeable
    Inactive,
}

impl ComboState {
    /// Get the string representation for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rfq => "rfq",
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }

    /// Check if the combo is tradeable
    #[must_use]
    pub fn is_tradeable(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl std::fmt::Display for ComboState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Leg in a combo instrument
///
/// Represents one instrument leg in a combo, with the instrument name
/// and size multiplier. A negative amount indicates the leg trades
/// in the opposite direction to the combo trade.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComboLeg {
    /// Instrument name (e.g., "BTC-29APR22-37500-C")
    pub instrument_name: String,
    /// Size multiplier (negative = opposite direction)
    pub amount: i32,
}

impl ComboLeg {
    /// Create a new combo leg
    #[must_use]
    pub fn new(instrument_name: String, amount: i32) -> Self {
        Self {
            instrument_name,
            amount,
        }
    }

    /// Check if this leg is in the same direction as the combo
    #[must_use]
    pub fn is_same_direction(&self) -> bool {
        self.amount > 0
    }

    /// Check if this leg is in the opposite direction to the combo
    #[must_use]
    pub fn is_opposite_direction(&self) -> bool {
        self.amount < 0
    }

    /// Get the absolute amount multiplier
    #[must_use]
    pub fn abs_amount(&self) -> i32 {
        self.amount.abs()
    }
}

/// Trade leg for combo creation request
///
/// Used when creating a combo via `/private/create_combo`.
/// Specifies the instrument, amount (as string), and direction.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComboTradeLeg {
    /// Instrument name
    pub instrument_name: String,
    /// Amount as string (API requirement)
    pub amount: String,
    /// Trade direction
    pub direction: OrderSide,
}

impl ComboTradeLeg {
    /// Create a new combo trade leg
    #[must_use]
    pub fn new(instrument_name: String, amount: String, direction: OrderSide) -> Self {
        Self {
            instrument_name,
            amount,
            direction,
        }
    }

    /// Create from numeric amount
    #[must_use]
    pub fn from_amount(instrument_name: String, amount: i32, direction: OrderSide) -> Self {
        Self {
            instrument_name,
            amount: amount.to_string(),
            direction,
        }
    }
}

/// Create combo request
///
/// Used to create a new combo or retrieve an existing combo
/// via `/private/create_combo`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateComboRequest {
    /// List of trade legs defining the combo structure
    pub trades: Vec<ComboTradeLeg>,
}

impl CreateComboRequest {
    /// Create a new combo request
    #[must_use]
    pub fn new(trades: Vec<ComboTradeLeg>) -> Self {
        Self { trades }
    }

    /// Get the number of legs in this combo request
    #[must_use]
    pub fn leg_count(&self) -> usize {
        self.trades.len()
    }

    /// Check if this is a valid combo (at least 2 legs)
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.trades.len() >= 2
    }
}

/// Combo details
///
/// Contains full details of a combo instrument,
/// returned by `/public/get_combo_details` or `/public/get_combos`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComboDetails {
    /// Unique combo identifier (e.g., "BTC-FS-29APR22_PERP")
    pub id: String,
    /// Internal instrument ID
    pub instrument_id: i64,
    /// Current combo state
    pub state: ComboState,
    /// Timestamp of last state change in milliseconds
    pub state_timestamp: i64,
    /// Combo creation timestamp in milliseconds
    pub creation_timestamp: i64,
    /// List of instrument legs in the combo
    pub legs: Vec<ComboLeg>,
}

impl ComboDetails {
    /// Get the number of legs in this combo
    #[must_use]
    pub fn leg_count(&self) -> usize {
        self.legs.len()
    }

    /// Check if the combo is currently tradeable
    #[must_use]
    pub fn is_tradeable(&self) -> bool {
        self.state.is_tradeable()
    }

    /// Get all instrument names in this combo
    #[must_use]
    pub fn instruments(&self) -> Vec<&str> {
        self.legs
            .iter()
            .map(|l| l.instrument_name.as_str())
            .collect()
    }

    /// Check if this is a futures spread (contains "FS" in ID)
    #[must_use]
    pub fn is_futures_spread(&self) -> bool {
        self.id.contains("-FS-")
    }

    /// Check if this is a call spread (contains "CS" in ID)
    #[must_use]
    pub fn is_call_spread(&self) -> bool {
        self.id.contains("-CS-")
    }

    /// Check if this is a put spread (contains "PS" in ID)
    #[must_use]
    pub fn is_put_spread(&self) -> bool {
        self.id.contains("-PS-")
    }

    /// Check if this is a reversal (contains "REV" in ID)
    #[must_use]
    pub fn is_reversal(&self) -> bool {
        self.id.contains("-REV-")
    }
}

/// List of combo IDs response
///
/// Simple wrapper for the list of combo IDs returned by `/public/get_combo_ids`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComboIds {
    /// List of combo identifiers
    pub ids: Vec<String>,
}

impl ComboIds {
    /// Create a new combo IDs list
    #[must_use]
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }

    /// Get the number of combos
    #[must_use]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Check if the list is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Check if a specific combo ID exists
    #[must_use]
    pub fn contains(&self, combo_id: &str) -> bool {
        self.ids.iter().any(|id| id == combo_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo_state_default() {
        let state = ComboState::default();
        assert_eq!(state, ComboState::Active);
    }

    #[test]
    fn test_combo_state_as_str() {
        assert_eq!(ComboState::Rfq.as_str(), "rfq");
        assert_eq!(ComboState::Active.as_str(), "active");
        assert_eq!(ComboState::Inactive.as_str(), "inactive");
    }

    #[test]
    fn test_combo_state_is_tradeable() {
        assert!(!ComboState::Rfq.is_tradeable());
        assert!(ComboState::Active.is_tradeable());
        assert!(!ComboState::Inactive.is_tradeable());
    }

    #[test]
    fn test_combo_state_display() {
        assert_eq!(format!("{}", ComboState::Rfq), "rfq");
        assert_eq!(format!("{}", ComboState::Active), "active");
        assert_eq!(format!("{}", ComboState::Inactive), "inactive");
    }

    #[test]
    fn test_combo_state_serialization() {
        let state = ComboState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"active\"");

        let deserialized: ComboState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ComboState::Active);
    }

    #[test]
    fn test_combo_leg_new() {
        let leg = ComboLeg::new("BTC-PERPETUAL".to_string(), -1);
        assert_eq!(leg.instrument_name, "BTC-PERPETUAL");
        assert_eq!(leg.amount, -1);
    }

    #[test]
    fn test_combo_leg_direction() {
        let positive_leg = ComboLeg::new("BTC-29APR22".to_string(), 1);
        assert!(positive_leg.is_same_direction());
        assert!(!positive_leg.is_opposite_direction());
        assert_eq!(positive_leg.abs_amount(), 1);

        let negative_leg = ComboLeg::new("BTC-PERPETUAL".to_string(), -1);
        assert!(!negative_leg.is_same_direction());
        assert!(negative_leg.is_opposite_direction());
        assert_eq!(negative_leg.abs_amount(), 1);
    }

    #[test]
    fn test_combo_leg_serialization() {
        let leg = ComboLeg::new("BTC-PERPETUAL".to_string(), -1);
        let json = serde_json::to_string(&leg).unwrap();
        let deserialized: ComboLeg = serde_json::from_str(&json).unwrap();
        assert_eq!(leg, deserialized);
    }

    #[test]
    fn test_combo_trade_leg_new() {
        let leg = ComboTradeLeg::new(
            "BTC-29APR22-37500-C".to_string(),
            "1".to_string(),
            OrderSide::Buy,
        );
        assert_eq!(leg.instrument_name, "BTC-29APR22-37500-C");
        assert_eq!(leg.amount, "1");
        assert_eq!(leg.direction, OrderSide::Buy);
    }

    #[test]
    fn test_combo_trade_leg_from_amount() {
        let leg = ComboTradeLeg::from_amount("BTC-29APR22-37500-C".to_string(), 1, OrderSide::Buy);
        assert_eq!(leg.amount, "1");
    }

    #[test]
    fn test_combo_trade_leg_serialization() {
        let leg = ComboTradeLeg::new(
            "BTC-29APR22-37500-C".to_string(),
            "1".to_string(),
            OrderSide::Buy,
        );
        let json = serde_json::to_string(&leg).unwrap();
        let deserialized: ComboTradeLeg = serde_json::from_str(&json).unwrap();
        assert_eq!(leg, deserialized);
    }

    #[test]
    fn test_create_combo_request_new() {
        let trades = vec![
            ComboTradeLeg::new(
                "BTC-29APR22-37500-C".to_string(),
                "1".to_string(),
                OrderSide::Buy,
            ),
            ComboTradeLeg::new(
                "BTC-29APR22-37500-P".to_string(),
                "1".to_string(),
                OrderSide::Sell,
            ),
        ];
        let request = CreateComboRequest::new(trades);
        assert_eq!(request.leg_count(), 2);
        assert!(request.is_valid());
    }

    #[test]
    fn test_create_combo_request_invalid() {
        let request = CreateComboRequest::new(vec![ComboTradeLeg::new(
            "BTC-29APR22-37500-C".to_string(),
            "1".to_string(),
            OrderSide::Buy,
        )]);
        assert!(!request.is_valid());
    }

    fn create_test_combo_details() -> ComboDetails {
        ComboDetails {
            id: "BTC-FS-29APR22_PERP".to_string(),
            instrument_id: 27,
            state: ComboState::Active,
            state_timestamp: 1650620605150,
            creation_timestamp: 1650620575000,
            legs: vec![
                ComboLeg::new("BTC-PERPETUAL".to_string(), -1),
                ComboLeg::new("BTC-29APR22".to_string(), 1),
            ],
        }
    }

    #[test]
    fn test_combo_details_leg_count() {
        let combo = create_test_combo_details();
        assert_eq!(combo.leg_count(), 2);
    }

    #[test]
    fn test_combo_details_is_tradeable() {
        let active_combo = create_test_combo_details();
        assert!(active_combo.is_tradeable());

        let mut inactive_combo = create_test_combo_details();
        inactive_combo.state = ComboState::Inactive;
        assert!(!inactive_combo.is_tradeable());
    }

    #[test]
    fn test_combo_details_instruments() {
        let combo = create_test_combo_details();
        let instruments = combo.instruments();
        assert_eq!(instruments.len(), 2);
        assert!(instruments.contains(&"BTC-PERPETUAL"));
        assert!(instruments.contains(&"BTC-29APR22"));
    }

    #[test]
    fn test_combo_details_type_detection() {
        let futures_spread = create_test_combo_details();
        assert!(futures_spread.is_futures_spread());
        assert!(!futures_spread.is_call_spread());
        assert!(!futures_spread.is_put_spread());
        assert!(!futures_spread.is_reversal());

        let mut call_spread = create_test_combo_details();
        call_spread.id = "BTC-CS-29APR22-39300_39600".to_string();
        assert!(call_spread.is_call_spread());

        let mut reversal = create_test_combo_details();
        reversal.id = "BTC-REV-29APR22-37500".to_string();
        assert!(reversal.is_reversal());
    }

    #[test]
    fn test_combo_details_serialization() {
        let combo = create_test_combo_details();
        let json = serde_json::to_string(&combo).unwrap();
        let deserialized: ComboDetails = serde_json::from_str(&json).unwrap();
        assert_eq!(combo.id, deserialized.id);
        assert_eq!(combo.state, deserialized.state);
        assert_eq!(combo.legs.len(), deserialized.legs.len());
    }

    #[test]
    fn test_combo_ids_new() {
        let ids = ComboIds::new(vec![
            "BTC-CS-29APR22-39300_39600".to_string(),
            "BTC-FS-29APR22_PERP".to_string(),
        ]);
        assert_eq!(ids.len(), 2);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_combo_ids_contains() {
        let ids = ComboIds::new(vec![
            "BTC-CS-29APR22-39300_39600".to_string(),
            "BTC-FS-29APR22_PERP".to_string(),
        ]);
        assert!(ids.contains("BTC-FS-29APR22_PERP"));
        assert!(!ids.contains("ETH-FS-29APR22_PERP"));
    }

    #[test]
    fn test_combo_ids_empty() {
        let ids = ComboIds::new(vec![]);
        assert!(ids.is_empty());
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_combo_ids_serialization() {
        let ids = ComboIds::new(vec!["BTC-FS-29APR22_PERP".to_string()]);
        let json = serde_json::to_string(&ids).unwrap();
        let deserialized: ComboIds = serde_json::from_str(&json).unwrap();
        assert_eq!(ids, deserialized);
    }
}
