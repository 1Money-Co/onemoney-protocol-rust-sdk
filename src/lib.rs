//! # OneMoney Rust SDK
//!
//! Official Rust SDK for OneMoney L1 blockchain REST API.
//!
//! ## Quick Start
//!
//! ```rust
//! use onemoney_protocol::{Client, ClientBuilder, Network};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create clients for different networks
//!     let mainnet_client = Client::mainnet();          // Mainnet
//!     let testnet_client = Client::testnet();          // Testnet
//!     let local_client = Client::local();              // Local development
//!
//!     // Or use the builder pattern
//!     let client = ClientBuilder::new()
//!         .network(Network::Testnet)
//!         .timeout(std::time::Duration::from_secs(30))
//!         .build()?;
//!
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod crypto;
pub mod error;
pub mod transport;
pub mod types;
pub mod utils;

// Re-export payload types from requests module
pub use client::{Client, ClientBuilder, Network};
pub use crypto::{Signable, sign_transaction_payload, *};
pub use error::{ConfigError, CryptoError, Error, Result};
pub use requests::{
    PaymentPayload, TokenAuthorityPayload, TokenBlacklistPayload, TokenBurnPayload,
    TokenMetadataUpdatePayload, TokenMintPayload, TokenPausePayload, TokenWhitelistPayload,
};
pub use transport::*;
pub use types::*;
pub use utils::*;
