/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Deribit API configuration
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeribitConfig {
    /// Client ID for API authentication
    pub client_id: String,
    /// Client secret for API authentication
    pub client_secret: String,
    /// Whether to use testnet
    pub test_net: bool,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Rate limit per second
    pub rate_limit: Option<u32>,
    /// User agent string
    pub user_agent: Option<String>,
}

impl DeribitConfig {
    /// Create a new configuration for production
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            test_net: false,
            timeout_seconds: 30,
            max_retries: 3,
            rate_limit: None,
            user_agent: None,
        }
    }

    /// Create a new configuration for testnet
    pub fn testnet(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            test_net: true,
            timeout_seconds: 30,
            max_retries: 3,
            rate_limit: None,
            user_agent: None,
        }
    }

    /// Set timeout in seconds
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set rate limit
    pub fn with_rate_limit(mut self, rate_limit: u32) -> Self {
        self.rate_limit = Some(rate_limit);
        self
    }

    /// Set user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Get the base URL for HTTP API
    pub fn base_url(&self) -> &'static str {
        if self.test_net {
            DeribitUrls::TEST_BASE_URL
        } else {
            DeribitUrls::PROD_BASE_URL
        }
    }

    /// Get the WebSocket URL
    pub fn ws_url(&self) -> &'static str {
        if self.test_net {
            DeribitUrls::TEST_WS_URL
        } else {
            DeribitUrls::PROD_WS_URL
        }
    }

    /// Get the API URL for HTTP requests
    pub fn api_url(&self) -> String {
        format!("{}/api/v2", self.base_url())
    }
}

impl Default for DeribitConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            test_net: true, // Default to testnet for safety
            timeout_seconds: 30,
            max_retries: 3,
            rate_limit: None,
            user_agent: Some("deribit-rust-client/1.0".to_string()),
        }
    }
}

/// Deribit API URLs
pub struct DeribitUrls;

impl DeribitUrls {
    /// Production base URL
    pub const PROD_BASE_URL: &'static str = "https://www.deribit.com";
    /// Test base URL
    pub const TEST_BASE_URL: &'static str = "https://test.deribit.com";
    /// Production WebSocket URL
    pub const PROD_WS_URL: &'static str = "wss://www.deribit.com/ws/api/v2";
    /// Test WebSocket URL
    pub const TEST_WS_URL: &'static str = "wss://test.deribit.com/ws/api/v2";
}

/// Connection configuration for WebSocket
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Base configuration
    pub base: DeribitConfig,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Pong timeout in seconds
    pub pong_timeout: u64,
    /// Reconnect attempts
    pub reconnect_attempts: u32,
    /// Reconnect delay in seconds
    pub reconnect_delay: u64,
    /// Maximum message size
    pub max_message_size: usize,
    /// Enable compression
    pub compression: bool,
}

impl WebSocketConfig {
    /// Create new WebSocket configuration
    pub fn new(base: DeribitConfig) -> Self {
        Self {
            base,
            ping_interval: 30,
            pong_timeout: 10,
            reconnect_attempts: 5,
            reconnect_delay: 5,
            max_message_size: 1024 * 1024, // 1MB
            compression: true,
        }
    }

    /// Set ping interval
    pub fn with_ping_interval(mut self, ping_interval: u64) -> Self {
        self.ping_interval = ping_interval;
        self
    }

    /// Set pong timeout
    pub fn with_pong_timeout(mut self, pong_timeout: u64) -> Self {
        self.pong_timeout = pong_timeout;
        self
    }

    /// Set reconnect attempts
    pub fn with_reconnect_attempts(mut self, reconnect_attempts: u32) -> Self {
        self.reconnect_attempts = reconnect_attempts;
        self
    }

    /// Set reconnect delay
    pub fn with_reconnect_delay(mut self, reconnect_delay: u64) -> Self {
        self.reconnect_delay = reconnect_delay;
        self
    }

    /// Enable/disable compression
    pub fn with_compression(mut self, compression: bool) -> Self {
        self.compression = compression;
        self
    }
}

