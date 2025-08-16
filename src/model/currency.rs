/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{impl_json_debug_pretty, impl_json_display};
/// Currencies supported by Deribit
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
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

impl Currency {
    /// Get the string representation of the currency
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Bitcoin => "BTC",
            Currency::Ethereum => "ETH",
            Currency::Solana => "SOL",
            Currency::UsdCoin => "USDC",
            Currency::Tether => "USDT",
            Currency::EuroR => "EURR",
            Currency::Matic => "MATIC",
            Currency::Ripple => "XRP",
            Currency::BinanceCoin => "BNB",
            Currency::PaxGold => "PAXG",
            Currency::StakedEthereum => "STETH",
            Currency::EthereumPoW => "ETHW",
            Currency::YieldUsdCoin => "USYC",
            Currency::EthenaUsd => "USDE",
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

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BTC" => Ok(Currency::Bitcoin),
            "ETH" => Ok(Currency::Ethereum),
            "SOL" => Ok(Currency::Solana),
            "USDC" => Ok(Currency::UsdCoin),
            "USDT" => Ok(Currency::Tether),
            "EURR" => Ok(Currency::EuroR),
            "MATIC" => Ok(Currency::Matic),
            "XRP" => Ok(Currency::Ripple),
            "BNB" => Ok(Currency::BinanceCoin),
            "PAXG" => Ok(Currency::PaxGold),
            "STETH" => Ok(Currency::StakedEthereum),
            "ETHW" => Ok(Currency::EthereumPoW),
            "USYC" => Ok(Currency::YieldUsdCoin),
            "USDE" => Ok(Currency::EthenaUsd),
            _ => Err(ParseCurrencyError(s.to_string())),
        }
    }
}

// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(Currency);

// Display implementations using compact JSON formatting
impl_json_display!(Currency);
