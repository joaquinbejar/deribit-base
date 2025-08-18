/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

//! Data models and structures for Deribit API
//!
//! This module contains all the data structures and types used across
//! the Deribit API implementations. These models are protocol-agnostic
//! and can be used with FIX, HTTP REST, and WebSocket clients.
//!
//! The models are organized by functional areas:
//! - Account and portfolio management
//! - Trading (orders, positions, trades)
//! - Market data (tickers, order books, instruments)
//! - Configuration and requests/responses
//! - Settlements and transfers

/// Account-related data structures and types
pub mod account;
/// Book summary and market overview structures
pub mod book_summary;
/// Configuration structures
pub mod config;
/// Currency enumeration and utilities
pub mod currency;
/// Extended market data structures
pub mod extended_market_data;
pub mod funding;
/// Instrument-related data structures and types
pub mod instrument;
/// Market data structures
pub mod market_data;
/// Order-related data structures and types
pub mod order;
pub mod order_management;
/// Position-related data structures and types
pub mod position;
/// Request structures for API calls
pub mod request;
/// Response structures and wrappers
pub mod response;
/// Settlement and delivery data structures
pub mod settlement;
/// Ticker data structures
pub mod ticker;
/// Trade-related data structures and types
pub mod trade;
pub mod transaction;
/// Transfer and withdrawal data structures
pub mod transfer;

pub use funding::{FundingChartData, FundingDataPoint, FundingRateData, TradingViewChartData};
pub use order_management::{
    BuyOrderRequest, EditOrderRequest, MassQuoteItem, MassQuoteRequest, SellOrderRequest,
    TransferResult,
};
pub use transaction::{
    Deposit, DepositsResponse, TransactionLog, TransactionLogEntry, TransactionType, Withdrawal,
    WithdrawalsResponse,
};
