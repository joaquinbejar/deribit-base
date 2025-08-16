/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::currency::Currency;
use serde::{Deserialize, Serialize};

use crate::{impl_json_debug_pretty, impl_json_display};
/// Account summary information
#[derive(Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    /// Account currency
    pub currency: Currency,
    /// Total balance
    pub balance: f64,
    /// Account equity
    pub equity: f64,
    /// Available funds for trading
    pub available_funds: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// Unrealized profit and loss
    pub unrealized_pnl: f64,
    /// Realized profit and loss
    pub realized_pnl: f64,
    /// Total profit and loss
    pub total_pl: f64,
    /// Session funding
    pub session_funding: f64,
    /// Session realized P&L
    pub session_rpl: f64,
    /// Session unrealized P&L
    pub session_upl: f64,
    /// Maintenance margin requirement
    pub maintenance_margin: f64,
    /// Initial margin requirement
    pub initial_margin: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: Option<f64>,
    /// Cross collateral enabled
    pub cross_collateral_enabled: Option<bool>,
    /// Delta total
    pub delta_total: Option<f64>,
    /// Futures profit and loss
    pub futures_pl: Option<f64>,
    /// Futures session realized profit and loss
    pub futures_session_rpl: Option<f64>,
    /// Futures session unrealized profit and loss
    pub futures_session_upl: Option<f64>,
    /// Options delta
    pub options_delta: Option<f64>,
    /// Options gamma
    pub options_gamma: Option<f64>,
    /// Options profit and loss
    pub options_pl: Option<f64>,
    /// Options session realized profit and loss
    pub options_session_rpl: Option<f64>,
    /// Options session unrealized profit and loss
    pub options_session_upl: Option<f64>,
    /// Options theta
    pub options_theta: Option<f64>,
    /// Options vega
    pub options_vega: Option<f64>,
    /// Portfolio margin enabled
    pub portfolio_margining_enabled: Option<bool>,
    /// Projected delta total
    pub projected_delta_total: Option<f64>,
    /// Projected initial margin
    pub projected_initial_margin: Option<f64>,
    /// Projected maintenance margin
    pub projected_maintenance_margin: Option<f64>,
    /// System name
    pub system_name: Option<String>,
    /// Type of account
    pub type_: Option<String>,
}

impl AccountSummary {
    /// Calculate margin utilization as percentage
    pub fn margin_utilization(&self) -> f64 {
        if self.equity != 0.0 {
            (self.initial_margin / self.equity) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate available margin
    pub fn available_margin(&self) -> f64 {
        self.equity - self.initial_margin
    }

    /// Check if account is at risk (high margin utilization)
    pub fn is_at_risk(&self, threshold: f64) -> bool {
        self.margin_utilization() > threshold
    }

    /// Calculate return on equity
    pub fn return_on_equity(&self) -> f64 {
        if self.equity != 0.0 {
            (self.total_pl / self.equity) * 100.0
        } else {
            0.0
        }
    }

    /// Get currency as string
    pub fn currency_str(&self) -> &'static str {
        self.currency.as_str()
    }
}

/// Portfolio information
#[derive(Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Currency of the portfolio
    pub currency: Currency,
    /// Account summaries for different currencies
    pub accounts: Vec<AccountSummary>,
    /// Total portfolio value in USD
    pub total_usd_value: Option<f64>,
    /// Cross-currency margin enabled
    pub cross_margin_enabled: bool,
}

impl Portfolio {
    /// Create a new empty portfolio
    pub fn new(currency: Currency) -> Self {
        Self {
            currency,
            accounts: Vec::new(),
            total_usd_value: None,
            cross_margin_enabled: false,
        }
    }

    /// Add an account summary to the portfolio
    pub fn add_account(&mut self, account: AccountSummary) {
        self.accounts.push(account);
    }

    /// Get account summary for a specific currency
    pub fn get_account(&self, currency: &Currency) -> Option<&AccountSummary> {
        self.accounts.iter().find(|acc| &acc.currency == currency)
    }

    /// Calculate total equity across all accounts
    pub fn total_equity(&self) -> f64 {
        self.accounts.iter().map(|acc| acc.equity).sum()
    }

    /// Calculate total unrealized PnL across all accounts
    pub fn total_unrealized_pnl(&self) -> f64 {
        self.accounts.iter().map(|acc| acc.unrealized_pnl).sum()
    }

    /// Calculate total realized PnL across all accounts
    pub fn total_realized_pnl(&self) -> f64 {
        self.accounts.iter().map(|acc| acc.realized_pnl).sum()
    }
}

// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(
    AccountSummary,    Portfolio
);

// Display implementations using compact JSON formatting
impl_json_display!(
    AccountSummary,    Portfolio
);
