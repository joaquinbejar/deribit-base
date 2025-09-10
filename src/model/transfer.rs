/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Transfer state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferState {
    /// Transfer is prepared but not yet confirmed
    Prepared,
    /// Transfer has been confirmed
    Confirmed,
    /// Transfer has been cancelled
    Cancelled,
    /// Transfer is waiting for admin approval
    WaitingForAdmin,
    /// Transfer failed due to insufficient funds
    InsufficientFunds,
    /// Transfer failed due to withdrawal limit
    WithdrawalLimit,
}

impl Default for TransferState {
    fn default() -> Self {
        Self::Prepared
    }
}

/// Address type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    /// Deposit address
    Deposit,
    /// Withdrawal address
    Withdrawal,
    /// Transfer address
    Transfer,
}

impl Default for AddressType {
    fn default() -> Self {
        Self::Deposit
    }
}

/// Transfer information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// Transfer ID
    pub id: i64,
    /// Currency being transferred
    pub currency: String,
    /// Transfer amount
    pub amount: f64,
    /// Transfer fee
    pub fee: f64,
    /// Destination address
    pub address: String,
    /// Blockchain transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Current transfer state
    pub state: TransferState,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub created_timestamp: i64,
    /// Last update timestamp (milliseconds since Unix epoch)
    pub updated_timestamp: i64,
    /// Confirmation timestamp (milliseconds since Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_timestamp: Option<i64>,
    /// Transfer type description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

impl Transfer {
    /// Create a new transfer
    pub fn new(
        id: i64,
        currency: String,
        amount: f64,
        fee: f64,
        address: String,
        created_timestamp: i64,
    ) -> Self {
        Self {
            id,
            currency,
            amount,
            fee,
            address,
            transaction_id: None,
            state: TransferState::Prepared,
            created_timestamp,
            updated_timestamp: created_timestamp,
            confirmed_timestamp: None,
            transfer_type: None,
        }
    }

    /// Set transaction ID
    pub fn with_transaction_id(mut self, tx_id: String) -> Self {
        self.transaction_id = Some(tx_id);
        self
    }

    /// Set transfer state
    pub fn with_state(mut self, state: TransferState) -> Self {
        self.state = state;
        self
    }

    /// Set transfer type
    pub fn with_type(mut self, transfer_type: String) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }

    /// Confirm the transfer
    pub fn confirm(&mut self, timestamp: i64) {
        self.state = TransferState::Confirmed;
        self.confirmed_timestamp = Some(timestamp);
        self.updated_timestamp = timestamp;
    }

    /// Cancel the transfer
    pub fn cancel(&mut self, timestamp: i64) {
        self.state = TransferState::Cancelled;
        self.updated_timestamp = timestamp;
    }

    /// Check if transfer is confirmed
    pub fn is_confirmed(&self) -> bool {
        matches!(self.state, TransferState::Confirmed)
    }

    /// Check if transfer is cancelled
    pub fn is_cancelled(&self) -> bool {
        matches!(self.state, TransferState::Cancelled)
    }

    /// Check if transfer is pending
    pub fn is_pending(&self) -> bool {
        matches!(
            self.state,
            TransferState::Prepared | TransferState::WaitingForAdmin
        )
    }

    /// Get net amount (amount - fee)
    pub fn net_amount(&self) -> f64 {
        self.amount - self.fee
    }
}

/// Address book entry
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBookEntry {
    /// Cryptocurrency address
    pub address: String,
    /// Currency for this address
    pub currency: String,
    /// User-defined label for the address
    pub label: String,
    /// Type of address
    #[serde(rename = "type")]
    pub address_type: AddressType,
    /// Whether this address requires email confirmation for withdrawals
    pub requires_confirmation: bool,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub creation_timestamp: i64,
    /// Whether this is a personal address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    /// Beneficiary information for compliance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_first_name: Option<String>,
    /// Beneficiary last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_last_name: Option<String>,
    /// Beneficiary address for compliance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_address: Option<String>,
    /// Beneficiary VASP DID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_did: Option<String>,
    /// Beneficiary VASP name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_name: Option<String>,
}

