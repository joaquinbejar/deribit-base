/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Utility functions and tools for Deribit API operations
//!
//! This module provides various utility functions for:
//! - Cryptographic operations (nonce generation, checksums)
//! - Time and timestamp formatting
//! - FIX protocol message formatting and validation
//! - Order and instrument data conversion
//! - String escaping and parsing utilities

use crate::model::order::{OrderSide, OrderType, TimeInForce};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use rand::{Rng, rng};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a random nonce string of the specified length.
/// Used for cryptographic operations and unique identifiers.
///
/// # Arguments
/// * `length` - The desired length of the nonce string
///
/// # Returns
/// A random alphanumeric string of the specified length
#[allow(dead_code)]
pub fn generate_nonce(length: usize) -> String {
    let mut rng = rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    base64::prelude::BASE64_STANDARD.encode(bytes)
}

/// Generate a Unix timestamp in milliseconds
#[allow(dead_code)]
pub fn generate_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Format a DateTime as FIX time string (YYYYMMDD-HH:MM:SS.sss)
#[allow(dead_code)]
pub fn format_fix_time(time: DateTime<Utc>) -> String {
    time.format("%Y%m%d-%H:%M:%S%.3f").to_string()
}

/// Parse a FIX time string into a DateTime
#[allow(dead_code)]
pub fn parse_fix_time(time_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_str(&format!("{time_str}+00:00"), "%Y%m%d-%H:%M:%S%.3f%z")
        .map(|dt| dt.with_timezone(&Utc))
}

/// Calculate FIX message checksum
#[allow(dead_code)]
pub fn calculate_checksum(message: &str) -> u8 {
    let sum: u32 = message.bytes().map(|b| b as u32).sum();
    (sum % 256) as u8
}

/// Validate FIX checksum
#[allow(dead_code)]
pub fn validate_checksum(message: &str) -> bool {
    if let Some(checksum_pos) = message.rfind("10=") {
        let message_without_checksum = &message[..checksum_pos];
        let expected_checksum = calculate_checksum(message_without_checksum);

        if let Some(checksum_str) = message[checksum_pos + 3..].split('\x01').next()
            && let Ok(actual_checksum) = checksum_str.parse::<u8>()
        {
            return expected_checksum == actual_checksum;
        }
    }
    false
}

/// Generate a unique client order ID
#[allow(dead_code)]
pub fn generate_client_order_id(prefix: &str) -> String {
    format!("{}_{}", prefix, generate_timestamp())
}

/// Convert price to FIX decimal format
#[allow(dead_code)]
pub fn format_price(price: f64, precision: usize) -> String {
    format!("{price:.precision$}")
}

/// Convert quantity to FIX decimal format
#[allow(dead_code)]
pub fn format_quantity(quantity: f64, precision: usize) -> String {
    format!("{quantity:.precision$}")
}

/// Parse FIX decimal string to f64
#[allow(dead_code)]
pub fn parse_decimal(decimal_str: &str) -> Result<f64, std::num::ParseFloatError> {
    decimal_str.parse::<f64>()
}

/// Escape FIX field value (replace SOH with readable representation)
#[allow(dead_code)]
pub fn escape_fix_value(value: &str) -> String {
    value.replace('\x01', "\\001") // SOH
}

/// Unescape FIX field value (restore SOH characters)
#[allow(dead_code)]
pub fn unescape_fix_value(value: &str) -> String {
    value.replace("\\001", "\x01") // SOH
}

/// Generate a unique request ID for JSON-RPC calls
#[allow(dead_code)]
pub fn generate_request_id(prefix: &str) -> String {
    let mut rng = rng();
    let random_part: u32 = rng.random();
    format!("{prefix}_{random_part}")
}

/// Convert OrderSide to FIX Side field
#[allow(dead_code)]
pub fn side_to_fix(side: OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "1",
        OrderSide::Sell => "2",
    }
}

/// Convert OrderType to FIX OrdType field
#[allow(dead_code)]
pub fn order_type_to_fix(order_type: OrderType) -> &'static str {
    match order_type {
        OrderType::Market => "1",
        OrderType::Limit => "2",
        OrderType::Stop => "3",
        OrderType::StopLimit => "4",
    }
}

