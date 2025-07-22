//! # Deribit Base
//!
//! This crate provides common structs, traits, and functionality shared across
//! all Deribit API client implementations (FIX, HTTP, WebSocket).

/// Constants and configuration settings
pub mod constants;
/// Error handling types and utilities
pub mod error;
/// Data models for orders, positions, and other trading entities
pub mod model;
/// Re-export commonly used types for convenience
pub mod prelude;
/// Utility functions and helpers
pub mod utils;
