/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Wallet-related data structures and types
//!
//! This module contains types for Deribit wallet operations including
//! deposit addresses, withdrawals, and clearance/compliance information.

use crate::model::transfer::AddressType;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Withdrawal state enumeration
///
/// Represents the current state of a withdrawal request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalState {
    /// Withdrawal is unconfirmed (awaiting email confirmation)
    #[default]
    Unconfirmed,
    /// Withdrawal has been confirmed by user
    Confirmed,
    /// Withdrawal has been cancelled
    Cancelled,
    /// Withdrawal has been completed successfully
    Completed,
    /// Withdrawal was interrupted
    Interrupted,
    /// Withdrawal was rejected
    Rejected,
}

impl WithdrawalState {
    /// Get the string representation for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unconfirmed => "unconfirmed",
            Self::Confirmed => "confirmed",
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Interrupted => "interrupted",
            Self::Rejected => "rejected",
        }
    }

    /// Check if the withdrawal is in a terminal state
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Cancelled | Self::Completed | Self::Interrupted | Self::Rejected
        )
    }

    /// Check if the withdrawal was successful
    #[must_use]
    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Completed)
    }
}

impl std::fmt::Display for WithdrawalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Withdrawal priority for BTC transactions
///
/// Higher priority results in faster confirmation but higher fees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalPriority {
    /// Insane priority - fastest, highest fee
    Insane,
    /// Extreme high priority
    ExtremeHigh,
    /// Very high priority
    VeryHigh,
    /// High priority (default)
    #[default]
    High,
    /// Medium priority
    Mid,
    /// Low priority
    Low,
    /// Very low priority - slowest, lowest fee
    VeryLow,
}

impl WithdrawalPriority {
    /// Get the string representation for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Insane => "insane",
            Self::ExtremeHigh => "extreme_high",
            Self::VeryHigh => "very_high",
            Self::High => "high",
            Self::Mid => "mid",
            Self::Low => "low",
            Self::VeryLow => "very_low",
        }
    }
}

impl std::fmt::Display for WithdrawalPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Deposit address information
///
/// Represents a deposit address for a specific currency,
/// returned by `/private/get_current_deposit_address` or `/private/create_deposit_address`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Cryptocurrency address in proper format
    pub address: String,
    /// Currency code (e.g., "BTC", "ETH")
    pub currency: String,
    /// Address type (deposit/withdrawal/transfer)
    #[serde(rename = "type")]
    pub address_type: AddressType,
    /// Creation timestamp in milliseconds since Unix epoch
    pub creation_timestamp: i64,
}

impl DepositAddress {
    /// Create a new deposit address
    #[must_use]
    pub fn new(address: String, currency: String, creation_timestamp: i64) -> Self {
        Self {
            address,
            currency,
            address_type: AddressType::Deposit,
            creation_timestamp,
        }
    }
}

/// Withdrawal request
///
/// Used to submit a withdrawal via `/private/withdraw`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    /// Currency to withdraw (e.g., "BTC", "ETH")
    pub currency: String,
    /// Destination address (must be in address book)
    pub address: String,
    /// Amount to withdraw
    pub amount: f64,
    /// Withdrawal priority (optional, BTC only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<WithdrawalPriority>,
}

impl WithdrawalRequest {
    /// Create a new withdrawal request
    #[must_use]
    pub fn new(currency: String, address: String, amount: f64) -> Self {
        Self {
            currency,
            address,
            amount,
            priority: None,
        }
    }

    /// Set withdrawal priority (BTC only)
    #[must_use]
    pub fn with_priority(mut self, priority: WithdrawalPriority) -> Self {
        self.priority = Some(priority);
        self
    }
}

/// Deposit identification for clearance operations
///
/// Used to identify a specific deposit when setting clearance originator
/// via `/private/set_clearance_originator`.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositId {
    /// Currency code
    pub currency: String,
    /// User/subaccount ID
    pub user_id: i64,
    /// Deposit address
    pub address: String,
    /// Blockchain transaction hash
    pub tx_hash: String,
}

impl DepositId {
    /// Create a new deposit ID
    #[must_use]
    pub fn new(currency: String, user_id: i64, address: String, tx_hash: String) -> Self {
        Self {
            currency,
            user_id,
            address,
            tx_hash,
        }
    }
}

/// Clearance originator information for compliance
///
/// Contains information about the originator of a deposit
/// for regulatory compliance purposes.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearanceOriginator {
    /// Whether the user is the originator (self-transfer)
    pub is_personal: bool,
    /// First name (if originator is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (if originator is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name (if originator is a legal entity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Geographical address of the originator
    pub address: String,
}

