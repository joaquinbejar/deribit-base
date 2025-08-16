/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Deribit API configuration
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
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

// Debug implementations using pretty JSON formatting
impl_json_debug_pretty!(
    DeribitConfig,    WebSocketConfig,    HttpConfig
);

// Display implementations using compact JSON formatting
impl_json_display!(
    DeribitConfig,    WebSocketConfig,    HttpConfig
);
