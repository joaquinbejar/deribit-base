/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Deribit API Constants
//!
//! This module contains all the constants used throughout the Deribit API implementation,
//! including URLs, limits, timeouts, and other configuration values based on the
//! official Deribit API v2.1.1 documentation.

// =============================================================================
// API ENDPOINTS
// =============================================================================

/// Deribit production WebSocket URL
pub const DERIBIT_WS_URL_PROD: &str = "wss://www.deribit.com/ws/api/v2";

/// Deribit test WebSocket URL
pub const DERIBIT_WS_URL_TEST: &str = "wss://test.deribit.com/ws/api/v2";

/// Deribit production HTTP API URL
pub const DERIBIT_HTTP_URL_PROD: &str = "https://www.deribit.com/api/v2";

/// Deribit test HTTP API URL
pub const DERIBIT_HTTP_URL_TEST: &str = "https://test.deribit.com/api/v2";

// =============================================================================
// RATE LIMITS
// =============================================================================

/// Maximum requests per second for authenticated users
pub const MAX_REQUESTS_PER_SECOND_AUTH: u32 = 20;

/// Maximum requests per second for non-authenticated users
pub const MAX_REQUESTS_PER_SECOND_UNAUTH: u32 = 10;

/// Maximum subscriptions per connection
pub const MAX_SUBSCRIPTIONS_PER_CONNECTION: u32 = 200;

/// Maximum message size in bytes
pub const MAX_MESSAGE_SIZE_BYTES: usize = 65536; // 64KB

// =============================================================================
// TIMEOUTS
// =============================================================================

/// Default connection timeout in milliseconds
pub const DEFAULT_CONNECTION_TIMEOUT_MS: u64 = 5000;

/// Default request timeout in milliseconds
pub const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 10000;

/// Heartbeat interval in milliseconds
pub const HEARTBEAT_INTERVAL_MS: u64 = 10000;

/// Maximum time to wait for heartbeat response in milliseconds
pub const HEARTBEAT_TIMEOUT_MS: u64 = 5000;

// =============================================================================
// CURRENCIES
// =============================================================================

/// Supported cryptocurrencies
pub const SUPPORTED_CRYPTOCURRENCIES: &[&str] = &["BTC", "ETH", "SOL", "USDC", "USDT", "EURR"];

/// Bitcoin currency code
pub const CURRENCY_BTC: &str = "BTC";

/// Ethereum currency code
pub const CURRENCY_ETH: &str = "ETH";

/// Solana currency code
pub const CURRENCY_SOL: &str = "SOL";

/// USD Coin currency code
pub const CURRENCY_USDC: &str = "USDC";

/// Tether currency code
pub const CURRENCY_USDT: &str = "USDT";

/// Euro stablecoin currency code
pub const CURRENCY_EURR: &str = "EURR";

// =============================================================================
// INSTRUMENT TYPES
// =============================================================================

/// Future instrument type
pub const INSTRUMENT_TYPE_FUTURE: &str = "future";

/// Option instrument type
pub const INSTRUMENT_TYPE_OPTION: &str = "option";

/// Perpetual instrument type
pub const INSTRUMENT_TYPE_PERPETUAL: &str = "perpetual";

/// Spot instrument type
pub const INSTRUMENT_TYPE_SPOT: &str = "spot";

/// Future combo instrument type
pub const INSTRUMENT_TYPE_FUTURE_COMBO: &str = "future_combo";

/// Option combo instrument type
pub const INSTRUMENT_TYPE_OPTION_COMBO: &str = "option_combo";

// =============================================================================
// ORDER LIMITS
// =============================================================================

/// Minimum order amount for BTC
pub const MIN_ORDER_AMOUNT_BTC: f64 = 0.0001;

/// Minimum order amount for ETH
pub const MIN_ORDER_AMOUNT_ETH: f64 = 0.001;

/// Minimum order amount for SOL
pub const MIN_ORDER_AMOUNT_SOL: f64 = 0.1;