/// Convert TimeInForce to FIX TimeInForce field
#[allow(dead_code)]
pub fn time_in_force_to_fix(tif: TimeInForce) -> &'static str {
    match tif {
        TimeInForce::Day => "0",
        TimeInForce::GoodTillCancel => "1",
        TimeInForce::ImmediateOrCancel => "3",
        TimeInForce::FillOrKill => "4",
    }
}

/// Validate Deribit instrument name format
#[allow(dead_code)]
pub fn validate_instrument_name(instrument: &str) -> bool {
    // Basic validation for Deribit instrument naming convention
    // Examples: BTC-PERPETUAL, ETH-25DEC20-600-C, BTC-25DEC20
    if instrument.is_empty() {
        return false;
    }

    // Must contain at least one dash
    if !instrument.contains('-') {
        return false;
    }

    // Must start with a valid currency
    let valid_currencies = ["BTC", "ETH", "USD", "USDC"];

    valid_currencies
        .iter()
        .any(|&currency| instrument.starts_with(currency))
}

/// Extract currency from Deribit instrument name
#[allow(dead_code)]
pub fn extract_currency_from_instrument(instrument: &str) -> Option<&str> {
    if let Some(dash_pos) = instrument.find('-') {
        Some(&instrument[..dash_pos])
    } else {
        None
    }
}