/// HTTP client configuration
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Base configuration
    pub base: DeribitConfig,
    /// Connection pool size
    pub pool_size: Option<usize>,
    /// Keep alive timeout
    pub keep_alive: Option<u64>,
    /// Enable HTTP/2
    pub http2: bool,
    /// Enable gzip compression
    pub gzip: bool,
}

impl HttpConfig {
    /// Create new HTTP configuration
    pub fn new(base: DeribitConfig) -> Self {
        Self {
            base,
            pool_size: None,
            keep_alive: Some(30),
            http2: true,
            gzip: true,
        }
    }

    /// Set connection pool size
    pub fn with_pool_size(mut self, pool_size: usize) -> Self {
        self.pool_size = Some(pool_size);
        self
    }

    /// Set keep alive timeout
    pub fn with_keep_alive(mut self, keep_alive: u64) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Enable/disable HTTP/2
    pub fn with_http2(mut self, http2: bool) -> Self {
        self.http2 = http2;
        self
    }

    /// Enable/disable gzip compression
    pub fn with_gzip(mut self, gzip: bool) -> Self {
        self.gzip = gzip;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deribit_config_new() {
        let config = DeribitConfig::new("client123".to_string(), "secret456".to_string());
        assert_eq!(config.client_id, "client123");
        assert_eq!(config.client_secret, "secret456");
        assert!(!config.test_net);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.rate_limit, None);
        assert_eq!(config.user_agent, None);
    }

