/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

use crate::error::codes::DeribitErrorCode;
use crate::error::types::DeribitError;

// Conversion from DeribitErrorCode to DeribitError
impl From<DeribitErrorCode> for DeribitError {
    fn from(error_code: DeribitErrorCode) -> Self {
        DeribitError::Api {
            code: error_code.code(),
            message: error_code.message().to_string(),
        }
    }
}

// Conversion from HTTP status codes to DeribitErrorCode
impl From<u16> for DeribitErrorCode {
    fn from(status: u16) -> Self {
        match status {
            400 => DeribitErrorCode::BadRequest,
            401 => DeribitErrorCode::Unauthorized,
            403 => DeribitErrorCode::Forbidden,
            404 => DeribitErrorCode::NotFound,
            429 => DeribitErrorCode::TooManyRequests,
            500 => DeribitErrorCode::InternalServerError,
            503 => DeribitErrorCode::TemporarilyUnavailable,
            _ => DeribitErrorCode::Unknown(status as i32),
        }
    }
}

// Conversion from string error messages to DeribitError
impl From<String> for DeribitError {
    fn from(message: String) -> Self {
        DeribitError::Connection(message)
    }
}

impl From<&str> for DeribitError {
    fn from(message: &str) -> Self {
        DeribitError::Connection(message.to_string())
    }
}

// Conversion from serde_json::Error to DeribitError
impl From<serde_json::Error> for DeribitError {
    fn from(error: serde_json::Error) -> Self {
        DeribitError::Serialization(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_to_deribit_error_conversion() {
        let error_code = DeribitErrorCode::AuthorizationRequired;
        let deribit_error: DeribitError = error_code.into();

        match deribit_error {
            DeribitError::Api { code, message } => {
                assert_eq!(code, 10000);
                assert_eq!(message, "authorization_required");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[test]
    fn test_http_status_to_error_code_conversion() {
        // Test HTTP status to DeribitErrorCode conversion
        let error_400 = DeribitErrorCode::from(400u16);
        assert_eq!(error_400, DeribitErrorCode::BadRequest);
        assert_eq!(error_400.code(), 11050); // BadRequest maps to 11050

        let error_401 = DeribitErrorCode::from(401u16);
        assert_eq!(error_401, DeribitErrorCode::Unauthorized);
        assert_eq!(error_401.code(), 13009); // Unauthorized maps to 13009

        let error_403 = DeribitErrorCode::from(403u16);
        assert_eq!(error_403, DeribitErrorCode::Forbidden);
        assert_eq!(error_403.code(), 13021); // Forbidden maps to 13021

        let error_404 = DeribitErrorCode::from(404u16);
        assert_eq!(error_404, DeribitErrorCode::NotFound);
        assert_eq!(error_404.code(), 13020); // NotFound maps to 13020

        let error_429 = DeribitErrorCode::from(429u16);
        assert_eq!(error_429, DeribitErrorCode::TooManyRequests);
        assert_eq!(error_429.code(), 10028); // TooManyRequests maps to 10028

        let error_500 = DeribitErrorCode::from(500u16);
        assert_eq!(error_500, DeribitErrorCode::InternalServerError);
        assert_eq!(error_500.code(), 11094); // InternalServerError maps to 11094

        let error_503 = DeribitErrorCode::from(503u16);
        assert_eq!(error_503, DeribitErrorCode::TemporarilyUnavailable);
        assert_eq!(error_503.code(), 13028); // TemporarilyUnavailable maps to 13028

        // Test unknown status code
        let error_418 = DeribitErrorCode::from(418u16);
        assert_eq!(error_418, DeribitErrorCode::Unknown(418));
        assert_eq!(error_418.code(), 418);

        // Verify all have proper messages
        for error in [
            error_400, error_401, error_403, error_404, error_429, error_500, error_503, error_418,
        ] {
            assert!(!error.message().is_empty());
        }
    }

    #[test]
    fn test_string_to_deribit_error_conversion() {
        let message = "Connection failed".to_string();
        let error: DeribitError = message.clone().into();

        match error {
            DeribitError::Connection(msg) => {
                assert_eq!(msg, message);
            }
            _ => panic!("Expected Connection error"),
        }
    }

    #[test]
    fn test_str_to_deribit_error_conversion() {
        let message = "Connection timeout";
        let error: DeribitError = message.into();

        match error {
            DeribitError::Connection(msg) => {
                assert_eq!(msg, message);
            }
            _ => panic!("Expected Connection error"),
        }
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let invalid_json = r#"{"invalid": json}"#;
        let json_error = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let deribit_error: DeribitError = json_error.into();

        match deribit_error {
            DeribitError::Serialization(message) => {
                assert!(!message.is_empty());
                assert!(message.contains("expected"));
            }
            _ => panic!("Expected Serialization error"),
        }
    }

    #[test]
    fn test_comprehensive_error_code_conversions() {
        // Test various error codes to ensure they convert properly
        let error_codes = vec![
            DeribitErrorCode::Success,
            DeribitErrorCode::Error,
            DeribitErrorCode::QtyTooLow,
            DeribitErrorCode::NotEnoughFunds,
            DeribitErrorCode::InvalidAmount,
            DeribitErrorCode::TooManyRequests,
            DeribitErrorCode::Unknown(99999),
        ];

        for error_code in error_codes {
            let deribit_error: DeribitError = error_code.clone().into();
            match deribit_error {
                DeribitError::Api { code, message } => {
                    assert_eq!(code, error_code.code());
                    assert_eq!(message, error_code.message());
                }
                _ => panic!("Expected Api error for {:?}", error_code),
            }
        }
    }

    #[test]
    fn test_error_chain_conversions() {
        // Test that we can chain conversions
        let status_code: u16 = 401;
        let error_code: DeribitErrorCode = status_code.into();
        let deribit_error: DeribitError = error_code.into();

        match deribit_error {
            DeribitError::Api { code, message } => {
                assert_eq!(code, 13009); // Unauthorized code
                assert_eq!(message, "unauthorized");
            }
            _ => panic!("Expected Api error"),
        }
    }
}
