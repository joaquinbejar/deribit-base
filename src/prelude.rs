/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

//! Common exports for easy importing

// Re-export commonly used types
pub use crate::error::{
    codes::DeribitErrorCode,
    types::{DeribitError, DeribitResult},
};

// Model exports
pub use crate::model::{
    account::{AccountSummary, Portfolio},
    book_summary::{BookSummaries, BookSummary},
    config::{DeribitConfig, DeribitUrls, HttpConfig, WebSocketConfig},
    extended_market_data::{
        CurrencyInfo, CurrencyInfoCollection, FundingRate, HistoricalVolatility, IndexPrice,
        MarketStatistics, WithdrawalPriority,
    },
    funding::{FundingChartData, FundingDataPoint, FundingRateData, TradingViewChartData},
    instrument::{Instrument, InstrumentKind, InstrumentType, OptionType},
    market_data::{Candle, Greeks, MarketStats, OrderBook, OrderBookEntry, Ticker},
    order::{OrderSide, OrderStatus, OrderType, TimeInForce},
    order_management::{
        BuyOrderRequest, EditOrderRequest, MassQuoteItem, MassQuoteRequest, SellOrderRequest,
        TransferResult,
    },
    position::Position,
    request::{
        AdvancedOrderType, AuthRequest, CancelAllOrdersRequest, CancelOrderRequest,
        ClosePositionRequest, ModifyOrderRequest, NewOrderRequest, TriggerType,
    },
    response::{
        AuthResponse, HeartbeatResponse, JsonRpcError, JsonRpcResponse, Notification,
        PaginatedResponse, Pagination, ServerTimeResponse, SubscriptionResponse, TestResponse,
    },
    settlement::{Settlement, SettlementType, Settlements},
    ticker::{TickerData, TickerStats},
    trade::{Liquidity, Trade, TradeStats},
    transaction::{
        Deposit, DepositsResponse, TransactionLog, TransactionLogEntry, TransactionType,
        Withdrawal, WithdrawalsResponse,
    },
    transfer::{
        AddressBookEntry, AddressType, SubaccountTransfer, Transfer, TransferState, Transfers,
    },
};

// Utility exports - macros and functions
pub use crate::constants::*;
pub use crate::utils::{
    logger::setup_logger,
    tools::{generate_nonce, generate_timestamp},
};
