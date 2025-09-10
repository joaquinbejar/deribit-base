//! Common error types for Deribit clients

use std::fmt;

/// Common error type for all Deribit clients
#[derive(Debug, Clone)]
pub enum DeribitError {
    /// Connection error
    Connection(String),
    /// Authentication error
    Authentication(String),
    /// API error with code and message
    Api {
        /// Error code returned by the API
        code: i32,
        /// Human-readable error message
        message: String,
    },
    /// Serialization/deserialization error
    Serialization(String),
    /// Network timeout
    Timeout,
    /// Invalid configuration
    InvalidConfig(String),
    /// Generic error
    Other(String),
}

impl fmt::Display for DeribitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeribitError::Connection(msg) => write!(f, "Connection error: {msg}"),
            DeribitError::Authentication(msg) => write!(f, "Authentication error: {msg}"),
            DeribitError::Api { code, message } => write!(f, "API error {code}: {message}"),
            DeribitError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            DeribitError::Timeout => write!(f, "Request timeout"),
            DeribitError::InvalidConfig(msg) => write!(f, "Invalid configuration: {msg}"),
            DeribitError::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for DeribitError {}

/// Result type alias for Deribit operations
pub type DeribitResult<T> = Result<T, DeribitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deribit_error_connection() {
        let error = DeribitError::Connection("Failed to connect".to_string());
        assert_eq!(error.to_string(), "Connection error: Failed to connect");
    }

    #[test]
    fn test_deribit_error_authentication() {
        let error = DeribitError::Authentication("Invalid credentials".to_string());
        assert_eq!(
            error.to_string(),
            "Authentication error: Invalid credentials"
        );
    }

    #[test]
    fn test_deribit_error_api() {
        let error = DeribitError::Api {
            code: 10009,
            message: "Invalid request".to_string(),
        };
        assert_eq!(error.to_string(), "API error 10009: Invalid request");
    }

    #[test]
    fn test_deribit_error_serialization() {
        let error = DeribitError::Serialization("JSON parse error".to_string());
        assert_eq!(error.to_string(), "Serialization error: JSON parse error");
    }

    #[test]
    fn test_deribit_error_timeout() {
        let error = DeribitError::Timeout;
        assert_eq!(error.to_string(), "Request timeout");
    }

    #[test]
    fn test_deribit_error_invalid_config() {
        let error = DeribitError::InvalidConfig("Missing API key".to_string());
        assert_eq!(error.to_string(), "Invalid configuration: Missing API key");
    }

    #[test]
    fn test_deribit_error_other() {
        let error = DeribitError::Other("Unknown error".to_string());
        assert_eq!(error.to_string(), "Error: Unknown error");
    }

    #[test]
    fn test_deribit_error_debug() {
        let error = DeribitError::Connection("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Connection"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_deribit_error_clone() {
        let error = DeribitError::Api {
            code: 123,
            message: "test".to_string(),
        };
        let cloned = error.clone();
        assert_eq!(error.to_string(), cloned.to_string());
    }

    #[test]
    fn test_deribit_result_type() {
        let success: DeribitResult<i32> = Ok(42);
        let failure: DeribitResult<i32> = Err(DeribitError::Timeout);

        assert!(success.is_ok());
        assert!(failure.is_err());
        if let Ok(value) = success {
            assert_eq!(value, 42);
        }
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = DeribitError::Connection("test".to_string());
        let _: &dyn std::error::Error = &error;
    }
}