impl ClearanceOriginator {
    /// Create a personal originator (self-transfer)
    #[must_use]
    pub fn personal(address: String) -> Self {
        Self {
            is_personal: true,
            first_name: None,
            last_name: None,
            company_name: None,
            address,
        }
    }

    /// Create an individual originator
    #[must_use]
    pub fn individual(first_name: String, last_name: String, address: String) -> Self {
        Self {
            is_personal: false,
            first_name: Some(first_name),
            last_name: Some(last_name),
            company_name: None,
            address,
        }
    }

    /// Create a company originator
    #[must_use]
    pub fn company(company_name: String, address: String) -> Self {
        Self {
            is_personal: false,
            first_name: None,
            last_name: None,
            company_name: Some(company_name),
            address,
        }
    }

    /// Check if this is a self-transfer
    #[must_use]
    pub fn is_self_transfer(&self) -> bool {
        self.is_personal
    }

    /// Check if originator is an individual
    #[must_use]
    pub fn is_individual(&self) -> bool {
        !self.is_personal && self.first_name.is_some() && self.last_name.is_some()
    }

    /// Check if originator is a company
    #[must_use]
    pub fn is_company(&self) -> bool {
        !self.is_personal && self.company_name.is_some()
    }
}

/// Clearance state for deposits
///
/// Represents the clearance/compliance state of a deposit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceState {
    /// Clearance not required
    #[default]
    NotRequired,
    /// Clearance is in progress
    InProgress,
    /// Clearance has been completed
    Completed,
    /// Clearance failed
    Failed,
}

impl ClearanceState {
    /// Get the string representation
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    /// Check if clearance is complete
    #[must_use]
    pub fn is_cleared(&self) -> bool {
        matches!(self, Self::NotRequired | Self::Completed)
    }
}

