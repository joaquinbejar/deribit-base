<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/deribit-base/refs/heads/main/doc/images/logo.png" alt="deribit-base" style="width: 80%; height: 80%;">
</div>

[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/deribit-base.svg)](https://crates.io/crates/deribit-base)
[![Downloads](https://img.shields.io/crates/d/deribit-base.svg)](https://crates.io/crates/deribit-base)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/deribit-base.svg)](https://github.com/joaquinbejar/deribit-base/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/deribit-base.svg)](https://github.com/joaquinbejar/deribit-base/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/deribit-base.svg)](https://github.com/joaquinbejar/deribit-base/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/deribit-base/CI)](https://github.com/joaquinbejar/deribit-base/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/deribit-base)](https://codecov.io/gh/joaquinbejar/deribit-base)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/deribit-base)](https://libraries.io/github/joaquinbejar/deribit-base)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deribit-base)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/deribit-base)

## Deribit Base

A comprehensive Rust library providing common data structures, utilities, and constants
for building Deribit cryptocurrency derivatives trading applications.

This crate serves as the foundation for all Deribit API client implementations,
supporting FIX 4.4, HTTP REST, and WebSocket protocols with a unified, type-safe interface.

### Features

- **🔧 Complete API Constants**: All Deribit API v2.1.1 configuration values
- **📊 Trading Data Models**: Orders, positions, market data, and account structures
- **🛡️ Comprehensive Error Handling**: 100+ official Deribit error codes with categorization
- **🔐 Cryptographic Utilities**: Secure authentication, checksums, and FIX protocol support
- **⚡ Protocol Agnostic**: Compatible with FIX, HTTP REST, and WebSocket implementations
- **🧪 Fully Tested**: 74 comprehensive unit tests ensuring reliability
- **📚 Complete Documentation**: Extensive API documentation with examples

### Supported Protocols

| Protocol | Version | Status |
|----------|---------|--------|
| **FIX** | 4.4 | ✅ Alpha Support |
| **HTTP REST** | v2.1.1 | ✅ Alpha Support |
| **WebSocket** | v2.1.1 | ✅ Alpha Support |

### Supported Assets

- **Bitcoin (BTC)** - Perpetuals, Futures, Options
- **Ethereum (ETH)** - Perpetuals, Futures, Options
- **Solana (SOL)** - Perpetuals, Futures, Options
- **Stablecoins** - USDC, USDT, EURR

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
deribit-base = "0.2"
```

#### Basic Usage

The crate provides a comprehensive prelude module that exports all commonly used types:
- API constants for endpoints, rate limits, and configuration
- Error handling with categorized Deribit error codes
- Trading data structures for orders, positions, and market data
- Utility functions for FIX protocol, authentication, and data formatting

#### FIX Protocol Utilities

The utils module provides essential functions for FIX protocol integration:
- Secure nonce generation for authentication
- FIX timestamp formatting and parsing
- Message checksum calculation and validation
- Order data conversion to FIX field formats

### Module Overview

#### [`constants`]
Complete set of Deribit API configuration values including:
- API endpoints (production & test)
- Rate limits and timeouts
- Supported currencies and instruments
- Order limits and precision settings
- FIX protocol constants

#### [`error`]
Comprehensive error handling with:
- 100+ official Deribit error codes
- Error categorization and helper methods
- Serde support for JSON serialization
- Standard Error trait implementations

#### [`model`]
Protocol-agnostic data structures for:
- **Trading**: Orders, positions, trades, settlements
- **Market Data**: Tickers, order books, instruments, candles
- **Account**: Portfolios, balances, transfers, subaccounts
- **Configuration**: Request/response wrappers, authentication

#### [`utils`]
Utility functions for:
- **Cryptographic Operations**: Nonce generation, checksums, hashing
- **FIX Protocol**: Message formatting, field conversions, validation
- **Data Processing**: Price/quantity formatting, decimal parsing
- **Logging**: Structured logging setup and configuration

### Technical Specifications

#### Rate Limits
- **Authenticated**: 20 requests/second
- **Unauthenticated**: 10 requests/second
- **WebSocket Subscriptions**: 200 per connection

#### Order Limits
| Currency | Min Order | Max Order | Price Precision | Amount Precision |
|----------|-----------|-----------|-----------------|------------------|
| BTC | 0.0001 | 1,000,000 | 8 decimals | 4 decimals |
| ETH | 0.001 | 10,000,000 | 4 decimals | 3 decimals |
| SOL | 0.1 | 100,000,000 | 4 decimals | 1 decimal |

#### Authentication
- **Access Token**: 8 hours (28,800 seconds)
- **Refresh Token**: 30 days (2,592,000 seconds)
- **Refresh Buffer**: 5 minutes before expiration

### Error Handling

The crate provides comprehensive error handling with categorized error codes:
- 100+ official Deribit error codes with descriptive names
- Error categorization methods for different error types
- Automatic conversion between numeric codes and enum variants
- Support for authorization, rate limiting, trading, and validation errors

### Testing

The crate includes comprehensive test coverage with 74 unit tests covering:
- All API constants and their relationships
- Error code conversions and categorization
- Data model serialization and validation
- Utility function correctness and edge cases

### Compatibility

- **Rust Version**: 1.70.0 or higher
- **Deribit API**: v2.1.1
- **FIX Protocol**: 4.4
- **Platforms**: Linux, macOS, Windows

### License

This project is licensed under the MIT License - see the LICENSE file for details.

### Links

- [Deribit API Documentation](https://docs.deribit.com/)
- [FIX 4.4 Specification](https://www.fixtrading.org/)
- [Repository](https://github.com/joaquinbejar/deribit-rs)

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

### **Contact Information**
- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/deribit-base>
- **Documentation**: <https://docs.rs/deribit-base>

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with Deribit. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
