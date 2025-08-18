/****************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Transaction type enumeration
#[derive(Clone, Serialize, Deserialize, Default)]
pub enum TransactionType {
    /// Deposit transaction
    Deposit,
    /// Withdrawal transaction
    Withdrawal,
    /// Trade transaction (default)
    #[default]
    Trade,
    /// Transfer transaction
    Transfer,
    /// Fee transaction
    Fee,
    /// Funding transaction
    Funding,
    /// Bonus transaction
    Bonus,
    /// Dividend transaction
    Dividend,
    /// Liquidation transaction
    Liquidation,
    /// Insurance transaction
    Insurance,
}

/// Generic transaction log entry
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Unique transaction identifier
    pub id: u64,
    /// Currency of the transaction
    pub currency: String,
    /// Transaction amount
    pub amount: f64,
    /// Account balance after transaction
    pub balance: f64,
    /// Transaction timestamp
    pub timestamp: u64,
    /// Type of transaction
    pub transaction_type: TransactionType,
    /// Additional transaction information
    pub info: Option<String>,
}

impl Default for TransactionLogEntry {
    fn default() -> Self {
        Self {
            id: 0,
            currency: String::new(),
            amount: 0.0,
            balance: 0.0,
            timestamp: 0,
            transaction_type: TransactionType::default(),
            info: None,
        }
    }
}

impl_json_display!(TransactionLogEntry);
impl_json_debug_pretty!(TransactionLogEntry);

/// Paginated transaction log response
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TransactionLog {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of transaction log entries
    pub logs: Vec<TransactionLogEntry>,
}

impl_json_display!(TransactionLog);
impl_json_debug_pretty!(TransactionLog);

/// Deposit information
#[derive(Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,
    /// Deposit amount
    pub amount: f64,
    /// Currency of the deposit
    pub currency: String,
    /// Current state of the deposit
    pub state: String,
    /// Timestamp when deposit was received
    pub received_timestamp: u64,
    /// Transaction ID on the blockchain
    pub transaction_id: Option<String>,
    /// Timestamp when deposit was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<u64>,
}

impl_json_display!(Deposit);
impl_json_debug_pretty!(Deposit);

/// Deposits response wrapper
#[derive(Clone, Serialize, Deserialize)]
pub struct DepositsResponse {
    /// Total count of deposits
    pub count: u32,
    /// List of deposit entries
    pub data: Vec<Deposit>,
}

impl_json_display!(DepositsResponse);
impl_json_debug_pretty!(DepositsResponse);

/// Withdrawal information
#[derive(Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal address
    pub address: String,
    /// Withdrawal amount
    pub amount: f64,
    /// Currency of the withdrawal
    pub currency: String,
    /// Withdrawal fee
    pub fee: f64,
    /// Unique withdrawal identifier
    pub id: u64,
    /// Withdrawal priority level
    pub priority: String,
    /// Current state of the withdrawal
    pub state: String,
    /// Timestamp when withdrawal was created
    pub created_timestamp: u64,
    /// Timestamp when withdrawal was last updated
    pub updated_timestamp: Option<u64>,
    /// Transaction ID on the blockchain
    pub transaction_id: Option<String>,
}

impl_json_display!(Withdrawal);
impl_json_debug_pretty!(Withdrawal);

/// Withdrawals response wrapper
#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalsResponse {
    /// Total count of withdrawals
    pub count: u32,
    /// List of withdrawal entries
    pub data: Vec<Withdrawal>,
}

impl_json_display!(WithdrawalsResponse);
impl_json_debug_pretty!(WithdrawalsResponse);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_transaction_log_entry() {
        let tx = TransactionLogEntry::default();
        assert_eq!(tx.id, 0);
        assert_eq!(tx.amount, 0.0);
    }
}
