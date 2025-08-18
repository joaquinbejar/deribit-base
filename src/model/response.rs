/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

use crate::model::trade::{LastTrade, TradeExecution};
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};
use crate::model::order::OrderInfo;
use crate::model::order_management::QuoteResult;

/// Generic JSON-RPC 2.0 response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Request ID
    pub id: Option<serde_json::Value>,
    /// Result data (present on success)
    pub result: Option<T>,
    /// Error information (present on error)
    pub error: Option<JsonRpcError>,
    /// Test net flag
    pub testnet: Option<bool>,
    /// Use server time
    #[serde(rename = "usIn")]
    pub us_in: Option<i64>,
    /// Use out time
    #[serde(rename = "usOut")]
    pub us_out: Option<i64>,
    /// Use diff time
    #[serde(rename = "usDiff")]
    pub us_diff: Option<i64>,
}

impl<T> JsonRpcResponse<T> {
    /// Create a successful response
    pub fn success(id: Option<serde_json::Value>, result: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
            testnet: None,
            us_in: None,
            us_out: None,
            us_diff: None,
        }
    }

    /// Create an error response
    pub fn error(id: Option<serde_json::Value>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
            testnet: None,
            us_in: None,
            us_out: None,
            us_diff: None,
        }
    }

    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        self.error.is_none() && self.result.is_some()
    }

    /// Check if the response is an error
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get the result, consuming the response
    pub fn into_result(self) -> Result<T, JsonRpcError> {
        match (self.result, self.error) {
            (Some(result), None) => Ok(result),
            (None, Some(error)) => Err(error),
            (Some(_), Some(error)) => Err(error), // Error takes precedence
            (None, None) => Err(JsonRpcError {
                code: -32603,
                message: "Internal error: neither result nor error present".to_string(),
                data: None,
            }),
        }
    }
}

/// JSON-RPC error information
#[derive(Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Create a new JSON-RPC error
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    /// Create an error with additional data
    pub fn with_data(code: i32, message: String, data: serde_json::Value) -> Self {
        Self {
            code,
            message,
            data: Some(data),
        }
    }

    /// Parse error (-32700)
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error".to_string())
    }

    /// Invalid request (-32600)
    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid Request".to_string())
    }

    /// Method not found (-32601)
    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found".to_string())
    }

    /// Invalid params (-32602)
    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params".to_string())
    }

    /// Internal error (-32603)
    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error".to_string())
    }

    /// Check if this is a server error (code between -32099 and -32000)
    pub fn is_server_error(&self) -> bool {
        self.code <= -32000 && self.code >= -32099
    }

    /// Check if this is an application error (code > -32000)
    pub fn is_application_error(&self) -> bool {
        self.code > -32000
    }
}

/// Authentication response
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Access token
    pub access_token: String,
    /// Token type (usually "bearer")
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: i64,
    /// Refresh token
    pub refresh_token: String,
    /// Scope
    pub scope: String,
}

/// Pagination information
#[derive(Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page
    pub page: Option<u32>,
    /// Items per page
    pub per_page: Option<u32>,
    /// Total items
    pub total: Option<u64>,
    /// Total pages
    pub pages: Option<u32>,
    /// Has more pages
    pub has_more: Option<bool>,
}

/// Generic paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Data items
    pub data: Vec<T>,
    /// Pagination information
    pub pagination: Option<Pagination>,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            pagination: None,
        }
    }

    /// Create a paginated response with pagination info
    pub fn with_pagination(data: Vec<T>, pagination: Pagination) -> Self {
        Self {
            data,
            pagination: Some(pagination),
        }
    }

    /// Check if there are more pages
    pub fn has_more(&self) -> bool {
        self.pagination
            .as_ref()
            .and_then(|p| p.has_more)
            .unwrap_or(false)
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the response is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// WebSocket notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification<T> {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters/data
    pub params: T,
}

impl<T> Notification<T> {
    /// Create a new notification
    pub fn new(method: String, params: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
        }
    }
}