/// Maximum order amount (no specific limit, but practical limit)
pub const MAX_ORDER_AMOUNT: f64 = 1_000_000.0;

/// Maximum number of open orders per instrument
pub const MAX_OPEN_ORDERS_PER_INSTRUMENT: u32 = 500;

/// Maximum number of open orders total
pub const MAX_OPEN_ORDERS_TOTAL: u32 = 2000;

// =============================================================================
// PRECISION
// =============================================================================

/// Price precision for BTC instruments (8 decimal places)
pub const PRICE_PRECISION_BTC: u8 = 8;

/// Price precision for ETH instruments (4 decimal places)
pub const PRICE_PRECISION_ETH: u8 = 4;

/// Price precision for SOL instruments (4 decimal places)
pub const PRICE_PRECISION_SOL: u8 = 4;

/// Amount precision for BTC (4 decimal places)
pub const AMOUNT_PRECISION_BTC: u8 = 4;

/// Amount precision for ETH (3 decimal places)
pub const AMOUNT_PRECISION_ETH: u8 = 3;

/// Amount precision for SOL (1 decimal place)
pub const AMOUNT_PRECISION_SOL: u8 = 1;

// =============================================================================
// JSON-RPC
// =============================================================================

/// JSON-RPC version
pub const JSONRPC_VERSION: &str = "2.0";

/// Default JSON-RPC request ID
pub const DEFAULT_REQUEST_ID: u64 = 1;

// =============================================================================
// WEBSOCKET CHANNELS
// =============================================================================

/// Book channel prefix
pub const CHANNEL_BOOK: &str = "book";

/// Trades channel prefix
pub const CHANNEL_TRADES: &str = "trades";

/// Ticker channel prefix
pub const CHANNEL_TICKER: &str = "ticker";

/// Quote channel prefix
pub const CHANNEL_QUOTE: &str = "quote";

/// User orders channel
pub const CHANNEL_USER_ORDERS: &str = "user.orders";

/// User trades channel
pub const CHANNEL_USER_TRADES: &str = "user.trades";

/// User portfolio channel
pub const CHANNEL_USER_PORTFOLIO: &str = "user.portfolio";

// =============================================================================
// FIX PROTOCOL
// =============================================================================

/// FIX version 4.4
pub const FIX_VERSION: &str = "FIX.4.4";

/// FIX message delimiter (SOH - Start of Header)
pub const FIX_DELIMITER: char = '\x01';

/// FIX message delimiter as string
pub const FIX_DELIMITER_STR: &str = "\x01";

/// Default FIX heartbeat interval in seconds
pub const FIX_HEARTBEAT_INTERVAL: u32 = 30;

// =============================================================================
// ERROR HANDLING
// =============================================================================

/// Maximum retry attempts for failed requests
pub const MAX_RETRY_ATTEMPTS: u8 = 3;

/// Base delay for exponential backoff in milliseconds
pub const RETRY_BASE_DELAY_MS: u64 = 1000;

/// Maximum delay for exponential backoff in milliseconds
pub const RETRY_MAX_DELAY_MS: u64 = 30000;

// =============================================================================
// MARKET DATA
// =============================================================================

/// Maximum depth levels for order book
pub const MAX_ORDER_BOOK_DEPTH: u32 = 10000;

/// Default order book depth
pub const DEFAULT_ORDER_BOOK_DEPTH: u32 = 20;

/// Maximum number of recent trades to fetch
pub const MAX_RECENT_TRADES: u32 = 10000;

/// Default number of recent trades
pub const DEFAULT_RECENT_TRADES: u32 = 100;

// =============================================================================
// AUTHENTICATION
// =============================================================================

/// Access token expiration time in seconds (8 hours)
pub const ACCESS_TOKEN_EXPIRATION_SEC: u64 = 28800;

/// Refresh token expiration time in seconds (30 days)
pub const REFRESH_TOKEN_EXPIRATION_SEC: u64 = 2592000;

