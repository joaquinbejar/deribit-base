#![warn(missing_docs)]

//! # Deribit Base
//!
//! This crate provides common structs, traits, and functionality shared across
//! all Deribit API client implementations (FIX, HTTP, WebSocket).

/// Error handling types and utilities
pub mod error;
/// Data models for orders, positions, and other trading entities
pub mod model;
/// Utility functions and helpers
pub mod utils;

pub mod constants;
/// Re-export commonly used types for convenience
pub mod prelude;
