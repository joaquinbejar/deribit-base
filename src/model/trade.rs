/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::{instrument::InstrumentKind, order::OrderSide};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Liquidity type enumeration
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Liquidity {
    /// Maker (provided liquidity)
    #[serde(rename = "M")]
    Maker,
    /// Taker (consumed liquidity)
    #[serde(rename = "T")]
    Taker,
    /// Mixed (both maker and taker in same trade)
    #[serde(rename = "MT")]
    Mixed,
}

/// Trade execution information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique trade identifier
    pub trade_id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Trade direction (buy/sell)
    pub direction: OrderSide,
    /// Trade amount
    pub amount: f64,
    /// Execution price
    pub price: f64,
    /// Trade timestamp
    pub timestamp: i64,
    /// Fee amount
    pub fee: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Liquidity type (maker/taker)
    pub liquidity: Liquidity,
    /// Mark price at time of trade
    pub mark_price: f64,
    /// Index price at time of trade
    pub index_price: f64,
    /// Instrument kind
    pub instrument_kind: Option<InstrumentKind>,
    /// Trade sequence number
    pub trade_seq: Option<u64>,
    /// User role in the trade
    pub user_role: Option<String>,
    /// Whether this is a block trade
    pub block_trade: Option<bool>,
    /// Underlying price (for options)
    pub underlying_price: Option<f64>,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Label associated with the order
    pub label: Option<String>,
    /// Profit and loss from this trade
    pub profit_loss: Option<f64>,
    /// Tick direction
    pub tick_direction: Option<i32>,
    /// Whether this trade was self-traded
    pub self_trade: Option<bool>,
}

impl Trade {
    /// Calculate the notional value of the trade
    pub fn notional_value(&self) -> f64 {
        self.amount * self.price
    }

    /// Check if this was a maker trade
    pub fn is_maker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Maker | Liquidity::Mixed)
    }

    /// Check if this was a taker trade
    pub fn is_taker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Taker | Liquidity::Mixed)
    }

    /// Check if this is a buy trade
    pub fn is_buy(&self) -> bool {
        self.direction == OrderSide::Buy
    }

    /// Check if this is a sell trade
    pub fn is_sell(&self) -> bool {
        self.direction == OrderSide::Sell
    }

    /// Get fee as percentage of notional
    pub fn fee_percentage(&self) -> f64 {
        if self.notional_value() != 0.0 {
            (self.fee / self.notional_value()) * 100.0
        } else {
            0.0
        }
    }
}

/// Trade statistics
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradeStats {
    /// Total number of trades
    pub count: u64,
    /// Total volume
    pub volume: f64,
    /// Total fees paid
    pub total_fees: f64,
    /// Average price
    pub avg_price: f64,
    /// Profit and loss
    pub pnl: f64,
    /// Number of winning trades
    pub winning_trades: u64,
    /// Number of losing trades
    pub losing_trades: u64,
}

impl TradeStats {
    /// Create empty trade statistics
    pub fn new() -> Self {
        Self {
            count: 0,
            volume: 0.0,
            total_fees: 0.0,
            avg_price: 0.0,
            pnl: 0.0,
            winning_trades: 0,
            losing_trades: 0,
        }
    }