/// Subscription response
#[derive(Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    /// Subscription ID
    pub subscription: String,
    /// Channel name
    pub channel: String,
}

/// Heartbeat response
#[derive(Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    /// Type (always "heartbeat")
    #[serde(rename = "type")]
    pub type_: String,
}

/// Test response for connectivity checks
#[derive(Clone, Serialize, Deserialize)]
pub struct TestResponse {
    /// Version information
    pub version: String,
}

/// Server time response
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerTimeResponse {
    /// Current server timestamp in milliseconds
    pub timestamp: i64,
}

/// Settlements response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct SettlementsResponse {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of settlement events
    pub settlements: Vec<crate::model::settlement::Settlement>,
}

impl SettlementsResponse {
    /// Create a new settlements response
    pub fn new(settlements: Vec<crate::model::settlement::Settlement>) -> Self {
        Self {
            continuation: None,
            settlements,
        }
    }

    /// Create settlements response with continuation token
    pub fn with_continuation(
        settlements: Vec<crate::model::settlement::Settlement>,
        continuation: String,
    ) -> Self {
        Self {
            continuation: Some(continuation),
            settlements,
        }
    }

    /// Check if there are more results
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }
}

/// Contract size response
#[derive(Clone, Serialize, Deserialize)]
pub struct ContractSizeResponse {
    pub contract_size: f64,
}

/// Status response
#[derive(Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub locked: bool,
    pub message: String,
    pub locked_indices: Vec<String>,
}

/// APR history response
#[derive(Clone, Serialize, Deserialize)]
pub struct AprHistoryResponse {
    pub data: Vec<AprDataPoint>,
    pub continuation: Option<String>,
}
/// APR data point
#[derive(Clone, Serialize, Deserialize)]
pub struct AprDataPoint {
    pub apr: f64,
    pub timestamp: u64,
    pub day: i32,
}

/// Hello response
#[derive(Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    pub version: String,
}

/// Delivery prices response
#[derive(Clone, Serialize, Deserialize)]
pub struct DeliveryPricesResponse {
    pub data: Vec<DeliveryPriceData>,
    pub records_total: u32,
}

/// Delivery price data
#[derive(Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    pub date: String,
    pub delivery_price: f64,
}

/// Expirations response
#[derive(Clone, Serialize, Deserialize)]
pub struct ExpirationsResponse {
    pub expirations: Vec<String>,
    pub future: Option<Vec<String>>,
    pub option: Option<Vec<String>>,
}

/// Last trades response
#[derive(Clone, Serialize, Deserialize)]
pub struct LastTradesResponse {
    pub has_more: bool,
    pub trades: Vec<LastTrade>,
}

/// Order response
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: OrderInfo,
    pub trades: Vec<TradeExecution>,
}

/// Mass quote response
#[derive(Clone, Serialize, Deserialize)]
pub struct MassQuoteResponse {
    pub quotes: Vec<QuoteResult>,
}


impl_json_display!(
    MassQuoteResponse,
    JsonRpcError,
    AuthResponse,
    Pagination,
    SubscriptionResponse,
    HeartbeatResponse,
    TestResponse,
    ServerTimeResponse,
    SettlementsResponse,
    ContractSizeResponse,
    StatusResponse,
    AprHistoryResponse,
    AprDataPoint,
    HelloResponse,
    DeliveryPricesResponse,
    DeliveryPriceData,
    ExpirationsResponse,
    LastTradesResponse,
    OrderResponse
);
impl_json_debug_pretty!(
    MassQuoteResponse,
    JsonRpcError,
    AuthResponse,
    Pagination,
    SubscriptionResponse,
    HeartbeatResponse,
    TestResponse,
    ServerTimeResponse,
    SettlementsResponse,
    ContractSizeResponse,
    StatusResponse,
    AprHistoryResponse,
    AprDataPoint,
    HelloResponse,
    DeliveryPricesResponse,
    DeliveryPriceData,
    ExpirationsResponse,
    LastTradesResponse,
    OrderResponse
);

impl std::error::Error for JsonRpcError {}