/// Format instrument name for Deribit API calls
#[allow(dead_code)]
pub fn format_deribit_instrument(
    currency: &str,
    expiry: Option<&str>,
    strike: Option<f64>,
    option_type: Option<&str>,
) -> String {
    let mut instrument = currency.to_string();

    if let Some(exp) = expiry {
        instrument.push('-');
        instrument.push_str(exp);

        if let Some(strike_price) = strike {
            instrument.push('-');
            instrument.push_str(&strike_price.to_string());

            if let Some(opt_type) = option_type {
                instrument.push('-');
                instrument.push_str(opt_type);
            }
        }
    } else {
        // Perpetual contract
        instrument.push_str("-PERPETUAL");
    }

    instrument
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce(32);
        let nonce2 = generate_nonce(32);

        assert_ne!(nonce1, nonce2);
        assert!(!nonce1.is_empty());
        assert!(!nonce2.is_empty());
    }

    #[test]
    fn test_checksum_calculation() {
        let message = "8=FIX.4.4\x019=61\x0135=A\x0149=CLIENT\x0156=DERIBITSERVER\x0134=1\x01";
        let checksum = calculate_checksum(message);
        assert_eq!(checksum, 169);
    }

    #[test]
    fn test_instrument_validation() {
        assert!(validate_instrument_name("BTC-PERPETUAL"));
        assert!(validate_instrument_name("ETH-25DEC20-600-C"));
        assert!(validate_instrument_name("BTC-25DEC20"));
        assert!(!validate_instrument_name("INVALID"));
        assert!(!validate_instrument_name(""));
    }

    #[test]
    fn test_currency_extraction() {
        assert_eq!(
            extract_currency_from_instrument("BTC-PERPETUAL"),
            Some("BTC")
        );
        assert_eq!(
            extract_currency_from_instrument("ETH-25DEC20-600-C"),
            Some("ETH")
        );
        assert_eq!(extract_currency_from_instrument("INVALID"), None);
    }

    #[test]
    fn test_instrument_formatting() {
        assert_eq!(
            format_deribit_instrument("BTC", None, None, None),
            "BTC-PERPETUAL"
        );
        assert_eq!(
            format_deribit_instrument("ETH", Some("25DEC20"), Some(600.0), Some("C")),
            "ETH-25DEC20-600-C"
        );
        assert_eq!(
            format_deribit_instrument("BTC", Some("25DEC20"), None, None),
            "BTC-25DEC20"
        );
    }

    #[test]
    fn test_generate_timestamp() {
        let timestamp1 = generate_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let timestamp2 = generate_timestamp();
        assert!(timestamp2 > timestamp1);
        assert!(timestamp1 > 0);
    }

    #[test]
    fn test_format_fix_time() {
        use chrono::{TimeZone, Utc};
        let dt = Utc
            .with_ymd_and_hms(2023, 12, 25, 14, 30, 45)
            .unwrap()
            .with_nanosecond(123_000_000)
            .unwrap();
        let formatted = format_fix_time(dt);
        assert_eq!(formatted, "20231225-14:30:45.123");
    }

    #[test]
    fn test_parse_fix_time() {
        let time_str = "20231225-14:30:45.123";
        let parsed = parse_fix_time(time_str).unwrap();
        assert_eq!(parsed.year(), 2023);
        assert_eq!(parsed.month(), 12);
        assert_eq!(parsed.day(), 25);
        assert_eq!(parsed.hour(), 14);
        assert_eq!(parsed.minute(), 30);
        assert_eq!(parsed.second(), 45);
    }

    #[test]
    fn test_validate_checksum() {
        let message =
            "8=FIX.4.4\x019=61\x0135=A\x0149=CLIENT\x0156=DERIBITSERVER\x0134=1\x0110=169\x01";
        assert!(validate_checksum(message));

        let invalid_message =
            "8=FIX.4.4\x019=61\x0135=A\x0149=CLIENT\x0156=DERIBITSERVER\x0134=1\x0110=170\x01";
        assert!(!validate_checksum(invalid_message));
    }

    #[test]
    fn test_generate_client_order_id() {
        let order_id1 = generate_client_order_id("TEST");
        std::thread::sleep(std::time::Duration::from_millis(1));
        let order_id2 = generate_client_order_id("TEST");

        assert!(order_id1.starts_with("TEST_"));
        assert!(order_id2.starts_with("TEST_"));
        // Since we use timestamps, they should be different with a small delay
        assert_ne!(order_id1, order_id2);
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(123.456789, 2), "123.46");
        assert_eq!(format_price(123.456789, 4), "123.4568");
        assert_eq!(format_price(123.0, 2), "123.00");
    }

    #[test]
    fn test_format_quantity() {
        assert_eq!(format_quantity(0.12345, 3), "0.123");
        assert_eq!(format_quantity(1.0, 1), "1.0");
        assert_eq!(format_quantity(10.5678, 2), "10.57");
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_decimal("123.456").unwrap(), 123.456);
        assert_eq!(parse_decimal("0.001").unwrap(), 0.001);
        assert!(parse_decimal("invalid").is_err());
    }

    #[test]
    fn test_escape_fix_value() {
        assert_eq!(escape_fix_value("test\x01value"), "test\\001value");
        assert_eq!(escape_fix_value("normal_text"), "normal_text");
    }

    #[test]
    fn test_unescape_fix_value() {
        assert_eq!(unescape_fix_value("test\\001value"), "test\x01value");
        assert_eq!(unescape_fix_value("normal_text"), "normal_text");
    }

    #[test]
    fn test_generate_request_id() {
        let req_id1 = generate_request_id("REQ");
        let req_id2 = generate_request_id("REQ");

        assert!(req_id1.starts_with("REQ_"));
        assert!(req_id2.starts_with("REQ_"));
        assert_ne!(req_id1, req_id2);
    }

    #[test]
    fn test_side_to_fix() {
        assert_eq!(side_to_fix(OrderSide::Buy), "1");
        assert_eq!(side_to_fix(OrderSide::Sell), "2");
    }

    #[test]
    fn test_order_type_to_fix() {
        assert_eq!(order_type_to_fix(OrderType::Market), "1");
        assert_eq!(order_type_to_fix(OrderType::Limit), "2");
        assert_eq!(order_type_to_fix(OrderType::Stop), "3");
        assert_eq!(order_type_to_fix(OrderType::StopLimit), "4");
    }

    #[test]
    fn test_time_in_force_to_fix() {
        assert_eq!(time_in_force_to_fix(TimeInForce::Day), "0");
        assert_eq!(time_in_force_to_fix(TimeInForce::GoodTillCancel), "1");
        assert_eq!(time_in_force_to_fix(TimeInForce::ImmediateOrCancel), "3");
        assert_eq!(time_in_force_to_fix(TimeInForce::FillOrKill), "4");
    }
}