impl AddressBookEntry {
    /// Create a new address book entry
    pub fn new(
        address: String,
        currency: String,
        label: String,
        address_type: AddressType,
        creation_timestamp: i64,
    ) -> Self {
        Self {
            address,
            currency,
            label,
            address_type,
            requires_confirmation: false,
            creation_timestamp,
            personal: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_address: None,
            beneficiary_vasp_did: None,
            beneficiary_vasp_name: None,
        }
    }

    /// Set confirmation requirement
    pub fn with_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }

    /// Set personal flag
    pub fn with_personal(mut self, personal: bool) -> Self {
        self.personal = Some(personal);
        self
    }

    /// Set beneficiary information
    pub fn with_beneficiary(
        mut self,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Self {
        self.beneficiary_first_name = Some(first_name);
        self.beneficiary_last_name = Some(last_name);
        self.beneficiary_address = Some(address);
        self
    }

    /// Set VASP information
    pub fn with_vasp(mut self, vasp_did: String, vasp_name: String) -> Self {
        self.beneficiary_vasp_did = Some(vasp_did);
        self.beneficiary_vasp_name = Some(vasp_name);
        self
    }

    /// Check if this is a withdrawal address
    pub fn is_withdrawal(&self) -> bool {
        matches!(self.address_type, AddressType::Withdrawal)
    }

    /// Check if this is a deposit address
    pub fn is_deposit(&self) -> bool {
        matches!(self.address_type, AddressType::Deposit)
    }

    /// Check if this is a transfer address
    pub fn is_transfer(&self) -> bool {
        matches!(self.address_type, AddressType::Transfer)
    }
}

/// Subaccount transfer information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubaccountTransfer {
    /// Transfer amount
    pub amount: f64,
    /// Currency being transferred
    pub currency: String,
    /// Destination subaccount ID
    pub destination: i64,
    /// Transfer ID
    pub id: i64,
    /// Source subaccount ID
    pub source: i64,
    /// Transfer state
    pub state: TransferState,
    /// Transfer timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Type of transfer
    pub transfer_type: String,
}

impl SubaccountTransfer {
    /// Create a new subaccount transfer
    pub fn new(
        id: i64,
        amount: f64,
        currency: String,
        source: i64,
        destination: i64,
        timestamp: i64,
    ) -> Self {
        Self {
            amount,
            currency,
            destination,
            id,
            source,
            state: TransferState::Prepared,
            timestamp,
            transfer_type: "subaccount".to_string(),
        }
    }

    /// Set transfer state
    pub fn with_state(mut self, state: TransferState) -> Self {
        self.state = state;
        self
    }

    /// Set transfer type
    pub fn with_type(mut self, transfer_type: String) -> Self {
        self.transfer_type = transfer_type;
        self
    }

    /// Check if transfer is between main account and subaccount
    pub fn is_main_subaccount_transfer(&self) -> bool {
        self.source == 0 || self.destination == 0
    }

    /// Check if transfer is between subaccounts
    pub fn is_subaccount_to_subaccount(&self) -> bool {
        self.source != 0 && self.destination != 0
    }
}

/// Collection of transfers
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfers {
    /// List of transfers
    pub transfers: Vec<Transfer>,
}

impl Transfers {
    /// Create a new transfers collection
    pub fn new() -> Self {
        Self {
            transfers: Vec::new(),
        }
    }

    /// Add a transfer
    pub fn add(&mut self, transfer: Transfer) {
        self.transfers.push(transfer);
    }

    /// Get transfers by currency
    pub fn by_currency(&self, currency: String) -> Vec<&Transfer> {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .collect()
    }

