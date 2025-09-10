/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::order::OrderInfo;
use crate::model::order_management::QuoteResult;
use crate::model::trade::{LastTrade, TradeExecution};
use pretty_simple_display::{DebugPretty, DisplaySimple};

use serde::{Deserialize, Serialize};

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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    /// Subscription ID
    pub subscription: String,
    /// Channel name
    pub channel: String,
}

/// Heartbeat response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    /// Type (always "heartbeat")
    #[serde(rename = "type")]
    pub type_: String,
}

/// Test response for connectivity checks
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TestResponse {
    /// Version information
    pub version: String,
}

/// Server time response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ServerTimeResponse {
    /// Current server timestamp in milliseconds
    pub timestamp: i64,
}

/// Settlements response structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ContractSizeResponse {
    /// Contract size value
    pub contract_size: f64,
}

/// Status response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Whether the system is locked (optional)
    pub locked: Option<bool>,
    /// Status message (optional)
    pub message: Option<String>,
    /// List of locked indices (optional)
    pub locked_indices: Option<Vec<String>>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// APR history response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AprHistoryResponse {
    /// List of APR data points
    pub data: Vec<AprDataPoint>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}
/// APR data point
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AprDataPoint {
    /// Annual percentage rate
    pub apr: f64,
    /// Timestamp of the data point (optional)
    pub timestamp: Option<u64>,
    /// Day of the data point
    pub day: i32,
}

/// Hello response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    /// Version string
    pub version: String,
}

/// Delivery prices response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPricesResponse {
    /// List of delivery price data
    pub data: Vec<DeliveryPriceData>,
    /// Total number of records
    pub records_total: u32,
}

/// Delivery price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    /// Date of the delivery price
    pub date: String,
    /// Delivery price value
    pub delivery_price: f64,
}

/// Currency-specific expirations
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CurrencyExpirations {
    /// Future instrument expirations
    pub future: Option<Vec<String>>,
    /// Option instrument expirations
    pub option: Option<Vec<String>>,
}

/// Expirations response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ExpirationsResponse {
    /// Direct future expirations (when currency="any")
    pub future: Option<Vec<String>>,
    /// Direct option expirations (when currency="any")
    pub option: Option<Vec<String>>,
    /// Map of currency to their expirations (when specific currency)
    #[serde(flatten)]
    pub currencies: std::collections::HashMap<String, CurrencyExpirations>,
}

/// Last trades response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct LastTradesResponse {
    /// Whether there are more trades available
    pub has_more: bool,
    /// List of recent trades
    pub trades: Vec<LastTrade>,
}

/// Order response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Order information
    pub order: OrderInfo,
    /// List of trade executions for the order
    pub trades: Vec<TradeExecution>,
}

/// Mass quote response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteResponse {
    /// List of quote results
    pub quotes: Vec<QuoteResult>,
}

