//! # Deribit Base
//!
//! This crate provides common structs, traits, and functionality shared across
//! all Deribit API client implementations (FIX, HTTP, WebSocket).

pub mod common;
pub mod error;
pub mod types;

pub use common::*;
pub use error::*;
pub use types::*;
