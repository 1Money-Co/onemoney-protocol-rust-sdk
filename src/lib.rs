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
pub mod crypto;
pub mod error;
pub mod types;
pub mod utils;

// Re-export the main types for easy access
pub use api::{
    AccountQuery, BlacklistAction, BlacklistTokenRequest, BurnTokenRequest, Client, ClientBuilder,
    FeeEstimateRequest, MintTokenRequest, Network, PauseAction, PauseTokenRequest, PaymentPayload,
    PaymentRequest, PaymentResponse, TokenAccountQuery, TokenAuthorityPayload,
    TokenAuthorityRequest, TokenBlacklistPayload, TokenBurnPayload, TokenMetadataUpdatePayload,
    TokenMintPayload, TokenOperationResponse, TokenPausePayload, TokenWhitelistPayload,
    TransactionReceipt, UpdateMetadataRequest, WhitelistAction, WhitelistTokenRequest,
};
pub use crypto::{Signable, sign_transaction_payload, *};
pub use error::{ConfigError, CryptoError, Error, Result};
pub use types::*;
pub use utils::EvmWallet;
