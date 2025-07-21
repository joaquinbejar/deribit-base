//! # Deribit Base
//!
//! This crate provides common structs, traits, and functionality shared across
//! all Deribit API client implementations (FIX, HTTP, WebSocket).

pub mod error;
pub mod model;
pub mod utils;

/// Re-export commonly used types for convenience
pub mod prelude {
    //! Convenience re-exports for common types and traits

    pub use crate::{
        model::order::{NewOrderRequest, OrderSide, OrderType, TimeInForce},
        model::position::Position,
        utils::setup_logger,
    };
}