/// Minimum time before token expiration to refresh (5 minutes)
pub const TOKEN_REFRESH_BUFFER_SEC: u64 = 300;

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Get minimum order amount for a given currency
pub fn get_min_order_amount(currency: &str) -> f64 {
    match currency {
        CURRENCY_BTC => MIN_ORDER_AMOUNT_BTC,
        CURRENCY_ETH => MIN_ORDER_AMOUNT_ETH,
        CURRENCY_SOL => MIN_ORDER_AMOUNT_SOL,
        _ => MIN_ORDER_AMOUNT_BTC, // Default to BTC
    }
}

/// Get price precision for a given currency
pub fn get_price_precision(currency: &str) -> u8 {
    match currency {
        CURRENCY_BTC => PRICE_PRECISION_BTC,
        CURRENCY_ETH => PRICE_PRECISION_ETH,
        CURRENCY_SOL => PRICE_PRECISION_SOL,
        _ => PRICE_PRECISION_BTC, // Default to BTC
    }
}

/// Get amount precision for a given currency
pub fn get_amount_precision(currency: &str) -> u8 {
    match currency {
        CURRENCY_BTC => AMOUNT_PRECISION_BTC,
        CURRENCY_ETH => AMOUNT_PRECISION_ETH,
        CURRENCY_SOL => AMOUNT_PRECISION_SOL,
        _ => AMOUNT_PRECISION_BTC, // Default to BTC
    }
}

