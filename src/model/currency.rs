/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{impl_json_debug_pretty, impl_json_display};
use crate::prelude::WithdrawalPriority;

/// Currencies supported by Deribit
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currencies {
    /// Bitcoin
    #[serde(rename = "BTC")]
    Bitcoin,
    /// Ethereum
    #[serde(rename = "ETH")]
    Ethereum,
    /// Solana
    #[serde(rename = "SOL")]
    Solana,
    /// USD Coin
    #[serde(rename = "USDC")]
    UsdCoin,
    /// Tether
    #[serde(rename = "USDT")]
    Tether,
    /// Euro Coin
    #[serde(rename = "EURR")]
    EuroR,
    /// Polygon
    #[serde(rename = "MATIC")]
    Matic,
    /// Ripple
    #[serde(rename = "XRP")]
    Ripple,
    /// Binance Coin
    #[serde(rename = "BNB")]
    BinanceCoin,
    /// PAX Gold
    #[serde(rename = "PAXG")]
    PaxGold,
    /// Ethereum Staking
    #[serde(rename = "STETH")]
    StakedEthereum,
    /// Ethereum PoW
    #[serde(rename = "ETHW")]
    EthereumPoW,
    /// Yield USD Coin
    #[serde(rename = "USYC")]
    YieldUsdCoin,
    /// Ethena USDe
    #[serde(rename = "USDE")]
    EthenaUsd,
}

impl Currencies {
    /// Get the string representation of the currency
    pub fn as_str(&self) -> &'static str {
        match self {
            Currencies::Bitcoin => "BTC",
            Currencies::Ethereum => "ETH",
            Currencies::Solana => "SOL",
            Currencies::UsdCoin => "USDC",
            Currencies::Tether => "USDT",
            Currencies::EuroR => "EURR",
            Currencies::Matic => "MATIC",
            Currencies::Ripple => "XRP",
            Currencies::BinanceCoin => "BNB",
            Currencies::PaxGold => "PAXG",
            Currencies::StakedEthereum => "STETH",
            Currencies::EthereumPoW => "ETHW",
            Currencies::YieldUsdCoin => "USYC",
            Currencies::EthenaUsd => "USDE",
        }
    }
}

/// Error type for parsing Currency from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCurrencyError(pub String);

impl std::fmt::Display for ParseCurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown currency: {}", self.0)
    }
}

impl std::error::Error for ParseCurrencyError {}

impl FromStr for Currencies {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BTC" => Ok(Currencies::Bitcoin),
            "ETH" => Ok(Currencies::Ethereum),
            "SOL" => Ok(Currencies::Solana),
            "USDC" => Ok(Currencies::UsdCoin),
            "USDT" => Ok(Currencies::Tether),
            "EURR" => Ok(Currencies::EuroR),
            "MATIC" => Ok(Currencies::Matic),
            "XRP" => Ok(Currencies::Ripple),
            "BNB" => Ok(Currencies::BinanceCoin),
            "PAXG" => Ok(Currencies::PaxGold),
            "STETH" => Ok(Currencies::StakedEthereum),
            "ETHW" => Ok(Currencies::EthereumPoW),
            "USYC" => Ok(Currencies::YieldUsdCoin),
            "USDE" => Ok(Currencies::EthenaUsd),
            _ => Err(ParseCurrencyError(s.to_string())),
        }
    }
}

/// Currency structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency symbol (BTC, ETH, etc.)
    pub currency: String,
    /// Long currency name
    pub currency_long: String,
    /// Withdrawal fee
    pub fee_precision: u32,
    /// Minimum withdrawal amount
    pub min_confirmations: u32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Withdrawal precision
    pub withdrawal_fee: f64,
    /// Withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// APR for yield-generating tokens
    pub apr: Option<f64>,
}



// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(Currencies, Currency);

// Display implementations using compact JSON formatting
impl_json_display!(Currencies, Currency);