    /// Calculate win rate as percentage
    pub fn win_rate(&self) -> f64 {
        if self.count > 0 {
            (self.winning_trades as f64 / self.count as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for TradeStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Trade execution
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Trading fee paid
    pub fee: f64,
    /// Currency of the trading fee
    pub fee_currency: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// User-defined label for the trade
    pub label: String,
    /// Liquidity type (maker/taker)
    pub liquidity: String,
    /// Mark price at execution time
    pub mark_price: f64,
    /// Matching engine identifier
    pub matching_id: Option<String>,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Type of the order that generated this trade
    pub order_type: String,
    /// Original order type before modifications
    pub original_order_type: Option<String>,
    /// Execution price
    pub price: f64,
    /// Whether this was a self trade
    pub self_trade: bool,
    /// Current state of the trade
    pub state: String,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
    /// Underlying asset price (for derivatives)
    pub underlying_price: Option<f64>,
}

/// User trade information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UserTrade {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Trading fee paid
    pub fee: f64,
    /// Currency of the trading fee
    pub fee_currency: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// User-defined label for the trade
    pub label: String,
    /// Liquidity type (maker/taker)
    pub liquidity: String,
    /// Mark price at execution time
    pub mark_price: f64,
    /// Matching engine identifier
    pub matching_id: Option<String>,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Type of the order that generated this trade
    pub order_type: String,
    /// Original order type before modifications
    pub original_order_type: Option<String>,
    /// Execution price
    pub price: f64,
    /// Whether this was a self trade
    pub self_trade: bool,
    /// Current state of the trade
    pub state: String,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
    /// Underlying asset price (for derivatives)
    pub underlying_price: Option<f64>,
}

/// Last trade
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct LastTrade {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Liquidity information
    pub liquid: Option<String>,
    /// Execution price
    pub price: f64,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::instrument::InstrumentKind;
    use crate::model::order::OrderSide;

    #[test]
    fn test_liquidity_variants() {
        let maker = Liquidity::Maker;
        let taker = Liquidity::Taker;
        let mixed = Liquidity::Mixed;

        assert_eq!(maker, Liquidity::Maker);
        assert_eq!(taker, Liquidity::Taker);
        assert_eq!(mixed, Liquidity::Mixed);
    }

    #[test]
    fn test_liquidity_serialization() {
        let maker = Liquidity::Maker;
        let taker = Liquidity::Taker;
        let mixed = Liquidity::Mixed;

        let maker_json = serde_json::to_string(&maker).unwrap();
        let taker_json = serde_json::to_string(&taker).unwrap();
        let mixed_json = serde_json::to_string(&mixed).unwrap();

        assert_eq!(maker_json, "\"M\"");
        assert_eq!(taker_json, "\"T\"");
        assert_eq!(mixed_json, "\"MT\"");

        let maker_deserialized: Liquidity = serde_json::from_str(&maker_json).unwrap();
        let taker_deserialized: Liquidity = serde_json::from_str(&taker_json).unwrap();
        let mixed_deserialized: Liquidity = serde_json::from_str(&mixed_json).unwrap();

        assert_eq!(maker_deserialized, Liquidity::Maker);
        assert_eq!(taker_deserialized, Liquidity::Taker);
        assert_eq!(mixed_deserialized, Liquidity::Mixed);
    }

    #[test]
    fn test_trade_creation() {
        let trade = Trade {
            trade_id: "12345".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_123".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50010.0,
            index_price: 50005.0,
            instrument_kind: Some(InstrumentKind::Future),
            trade_seq: Some(12345),
            user_role: Some("maker".to_string()),
            block_trade: Some(false),
            underlying_price: Some(50000.0),
            iv: None,
            label: Some("test_trade".to_string()),
            profit_loss: Some(100.0),
            tick_direction: Some(1),
            self_trade: Some(false),
        };

        assert_eq!(trade.trade_id, "12345");
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.direction, OrderSide::Buy);
        assert_eq!(trade.amount, 1.0);
        assert_eq!(trade.price, 50000.0);
        assert_eq!(trade.fee, 25.0);
        assert_eq!(trade.liquidity, Liquidity::Maker);
    }

    #[test]
    fn test_trade_notional_value() {
        let trade = Trade {
            trade_id: "12345".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_123".to_string(),
            direction: OrderSide::Buy,
            amount: 2.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 50.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50010.0,
            index_price: 50005.0,
            instrument_kind: None,
            trade_seq: None,
            user_role: None,
            block_trade: None,
            underlying_price: None,
            iv: None,
            label: None,
            profit_loss: None,
            tick_direction: None,
            self_trade: None,
        };

        assert_eq!(trade.notional_value(), 100000.0);
    }

    #[test]
    fn test_trade_liquidity_checks() {
        let maker_trade = Trade {
            trade_id: "1".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_1".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50000.0,
            index_price: 50000.0,
            instrument_kind: None,
            trade_seq: None,
            user_role: None,
            block_trade: None,
            underlying_price: None,
            iv: None,
            label: None,
            profit_loss: None,
            tick_direction: None,
            self_trade: None,
        };

        let taker_trade = Trade {
            liquidity: Liquidity::Taker,
            ..maker_trade.clone()
        };

        let mixed_trade = Trade {
            liquidity: Liquidity::Mixed,
            ..maker_trade.clone()
        };

        assert!(maker_trade.is_maker());
        assert!(!maker_trade.is_taker());

        assert!(!taker_trade.is_maker());
        assert!(taker_trade.is_taker());

        assert!(mixed_trade.is_maker());
        assert!(mixed_trade.is_taker());
    }

    #[test]
    fn test_trade_direction_checks() {
        let buy_trade = Trade {
            trade_id: "1".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_1".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50000.0,
            index_price: 50000.0,
            instrument_kind: None,
            trade_seq: None,
            user_role: None,
            block_trade: None,
            underlying_price: None,
            iv: None,
            label: None,
            profit_loss: None,
            tick_direction: None,
            self_trade: None,
        };

        let sell_trade = Trade {
            direction: OrderSide::Sell,
            ..buy_trade.clone()
        };

        assert!(buy_trade.is_buy());
        assert!(!buy_trade.is_sell());

        assert!(!sell_trade.is_buy());
        assert!(sell_trade.is_sell());
    }

    #[test]
    fn test_trade_fee_percentage() {
        let trade = Trade {
            trade_id: "1".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_1".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50000.0,
            index_price: 50000.0,
            instrument_kind: None,
            trade_seq: None,
            user_role: None,
            block_trade: None,
            underlying_price: None,
            iv: None,
            label: None,
            profit_loss: None,
            tick_direction: None,
            self_trade: None,
        };

        assert_eq!(trade.fee_percentage(), 0.05); // 25 / 50000 * 100

        let zero_notional_trade = Trade {
            amount: 0.0,
            price: 0.0,
            ..trade
        };

        assert_eq!(zero_notional_trade.fee_percentage(), 0.0);
    }

    #[test]
    fn test_trade_stats_new() {
        let stats = TradeStats::new();
        assert_eq!(stats.count, 0);
        assert_eq!(stats.volume, 0.0);
        assert_eq!(stats.total_fees, 0.0);
        assert_eq!(stats.avg_price, 0.0);
        assert_eq!(stats.pnl, 0.0);
        assert_eq!(stats.winning_trades, 0);
        assert_eq!(stats.losing_trades, 0);
    }

    #[test]
    fn test_trade_stats_default() {
        let stats = TradeStats::default();
        assert_eq!(stats.count, 0);
        assert_eq!(stats.volume, 0.0);
        assert_eq!(stats.total_fees, 0.0);
        assert_eq!(stats.avg_price, 0.0);
        assert_eq!(stats.pnl, 0.0);
        assert_eq!(stats.winning_trades, 0);
        assert_eq!(stats.losing_trades, 0);
    }

    #[test]
    fn test_trade_stats_win_rate() {
        let mut stats = TradeStats::new();
        stats.count = 10;
        stats.winning_trades = 7;
        stats.losing_trades = 3;

        assert_eq!(stats.win_rate(), 70.0);

        let empty_stats = TradeStats::new();
        assert_eq!(empty_stats.win_rate(), 0.0);
    }

    #[test]
    fn test_trade_execution_creation() {
        let execution = TradeExecution {
            amount: 1.0,
            direction: "buy".to_string(),
            fee: 25.0,
            fee_currency: "USD".to_string(),
            index_price: 50005.0,
            instrument_name: "BTC-PERPETUAL".to_string(),
            iv: Some(0.5),
            label: "test_label".to_string(),
            liquidity: "M".to_string(),
            mark_price: 50010.0,
            matching_id: Some("match_123".to_string()),
            order_id: "order_123".to_string(),
            order_type: "limit".to_string(),
            original_order_type: Some("limit".to_string()),
            price: 50000.0,
            self_trade: false,
            state: "filled".to_string(),
            tick_direction: 1,
            timestamp: 1640995200000,
            trade_id: "trade_123".to_string(),
            trade_seq: 12345,
            underlying_price: Some(50000.0),
        };

        assert_eq!(execution.amount, 1.0);
        assert_eq!(execution.direction, "buy");
        assert_eq!(execution.fee, 25.0);
        assert_eq!(execution.instrument_name, "BTC-PERPETUAL");
        assert_eq!(execution.price, 50000.0);
        assert_eq!(execution.trade_id, "trade_123");
        assert!(!execution.self_trade);
    }

    #[test]
    fn test_user_trade_creation() {
        let user_trade = UserTrade {
            amount: 2.0,
            direction: "sell".to_string(),
            fee: 50.0,
            fee_currency: "USD".to_string(),
            index_price: 49995.0,
            instrument_name: "ETH-PERPETUAL".to_string(),
            iv: None,
            label: "user_label".to_string(),
            liquidity: "T".to_string(),
            mark_price: 49990.0,
            matching_id: None,
            order_id: "user_order_456".to_string(),
            order_type: "market".to_string(),
            original_order_type: None,
            price: 49985.0,
            self_trade: true,
            state: "filled".to_string(),
            tick_direction: -1,
            timestamp: 1640995300000,
            trade_id: "user_trade_456".to_string(),
            trade_seq: 12346,
            underlying_price: None,
        };

        assert_eq!(user_trade.amount, 2.0);
        assert_eq!(user_trade.direction, "sell");
        assert_eq!(user_trade.fee, 50.0);
        assert_eq!(user_trade.instrument_name, "ETH-PERPETUAL");
        assert_eq!(user_trade.price, 49985.0);
        assert_eq!(user_trade.trade_id, "user_trade_456");
        assert!(user_trade.self_trade);
        assert_eq!(user_trade.tick_direction, -1);
    }

    #[test]
    fn test_last_trade_creation() {
        let last_trade = LastTrade {
            amount: 0.5,
            direction: "buy".to_string(),
            index_price: 50005.0,
            instrument_name: "BTC-25DEC24-50000-C".to_string(),
            iv: Some(0.75),
            liquid: Some("liquid".to_string()),
            price: 2500.0,
            tick_direction: 0,
            timestamp: 1640995400000,
            trade_id: "last_trade_789".to_string(),
            trade_seq: 12347,
        };

        assert_eq!(last_trade.amount, 0.5);
        assert_eq!(last_trade.direction, "buy");
        assert_eq!(last_trade.index_price, 50005.0);
        assert_eq!(last_trade.instrument_name, "BTC-25DEC24-50000-C");
        assert_eq!(last_trade.iv, Some(0.75));
        assert_eq!(last_trade.price, 2500.0);
        assert_eq!(last_trade.tick_direction, 0);
        assert_eq!(last_trade.trade_id, "last_trade_789");
        assert_eq!(last_trade.trade_seq, 12347);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let trade = Trade {
            trade_id: "test_trade".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_123".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50010.0,
            index_price: 50005.0,
            instrument_kind: Some(InstrumentKind::Future),
            trade_seq: Some(12345),
            user_role: Some("maker".to_string()),
            block_trade: Some(false),
            underlying_price: Some(50000.0),
            iv: None,
            label: Some("test_label".to_string()),
            profit_loss: Some(100.0),
            tick_direction: Some(1),
            self_trade: Some(false),
        };

        let json = serde_json::to_string(&trade).unwrap();
        let deserialized: Trade = serde_json::from_str(&json).unwrap();

        assert_eq!(trade.trade_id, deserialized.trade_id);
        assert_eq!(trade.instrument_name, deserialized.instrument_name);
        assert_eq!(trade.direction, deserialized.direction);
        assert_eq!(trade.amount, deserialized.amount);
        assert_eq!(trade.price, deserialized.price);
        assert_eq!(trade.liquidity, deserialized.liquidity);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let liquidity = Liquidity::Maker;
        let debug_str = format!("{:?}", liquidity);
        let display_str = format!("{}", liquidity);

        assert!(debug_str.contains("Maker") || debug_str.contains("M"));
        assert!(display_str.contains("M"));

        let stats = TradeStats::new();
        let stats_debug = format!("{:?}", stats);
        let stats_display = format!("{}", stats);

        assert!(stats_debug.contains("count") || stats_debug.contains("0"));
        assert!(stats_display.contains("0"));
    }

    #[test]
    fn test_cloning() {
        let trade = Trade {
            trade_id: "clone_test".to_string(),
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_id: "order_123".to_string(),
            direction: OrderSide::Buy,
            amount: 1.0,
            price: 50000.0,
            timestamp: 1640995200000,
            fee: 25.0,
            fee_currency: "USD".to_string(),
            liquidity: Liquidity::Maker,
            mark_price: 50010.0,
            index_price: 50005.0,
            instrument_kind: None,
            trade_seq: None,
            user_role: None,
            block_trade: None,
            underlying_price: None,
            iv: None,
            label: None,
            profit_loss: None,
            tick_direction: None,
            self_trade: None,
        };

        let cloned_trade = trade.clone();
        assert_eq!(trade.trade_id, cloned_trade.trade_id);
        assert_eq!(trade.amount, cloned_trade.amount);
        assert_eq!(trade.price, cloned_trade.price);
        assert_eq!(trade.liquidity, cloned_trade.liquidity);

        let liquidity = Liquidity::Taker;
        let cloned_liquidity = liquidity.clone();
        assert_eq!(liquidity, cloned_liquidity);
    }
}