/// Check if a currency is supported
pub fn is_supported_currency(currency: &str) -> bool {
    matches!(
        currency,
        CURRENCY_BTC | CURRENCY_ETH | CURRENCY_SOL | CURRENCY_USDC | CURRENCY_USDT | CURRENCY_EURR
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_urls() {
        assert!(DERIBIT_WS_URL_PROD.starts_with("wss://"));
        assert!(DERIBIT_WS_URL_TEST.starts_with("wss://"));
        assert!(DERIBIT_HTTP_URL_PROD.starts_with("https://"));
        assert!(DERIBIT_HTTP_URL_TEST.starts_with("https://"));

        assert!(DERIBIT_WS_URL_PROD.contains("www.deribit.com"));
        assert!(DERIBIT_WS_URL_TEST.contains("test.deribit.com"));
    }

    #[test]
    fn test_supported_currencies() {
        assert!(is_supported_currency(CURRENCY_BTC));
        assert!(is_supported_currency(CURRENCY_ETH));
        assert!(is_supported_currency(CURRENCY_SOL));
        assert!(is_supported_currency(CURRENCY_USDC));
        assert!(is_supported_currency(CURRENCY_USDT));
        assert!(is_supported_currency(CURRENCY_EURR));

        assert!(!is_supported_currency("INVALID"));
        assert!(!is_supported_currency("XRP"));
    }

    #[test]
    fn test_min_order_amounts() {
        assert_eq!(get_min_order_amount(CURRENCY_BTC), MIN_ORDER_AMOUNT_BTC);
        assert_eq!(get_min_order_amount(CURRENCY_ETH), MIN_ORDER_AMOUNT_ETH);
        assert_eq!(get_min_order_amount(CURRENCY_SOL), MIN_ORDER_AMOUNT_SOL);

        // Test default fallback
        assert_eq!(get_min_order_amount("UNKNOWN"), MIN_ORDER_AMOUNT_BTC);
    }

    #[test]
    fn test_precision_functions() {
        assert_eq!(get_price_precision(CURRENCY_BTC), PRICE_PRECISION_BTC);
        assert_eq!(get_price_precision(CURRENCY_ETH), PRICE_PRECISION_ETH);
        assert_eq!(get_price_precision(CURRENCY_SOL), PRICE_PRECISION_SOL);

        assert_eq!(get_amount_precision(CURRENCY_BTC), AMOUNT_PRECISION_BTC);
        assert_eq!(get_amount_precision(CURRENCY_ETH), AMOUNT_PRECISION_ETH);
        assert_eq!(get_amount_precision(CURRENCY_SOL), AMOUNT_PRECISION_SOL);
    }

    #[test]
    fn test_rate_limits() {
        assert!(MAX_REQUESTS_PER_SECOND_AUTH > MAX_REQUESTS_PER_SECOND_UNAUTH);
        assert!(MAX_SUBSCRIPTIONS_PER_CONNECTION > 0);
        assert!(MAX_MESSAGE_SIZE_BYTES > 0);
    }

    #[test]
    fn test_timeouts() {
        assert!(DEFAULT_CONNECTION_TIMEOUT_MS > 0);
        assert!(DEFAULT_REQUEST_TIMEOUT_MS > DEFAULT_CONNECTION_TIMEOUT_MS);
        assert!(HEARTBEAT_INTERVAL_MS > HEARTBEAT_TIMEOUT_MS);
    }

    #[test]
    fn test_order_limits() {
        assert!(MIN_ORDER_AMOUNT_BTC > 0.0);
        assert!(MIN_ORDER_AMOUNT_ETH > 0.0);
        assert!(MIN_ORDER_AMOUNT_SOL > 0.0);
        assert!(MAX_ORDER_AMOUNT > MIN_ORDER_AMOUNT_BTC);
        assert!(MAX_OPEN_ORDERS_TOTAL > MAX_OPEN_ORDERS_PER_INSTRUMENT);
    }

    #[test]
    fn test_jsonrpc_constants() {
        assert_eq!(JSONRPC_VERSION, "2.0");
        assert!(DEFAULT_REQUEST_ID > 0);
    }

    #[test]
    fn test_fix_constants() {
        assert_eq!(FIX_VERSION, "FIX.4.4");
        assert_eq!(FIX_DELIMITER, '\x01');
        assert_eq!(FIX_DELIMITER_STR, "\x01");
        assert!(FIX_HEARTBEAT_INTERVAL > 0);
    }

    #[test]
    fn test_channel_names() {
        assert!(!CHANNEL_BOOK.is_empty());
        assert!(!CHANNEL_TRADES.is_empty());
        assert!(!CHANNEL_TICKER.is_empty());
        assert!(!CHANNEL_QUOTE.is_empty());
        assert!(CHANNEL_USER_ORDERS.starts_with("user."));
        assert!(CHANNEL_USER_TRADES.starts_with("user."));
        assert!(CHANNEL_USER_PORTFOLIO.starts_with("user."));
    }

    #[test]
    fn test_instrument_types() {
        let types = [
            INSTRUMENT_TYPE_FUTURE,
            INSTRUMENT_TYPE_OPTION,
            INSTRUMENT_TYPE_PERPETUAL,
            INSTRUMENT_TYPE_SPOT,
            INSTRUMENT_TYPE_FUTURE_COMBO,
            INSTRUMENT_TYPE_OPTION_COMBO,
        ];

        for instrument_type in types {
            assert!(!instrument_type.is_empty());
        }
    }

    #[test]
    fn test_authentication_constants() {
        assert!(ACCESS_TOKEN_EXPIRATION_SEC > 0);
        assert!(REFRESH_TOKEN_EXPIRATION_SEC > ACCESS_TOKEN_EXPIRATION_SEC);
        assert!(TOKEN_REFRESH_BUFFER_SEC < ACCESS_TOKEN_EXPIRATION_SEC);
    }

    #[test]
    fn test_market_data_constants() {
        assert!(MAX_ORDER_BOOK_DEPTH > DEFAULT_ORDER_BOOK_DEPTH);
        assert!(MAX_RECENT_TRADES > DEFAULT_RECENT_TRADES);
        assert!(DEFAULT_ORDER_BOOK_DEPTH > 0);
        assert!(DEFAULT_RECENT_TRADES > 0);
    }

    #[test]
    fn test_error_handling_constants() {
        assert!(MAX_RETRY_ATTEMPTS > 0);
        assert!(RETRY_BASE_DELAY_MS > 0);
        assert!(RETRY_MAX_DELAY_MS > RETRY_BASE_DELAY_MS);
    }
}
