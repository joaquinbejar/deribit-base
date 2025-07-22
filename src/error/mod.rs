/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/

//! Error handling types and utilities for Deribit API
//!
//! This module provides comprehensive error handling for all Deribit API operations,
//! including:
//! - RPC error codes from the official Deribit API documentation
//! - Error type definitions and result types
//! - Error code conversions and categorization

/// RPC error codes from Deribit API v2.1.1 documentation
pub mod codes;
/// Error type definitions and result types
pub mod types;

/// Internal error code conversions
mod conversions;
