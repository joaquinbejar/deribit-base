/****************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Trade,
    Transfer,
    Fee,
    Funding,
    Bonus,
    Dividend,
    Liquidation,
    Insurance,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Trade
    }
}

/// Generic transaction log entry
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    pub id: u64,
    pub currency: String,
    pub amount: f64,
    pub balance: f64,
    pub timestamp: u64,
    pub transaction_type: TransactionType,
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
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLog {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of transaction log entries
    pub logs: Vec<TransactionLogEntry>,
}

impl Default for TransactionLog {
    fn default() -> Self {
        Self {
            continuation: None,
            logs: Vec::new(),
        }
    }
}

impl_json_display!(TransactionLog);
impl_json_debug_pretty!(TransactionLog);

/// Deposit information
#[derive(Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub address: String,
    pub amount: f64,
    pub currency: String,
    pub state: String,
    pub received_timestamp: u64,
    pub transaction_id: Option<String>,
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
    pub address: String,
    pub amount: f64,
    pub currency: String,
    pub fee: f64,
    pub id: u64,
    pub priority: String,
    pub state: String,
    pub created_timestamp: u64,
    pub updated_timestamp: Option<u64>,
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