impl std::fmt::Display for ClearanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_state_default() {
        let state = WithdrawalState::default();
        assert_eq!(state, WithdrawalState::Unconfirmed);
    }

    #[test]
    fn test_withdrawal_state_as_str() {
        assert_eq!(WithdrawalState::Unconfirmed.as_str(), "unconfirmed");
        assert_eq!(WithdrawalState::Confirmed.as_str(), "confirmed");
        assert_eq!(WithdrawalState::Cancelled.as_str(), "cancelled");
        assert_eq!(WithdrawalState::Completed.as_str(), "completed");
        assert_eq!(WithdrawalState::Interrupted.as_str(), "interrupted");
        assert_eq!(WithdrawalState::Rejected.as_str(), "rejected");
    }

    #[test]
    fn test_withdrawal_state_is_terminal() {
        assert!(!WithdrawalState::Unconfirmed.is_terminal());
        assert!(!WithdrawalState::Confirmed.is_terminal());
        assert!(WithdrawalState::Cancelled.is_terminal());
        assert!(WithdrawalState::Completed.is_terminal());
        assert!(WithdrawalState::Interrupted.is_terminal());
        assert!(WithdrawalState::Rejected.is_terminal());
    }

    #[test]
    fn test_withdrawal_state_is_successful() {
        assert!(!WithdrawalState::Unconfirmed.is_successful());
        assert!(!WithdrawalState::Cancelled.is_successful());
        assert!(WithdrawalState::Completed.is_successful());
    }

    #[test]
    fn test_withdrawal_state_serialization() {
        let state = WithdrawalState::Completed;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"completed\"");

        let deserialized: WithdrawalState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, WithdrawalState::Completed);
    }

    #[test]
    fn test_withdrawal_priority_default() {
        let priority = WithdrawalPriority::default();
        assert_eq!(priority, WithdrawalPriority::High);
    }

    #[test]
    fn test_withdrawal_priority_as_str() {
        assert_eq!(WithdrawalPriority::Insane.as_str(), "insane");
        assert_eq!(WithdrawalPriority::ExtremeHigh.as_str(), "extreme_high");
        assert_eq!(WithdrawalPriority::VeryHigh.as_str(), "very_high");
        assert_eq!(WithdrawalPriority::High.as_str(), "high");
        assert_eq!(WithdrawalPriority::Mid.as_str(), "mid");
        assert_eq!(WithdrawalPriority::Low.as_str(), "low");
        assert_eq!(WithdrawalPriority::VeryLow.as_str(), "very_low");
    }

    #[test]
    fn test_withdrawal_priority_serialization() {
        let priority = WithdrawalPriority::VeryHigh;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"very_high\"");

        let deserialized: WithdrawalPriority = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, WithdrawalPriority::VeryHigh);
    }

    #[test]
    fn test_deposit_address_new() {
        let addr = DepositAddress::new(
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            "BTC".to_string(),
            1640995200000,
        );
        assert_eq!(addr.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(addr.currency, "BTC");
        assert_eq!(addr.address_type, AddressType::Deposit);
        assert_eq!(addr.creation_timestamp, 1640995200000);
    }

    #[test]
    fn test_deposit_address_serialization() {
        let addr = DepositAddress::new(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f".to_string(),
            "ETH".to_string(),
            1640995200000,
        );
        let json = serde_json::to_string(&addr).unwrap();
        let deserialized: DepositAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(addr, deserialized);
    }

    #[test]
    fn test_withdrawal_request_new() {
        let request = WithdrawalRequest::new(
            "BTC".to_string(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            1.5,
        );
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.amount, 1.5);
        assert!(request.priority.is_none());
    }

    #[test]
    fn test_withdrawal_request_with_priority() {
        let request = WithdrawalRequest::new("BTC".to_string(), "address".to_string(), 1.0)
            .with_priority(WithdrawalPriority::VeryHigh);
        assert_eq!(request.priority, Some(WithdrawalPriority::VeryHigh));
    }

    #[test]
    fn test_deposit_id_new() {
        let deposit_id = DepositId::new(
            "BTC".to_string(),
            123,
            "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz".to_string(),
            "230669110fdaf0a0dbcdc079b6b8b43d5af29cc73683835b9bc6b3406c065fda".to_string(),
        );
        assert_eq!(deposit_id.currency, "BTC");
        assert_eq!(deposit_id.user_id, 123);
    }

    #[test]
    fn test_deposit_id_serialization() {
        let deposit_id = DepositId::new(
            "BTC".to_string(),
            123,
            "address".to_string(),
            "tx_hash".to_string(),
        );
        let json = serde_json::to_string(&deposit_id).unwrap();
        let deserialized: DepositId = serde_json::from_str(&json).unwrap();
        assert_eq!(deposit_id, deserialized);
    }

    #[test]
    fn test_clearance_originator_personal() {
        let originator = ClearanceOriginator::personal("123 Main St".to_string());
        assert!(originator.is_personal);
        assert!(originator.is_self_transfer());
        assert!(!originator.is_individual());
        assert!(!originator.is_company());
    }

    #[test]
    fn test_clearance_originator_individual() {
        let originator = ClearanceOriginator::individual(
            "John".to_string(),
            "Doe".to_string(),
            "123 Main St".to_string(),
        );
        assert!(!originator.is_personal);
        assert!(!originator.is_self_transfer());
        assert!(originator.is_individual());
        assert!(!originator.is_company());
        assert_eq!(originator.first_name, Some("John".to_string()));
        assert_eq!(originator.last_name, Some("Doe".to_string()));
    }

    #[test]
    fn test_clearance_originator_company() {
        let originator =
            ClearanceOriginator::company("Acme Corp".to_string(), "456 Business Ave".to_string());
        assert!(!originator.is_personal);
        assert!(!originator.is_self_transfer());
        assert!(!originator.is_individual());
        assert!(originator.is_company());
        assert_eq!(originator.company_name, Some("Acme Corp".to_string()));
    }

    #[test]
    fn test_clearance_originator_serialization() {
        let originator = ClearanceOriginator::individual(
            "John".to_string(),
            "Doe".to_string(),
            "123 Main St".to_string(),
        );
        let json = serde_json::to_string(&originator).unwrap();
        let deserialized: ClearanceOriginator = serde_json::from_str(&json).unwrap();
        assert_eq!(originator, deserialized);
    }

    #[test]
    fn test_clearance_state_default() {
        let state = ClearanceState::default();
        assert_eq!(state, ClearanceState::NotRequired);
    }

    #[test]
    fn test_clearance_state_as_str() {
        assert_eq!(ClearanceState::NotRequired.as_str(), "not_required");
        assert_eq!(ClearanceState::InProgress.as_str(), "in_progress");
        assert_eq!(ClearanceState::Completed.as_str(), "completed");
        assert_eq!(ClearanceState::Failed.as_str(), "failed");
    }

    #[test]
    fn test_clearance_state_is_cleared() {
        assert!(ClearanceState::NotRequired.is_cleared());
        assert!(!ClearanceState::InProgress.is_cleared());
        assert!(ClearanceState::Completed.is_cleared());
        assert!(!ClearanceState::Failed.is_cleared());
    }

    #[test]
    fn test_clearance_state_serialization() {
        let state = ClearanceState::InProgress;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"in_progress\"");

        let deserialized: ClearanceState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ClearanceState::InProgress);
    }
}