    #[test]
    fn test_deribit_config_testnet() {
        let config = DeribitConfig::testnet("client123".to_string(), "secret456".to_string());
        assert_eq!(config.client_id, "client123");
        assert_eq!(config.client_secret, "secret456");
        assert!(config.test_net);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.rate_limit, None);
        assert_eq!(config.user_agent, None);
    }

    #[test]
    fn test_deribit_config_with_timeout() {
        let config =
            DeribitConfig::new("client".to_string(), "secret".to_string()).with_timeout(60);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_deribit_config_with_max_retries() {
        let config =
            DeribitConfig::new("client".to_string(), "secret".to_string()).with_max_retries(5);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_deribit_config_with_rate_limit() {
        let config =
            DeribitConfig::new("client".to_string(), "secret".to_string()).with_rate_limit(10);
        assert_eq!(config.rate_limit, Some(10));
    }

    #[test]
    fn test_deribit_config_with_user_agent() {
        let config = DeribitConfig::new("client".to_string(), "secret".to_string())
            .with_user_agent("custom-agent".to_string());
        assert_eq!(config.user_agent, Some("custom-agent".to_string()));
    }

    #[test]
    fn test_deribit_config_base_url() {
        let prod_config = DeribitConfig::new("client".to_string(), "secret".to_string());
        assert_eq!(prod_config.base_url(), DeribitUrls::PROD_BASE_URL);

        let test_config = DeribitConfig::testnet("client".to_string(), "secret".to_string());
        assert_eq!(test_config.base_url(), DeribitUrls::TEST_BASE_URL);
    }

    #[test]
    fn test_deribit_config_ws_url() {
        let prod_config = DeribitConfig::new("client".to_string(), "secret".to_string());
        assert_eq!(prod_config.ws_url(), DeribitUrls::PROD_WS_URL);

        let test_config = DeribitConfig::testnet("client".to_string(), "secret".to_string());
        assert_eq!(test_config.ws_url(), DeribitUrls::TEST_WS_URL);
    }

    #[test]
    fn test_deribit_config_api_url() {
        let prod_config = DeribitConfig::new("client".to_string(), "secret".to_string());
        assert_eq!(prod_config.api_url(), "https://www.deribit.com/api/v2");

        let test_config = DeribitConfig::testnet("client".to_string(), "secret".to_string());
        assert_eq!(test_config.api_url(), "https://test.deribit.com/api/v2");
    }

    #[test]
    fn test_deribit_config_default() {
        let config = DeribitConfig::default();
        assert_eq!(config.client_id, "");
        assert_eq!(config.client_secret, "");
        assert!(config.test_net);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.rate_limit, None);
        assert_eq!(
            config.user_agent,
            Some("deribit-rust-client/1.0".to_string())
        );
    }

    #[test]
    fn test_deribit_urls_constants() {
        assert_eq!(DeribitUrls::PROD_BASE_URL, "https://www.deribit.com");
        assert_eq!(DeribitUrls::TEST_BASE_URL, "https://test.deribit.com");
        assert_eq!(DeribitUrls::PROD_WS_URL, "wss://www.deribit.com/ws/api/v2");
        assert_eq!(DeribitUrls::TEST_WS_URL, "wss://test.deribit.com/ws/api/v2");
    }

    #[test]
    fn test_websocket_config_new() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base.clone());

        assert_eq!(ws_config.base.client_id, base.client_id);
        assert_eq!(ws_config.ping_interval, 30);
        assert_eq!(ws_config.pong_timeout, 10);
        assert_eq!(ws_config.reconnect_attempts, 5);
        assert_eq!(ws_config.reconnect_delay, 5);
        assert_eq!(ws_config.max_message_size, 1024 * 1024);
        assert!(ws_config.compression);
    }

    #[test]
    fn test_websocket_config_with_ping_interval() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base).with_ping_interval(60);
        assert_eq!(ws_config.ping_interval, 60);
    }

    #[test]
    fn test_websocket_config_with_pong_timeout() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base).with_pong_timeout(20);
        assert_eq!(ws_config.pong_timeout, 20);
    }

    #[test]
    fn test_websocket_config_with_reconnect_attempts() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base).with_reconnect_attempts(10);
        assert_eq!(ws_config.reconnect_attempts, 10);
    }

    #[test]
    fn test_websocket_config_with_reconnect_delay() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base).with_reconnect_delay(15);
        assert_eq!(ws_config.reconnect_delay, 15);
    }

    #[test]
    fn test_websocket_config_with_compression() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base).with_compression(false);
        assert!(!ws_config.compression);
    }

    #[test]
    fn test_http_config_new() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base.clone());

        assert_eq!(http_config.base.client_id, base.client_id);
        assert_eq!(http_config.pool_size, None);
        assert_eq!(http_config.keep_alive, Some(30));
        assert!(http_config.http2);
        assert!(http_config.gzip);
    }

    #[test]
    fn test_http_config_with_pool_size() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base).with_pool_size(20);
        assert_eq!(http_config.pool_size, Some(20));
    }

    #[test]
    fn test_http_config_with_keep_alive() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base).with_keep_alive(60);
        assert_eq!(http_config.keep_alive, Some(60));
    }

    #[test]
    fn test_http_config_with_http2() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base).with_http2(false);
        assert!(!http_config.http2);
    }

    #[test]
    fn test_http_config_with_gzip() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base).with_gzip(false);
        assert!(!http_config.gzip);
    }

    #[test]
    fn test_config_serialization() {
        let config = DeribitConfig::new("client".to_string(), "secret".to_string());
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DeribitConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.client_id, deserialized.client_id);
        assert_eq!(config.client_secret, deserialized.client_secret);
    }

    #[test]
    fn test_websocket_config_serialization() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let ws_config = WebSocketConfig::new(base);
        let json = serde_json::to_string(&ws_config).unwrap();
        let deserialized: WebSocketConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(ws_config.ping_interval, deserialized.ping_interval);
    }

    #[test]
    fn test_http_config_serialization() {
        let base = DeribitConfig::new("client".to_string(), "secret".to_string());
        let http_config = HttpConfig::new(base);
        let json = serde_json::to_string(&http_config).unwrap();
        let deserialized: HttpConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(http_config.http2, deserialized.http2);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let config = DeribitConfig::new("client".to_string(), "secret".to_string());
        let debug_str = format!("{:?}", config);
        let display_str = format!("{}", config);

        assert!(debug_str.contains("client"));
        assert!(display_str.contains("client"));
    }
}
