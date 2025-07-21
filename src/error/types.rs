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
    Api { code: i32, message: String },
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
