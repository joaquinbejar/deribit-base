//! Common functionality shared across all Deribit clients

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Common configuration for Deribit clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeribitConfig {
    pub client_id: String,
    pub client_secret: String,
    pub test_net: bool,
    pub timeout_seconds: u64,
}

impl Default for DeribitConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            test_net: true,
            timeout_seconds: 30,
        }
    }
}

/// Base URL configuration
pub struct DeribitUrls;

impl DeribitUrls {
    pub const PROD_BASE_URL: &'static str = "https://www.deribit.com";
    pub const TEST_BASE_URL: &'static str = "https://test.deribit.com";

    pub const PROD_WS_URL: &'static str = "wss://www.deribit.com/ws/api/v2";
    pub const TEST_WS_URL: &'static str = "wss://test.deribit.com/ws/api/v2";

    pub fn get_base_url(test_net: bool) -> &'static str {
        if test_net {
            Self::TEST_BASE_URL
        } else {
            Self::PROD_BASE_URL
        }
    }

    pub fn get_ws_url(test_net: bool) -> &'static str {
        if test_net {
            Self::TEST_WS_URL
        } else {
            Self::PROD_WS_URL
        }
    }
}

/// Common trait for all Deribit clients
#[async_trait]
pub trait DeribitClient {
    type Error;

    /// Connect to Deribit
    async fn connect(&mut self) -> Result<(), Self::Error>;

    /// Disconnect from Deribit
    async fn disconnect(&mut self) -> Result<(), Self::Error>;

    /// Check if connected
    fn is_connected(&self) -> bool;
}