impl std::error::Error for JsonRpcError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_rpc_response_success() {
        let response = JsonRpcResponse::success(Some(json!(1)), "test_result".to_string());
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Some(json!(1)));
        assert_eq!(response.result, Some("test_result".to_string()));
        assert_eq!(response.error, None);
        assert!(response.is_success());
        assert!(!response.is_error());
    }

    #[test]
    fn test_json_rpc_response_error() {
        let error = JsonRpcError::new(-32600, "Invalid Request".to_string());
        let response: JsonRpcResponse<String> =
            JsonRpcResponse::error(Some(json!(1)), error.clone());

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Some(json!(1)));
        assert_eq!(response.result, None);
        assert!(response.error.is_some());
        assert!(!response.is_success());
        assert!(response.is_error());
    }

    #[test]
    fn test_json_rpc_response_into_result_success() {
        let response = JsonRpcResponse::success(Some(json!(1)), "test_result".to_string());
        let result = response.into_result().unwrap();
        assert_eq!(result, "test_result");
    }

    #[test]
    fn test_json_rpc_response_into_result_error() {
        let error = JsonRpcError::new(-32600, "Invalid Request".to_string());
        let response: JsonRpcResponse<String> =
            JsonRpcResponse::error(Some(json!(1)), error.clone());
        let result = response.into_result();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, -32600);
        assert_eq!(err.message, "Invalid Request");
    }

    #[test]
    fn test_json_rpc_response_into_result_neither() {
        let response: JsonRpcResponse<String> = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            result: None,
            error: None,
            testnet: None,
            us_in: None,
            us_out: None,
            us_diff: None,
        };
        let result = response.into_result();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, -32603);
        assert!(err.message.contains("Internal error"));
    }

    #[test]
    fn test_json_rpc_error_new() {
        let error = JsonRpcError::new(-32600, "Invalid Request".to_string());
        assert_eq!(error.code, -32600);
        assert_eq!(error.message, "Invalid Request");
        assert_eq!(error.data, None);
    }

    #[test]
    fn test_json_rpc_error_with_data() {
        let data = json!({"details": "Additional error information"});
        let error = JsonRpcError::with_data(-32602, "Invalid params".to_string(), data.clone());
        assert_eq!(error.code, -32602);
        assert_eq!(error.message, "Invalid params");
        assert_eq!(error.data, Some(data));
    }

    #[test]
    fn test_json_rpc_error_standard_errors() {
        let parse_error = JsonRpcError::parse_error();
        assert_eq!(parse_error.code, -32700);
        assert_eq!(parse_error.message, "Parse error");

        let invalid_request = JsonRpcError::invalid_request();
        assert_eq!(invalid_request.code, -32600);
        assert_eq!(invalid_request.message, "Invalid Request");

        let method_not_found = JsonRpcError::method_not_found();
        assert_eq!(method_not_found.code, -32601);
        assert_eq!(method_not_found.message, "Method not found");

        let invalid_params = JsonRpcError::invalid_params();
        assert_eq!(invalid_params.code, -32602);
        assert_eq!(invalid_params.message, "Invalid params");

        let internal_error = JsonRpcError::internal_error();
        assert_eq!(internal_error.code, -32603);
        assert_eq!(internal_error.message, "Internal error");
    }

    #[test]
    fn test_json_rpc_error_is_server_error() {
        let server_error = JsonRpcError::new(-32001, "Server error".to_string());
        assert!(server_error.is_server_error());
        assert!(!server_error.is_application_error());

        let app_error = JsonRpcError::new(-31999, "Application error".to_string());
        assert!(!app_error.is_server_error());
        assert!(app_error.is_application_error());
    }

    #[test]
    fn test_json_rpc_error_display() {
        let error = JsonRpcError::new(-32600, "Invalid Request".to_string());
        let display_str = format!("{}", error);
        // The display implementation uses JSON formatting from the macro
        assert!(display_str.contains("-32600"));
        assert!(display_str.contains("Invalid Request"));
    }

    #[test]
    fn test_auth_response() {
        let auth_response = AuthResponse {
            access_token: "access_token_123".to_string(),
            token_type: "bearer".to_string(),
            expires_in: 3600,
            refresh_token: "refresh_token_456".to_string(),
            scope: "read write".to_string(),
        };

        assert_eq!(auth_response.access_token, "access_token_123");
        assert_eq!(auth_response.token_type, "bearer");
        assert_eq!(auth_response.expires_in, 3600);
        assert_eq!(auth_response.refresh_token, "refresh_token_456");
        assert_eq!(auth_response.scope, "read write");
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination {
            page: Some(1),
            per_page: Some(50),
            total: Some(1000),
            pages: Some(20),
            has_more: Some(true),
        };

        assert_eq!(pagination.page, Some(1));
        assert_eq!(pagination.per_page, Some(50));
        assert_eq!(pagination.total, Some(1000));
        assert_eq!(pagination.pages, Some(20));
        assert_eq!(pagination.has_more, Some(true));
    }

    #[test]
    fn test_paginated_response_new() {
        let data = vec!["item1".to_string(), "item2".to_string()];
        let response = PaginatedResponse::new(data.clone());

        assert_eq!(response.data, data);
        assert_eq!(response.pagination, None);
        assert_eq!(response.len(), 2);
        assert!(!response.is_empty());
        assert!(!response.has_more());
    }

    #[test]
    fn test_paginated_response_with_pagination() {
        let data = vec!["item1".to_string(), "item2".to_string()];
        let pagination = Pagination {
            page: Some(1),
            per_page: Some(2),
            total: Some(10),
            pages: Some(5),
            has_more: Some(true),
        };
        let response = PaginatedResponse::with_pagination(data.clone(), pagination);

        assert_eq!(response.data, data);
        assert!(response.pagination.is_some());
        assert_eq!(response.len(), 2);
        assert!(!response.is_empty());
        assert!(response.has_more());
    }

    #[test]
    fn test_paginated_response_empty() {
        let response: PaginatedResponse<String> = PaginatedResponse::new(vec![]);
        assert_eq!(response.len(), 0);
        assert!(response.is_empty());
        assert!(!response.has_more());
    }

    #[test]
    fn test_notification() {
        let notification = Notification::new("ticker".to_string(), "BTC-PERPETUAL".to_string());
        assert_eq!(notification.jsonrpc, "2.0");
        assert_eq!(notification.method, "ticker");
        assert_eq!(notification.params, "BTC-PERPETUAL");
    }

    #[test]
    fn test_subscription_response() {
        let subscription = SubscriptionResponse {
            subscription: "sub_123".to_string(),
            channel: "ticker.BTC-PERPETUAL".to_string(),
        };
        assert_eq!(subscription.subscription, "sub_123");
        assert_eq!(subscription.channel, "ticker.BTC-PERPETUAL");
    }

    #[test]
    fn test_heartbeat_response() {
        let heartbeat = HeartbeatResponse {
            type_: "heartbeat".to_string(),
        };
        assert_eq!(heartbeat.type_, "heartbeat");
    }

    #[test]
    fn test_test_response() {
        let test_response = TestResponse {
            version: "1.2.3".to_string(),
        };
        assert_eq!(test_response.version, "1.2.3");
    }

    #[test]
    fn test_server_time_response() {
        let server_time = ServerTimeResponse {
            timestamp: 1640995200000,
        };
        assert_eq!(server_time.timestamp, 1640995200000);
    }

    #[test]
    fn test_settlements_response_new() {
        let settlements = vec![];
        let response = SettlementsResponse::new(settlements);
        assert_eq!(response.continuation, None);
        assert!(response.settlements.is_empty());
        assert!(!response.has_more());
    }

    #[test]
    fn test_settlements_response_with_continuation() {
        let settlements = vec![];
        let response = SettlementsResponse::with_continuation(settlements, "token_123".to_string());
        assert_eq!(response.continuation, Some("token_123".to_string()));
        assert!(response.has_more());
    }

    #[test]
    fn test_contract_size_response() {
        let contract_size = ContractSizeResponse { contract_size: 1.0 };
        assert_eq!(contract_size.contract_size, 1.0);
    }

    #[test]
    fn test_status_response() {
        let mut additional_fields = std::collections::HashMap::new();
        additional_fields.insert("custom_field".to_string(), json!("custom_value"));

        let status = StatusResponse {
            locked: Some(false),
            message: Some("System operational".to_string()),
            locked_indices: Some(vec!["BTC".to_string(), "ETH".to_string()]),
            additional_fields,
        };

        assert_eq!(status.locked, Some(false));
        assert_eq!(status.message, Some("System operational".to_string()));
        assert_eq!(
            status.locked_indices,
            Some(vec!["BTC".to_string(), "ETH".to_string()])
        );
        assert_eq!(
            status.additional_fields.get("custom_field"),
            Some(&json!("custom_value"))
        );
    }

    #[test]
    fn test_apr_data_point() {
        let apr_point = AprDataPoint {
            apr: 5.25,
            timestamp: Some(1640995200000),
            day: 365,
        };
        assert_eq!(apr_point.apr, 5.25);
        assert_eq!(apr_point.timestamp, Some(1640995200000));
        assert_eq!(apr_point.day, 365);
    }

    #[test]
    fn test_apr_history_response() {
        let data_points = vec![AprDataPoint {
            apr: 5.25,
            timestamp: Some(1640995200000),
            day: 365,
        }];
        let apr_history = AprHistoryResponse {
            data: data_points,
            continuation: Some("token_456".to_string()),
        };
        assert_eq!(apr_history.data.len(), 1);
        assert_eq!(apr_history.continuation, Some("token_456".to_string()));
    }

    #[test]
    fn test_hello_response() {
        let hello = HelloResponse {
            version: "2.1.1".to_string(),
        };
        assert_eq!(hello.version, "2.1.1");
    }

    #[test]
    fn test_delivery_price_data() {
        let delivery_price = DeliveryPriceData {
            date: "2024-01-01".to_string(),
            delivery_price: 50000.0,
        };
        assert_eq!(delivery_price.date, "2024-01-01");
        assert_eq!(delivery_price.delivery_price, 50000.0);
    }

    #[test]
    fn test_delivery_prices_response() {
        let data = vec![DeliveryPriceData {
            date: "2024-01-01".to_string(),
            delivery_price: 50000.0,
        }];
        let delivery_prices = DeliveryPricesResponse {
            data,
            records_total: 1,
        };
        assert_eq!(delivery_prices.data.len(), 1);
        assert_eq!(delivery_prices.records_total, 1);
    }

    #[test]
    fn test_currency_expirations() {
        let expirations = CurrencyExpirations {
            future: Some(vec!["2024-03-29".to_string()]),
            option: Some(vec!["2024-01-26".to_string(), "2024-02-23".to_string()]),
        };
        assert_eq!(expirations.future, Some(vec!["2024-03-29".to_string()]));
        assert_eq!(
            expirations.option,
            Some(vec!["2024-01-26".to_string(), "2024-02-23".to_string()])
        );
    }

    #[test]
    fn test_expirations_response() {
        let mut currencies = std::collections::HashMap::new();
        currencies.insert(
            "BTC".to_string(),
            CurrencyExpirations {
                future: Some(vec!["2024-03-29".to_string()]),
                option: Some(vec!["2024-01-26".to_string()]),
            },
        );

        let expirations = ExpirationsResponse {
            future: Some(vec!["2024-03-29".to_string()]),
            option: Some(vec!["2024-01-26".to_string()]),
            currencies,
        };

        assert_eq!(expirations.future, Some(vec!["2024-03-29".to_string()]));
        assert_eq!(expirations.option, Some(vec!["2024-01-26".to_string()]));
        assert!(expirations.currencies.contains_key("BTC"));
    }

    #[test]
    fn test_last_trades_response() {
        let last_trades = LastTradesResponse {
            has_more: true,
            trades: vec![],
        };
        assert!(last_trades.has_more);
        assert!(last_trades.trades.is_empty());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let response = JsonRpcResponse::success(Some(json!(1)), "test_result".to_string());
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: JsonRpcResponse<String> = serde_json::from_str(&json).unwrap();

        assert_eq!(response.jsonrpc, deserialized.jsonrpc);
        assert_eq!(response.id, deserialized.id);
        assert_eq!(response.result, deserialized.result);
        assert_eq!(response.error.is_none(), deserialized.error.is_none());
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let auth_response = AuthResponse {
            access_token: "token".to_string(),
            token_type: "bearer".to_string(),
            expires_in: 3600,
            refresh_token: "refresh".to_string(),
            scope: "read".to_string(),
        };

        let debug_str = format!("{:?}", auth_response);
        let display_str = format!("{}", auth_response);

        assert!(debug_str.contains("token"));
        assert!(display_str.contains("token"));
    }
}
