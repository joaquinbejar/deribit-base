/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

//! Common exports for easy importing

// Re-export commonly used types
pub use crate::error::types::{DeribitError, DeribitResult};

// Model exports
pub use crate::model::{
    account::{AccountSummary, Portfolio},
    config::{DeribitConfig, DeribitUrls, HttpConfig, WebSocketConfig},
    currency::Currency,
    instrument::{Instrument, InstrumentKind, InstrumentType, OptionType},
    market_data::{Candle, Greeks, MarketStats, OrderBook, OrderBookEntry, Ticker},
    order::{OrderSide, OrderStatus, OrderType, TimeInForce},
    position::Position,
    request::{
        AdvancedOrderType,
        AuthRequest,
        CancelAllOrdersRequest,
        CancelOrderRequest,
        ClosePositionRequest,
        ModifyOrderRequest,
        NewOrderRequest,
        TriggerType,
        fix, // FIX protocol structures
    },
    response::{
        AuthResponse, HeartbeatResponse, JsonRpcError, JsonRpcResponse, Notification,
        PaginatedResponse, Pagination, ServerTimeResponse, SubscriptionResponse, TestResponse,
    },
    trade::{Liquidity, Trade, TradeStats},
};

// Utility exports - macros and functions
pub use crate::utils::setup_logger;