    /// Get transfers by state
    pub fn by_state(&self, state: TransferState) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.state == state).collect()
    }

    /// Get pending transfers
    pub fn pending(&self) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.is_pending()).collect()
    }

    /// Get confirmed transfers
    pub fn confirmed(&self) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.is_confirmed()).collect()
    }

    /// Calculate total amount by currency
    pub fn total_amount(&self, currency: String) -> f64 {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .map(|t| t.amount)
            .sum()
    }

    /// Calculate total fees by currency
    pub fn total_fees(&self, currency: String) -> f64 {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .map(|t| t.fee)
            .sum()
    }
}

impl Default for Transfers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_creation() {
        let transfer = Transfer::new(
            12345,
            "BTC".to_string(),
            1.0,
            0.0005,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            1640995200000,
        );

        assert_eq!(transfer.id, 12345);
        assert_eq!(transfer.currency, "BTC");
        assert_eq!(transfer.amount, 1.0);
        assert_eq!(transfer.fee, 0.0005);
        assert_eq!(transfer.net_amount(), 0.9995);
        assert!(transfer.is_pending());
    }

    #[test]
    fn test_transfer_state_changes() {
        let mut transfer = Transfer::new(
            1,
            "BTC".to_string(),
            1.0,
            0.001,
            "address".to_string(),
            1000,
        );

        assert!(transfer.is_pending());
        assert!(!transfer.is_confirmed());

        transfer.confirm(2000);
        assert!(transfer.is_confirmed());
        assert!(!transfer.is_pending());
        assert_eq!(transfer.confirmed_timestamp, Some(2000));

        transfer.cancel(3000);
        assert!(transfer.is_cancelled());
        assert_eq!(transfer.updated_timestamp, 3000);
    }

    #[test]
    fn test_address_book_entry() {
        let entry = AddressBookEntry::new(
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            "BTC".to_string(),
            "Main wallet".to_string(),
            AddressType::Withdrawal,
            1640995200000,
        )
        .with_confirmation(true)
        .with_personal(false)
        .with_beneficiary(
            "John".to_string(),
            "Doe".to_string(),
            "123 Main St".to_string(),
        );

        assert!(entry.is_withdrawal());
        assert!(!entry.is_deposit());
        assert!(entry.requires_confirmation);
        assert_eq!(entry.beneficiary_first_name, Some("John".to_string()));
    }

    #[test]
    fn test_subaccount_transfer() {
        let transfer = SubaccountTransfer::new(
            1,
            100.0,
            "BTC".to_string(),
            0,   // main account
            123, // subaccount
            1640995200000,
        );

        assert!(transfer.is_main_subaccount_transfer());
        assert!(!transfer.is_subaccount_to_subaccount());
    }

    #[test]
    fn test_transfers_collection() {
        let mut transfers = Transfers::new();

        transfers.add(
            Transfer::new(1, "BTC".to_string(), 1.0, 0.001, "addr1".to_string(), 1000)
                .with_state(TransferState::Confirmed),
        );

        transfers.add(Transfer::new(
            2,
            "BTC".to_string(),
            0.5,
            0.0005,
            "addr2".to_string(),
            2000,
        ));

        assert_eq!(transfers.transfers.len(), 2);
        assert_eq!(transfers.by_currency("BTC".to_string()).len(), 2);
        assert_eq!(transfers.confirmed().len(), 1);
        assert_eq!(transfers.pending().len(), 1);
        assert_eq!(transfers.total_amount("BTC".to_string()), 1.5);
        assert_eq!(transfers.total_fees("BTC".to_string()), 0.0015);
    }

    #[test]
    fn test_serde() {
        let transfer = Transfer::new(
            12345,
            "BTC".to_string(),
            1.0,
            0.0005,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            1640995200000,
        )
        .with_transaction_id("tx123".to_string())
        .with_state(TransferState::Confirmed);

        let json = serde_json::to_string(&transfer).unwrap();
        let deserialized: Transfer = serde_json::from_str(&json).unwrap();
        assert_eq!(transfer, deserialized);
    }
}
