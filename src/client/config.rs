//! Network configuration and API endpoints.

use std::time::Duration;

/// Default mainnet API URL.
pub const MAINNET_URL: &str = "https://api.mainnet.1money.network";

/// Default testnet API URL.
pub const TESTNET_URL: &str = "https://api.testnet.1money.network";

/// Default local API URL.
pub const LOCAL_URL: &str = "http://127.0.0.1:18555";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// API version prefix.
pub const API_VERSION: &str = "/v1";

/// Build an API path with version prefix.
pub fn api_path(path: &str) -> String {
    format!("{}{}", API_VERSION, path)
}

/// Network environment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Network {
    /// Mainnet environment.
    #[default]
    Mainnet,
    /// Testnet environment.
    Testnet,
    /// Local development environment.
    Local,
}

impl Network {
    /// Get the base URL for this network.
    pub fn url(&self) -> &'static str {
        match self {
            Network::Mainnet => MAINNET_URL,
            Network::Testnet => TESTNET_URL,
            Network::Local => LOCAL_URL,
        }
    }

    /// Check if this is a production network.
    pub fn is_production(&self) -> bool {
        matches!(self, Network::Mainnet)
    }

    /// Check if this is a test network.
    pub fn is_test(&self) -> bool {
        matches!(self, Network::Testnet | Network::Local)
    }
}

/// API endpoint paths.
pub mod endpoints {
    /// Account-related endpoints.
    pub mod accounts {
        pub const NONCE: &str = "/accounts/nonce";
        pub const TOKEN_ACCOUNT: &str = "/accounts/token_account";
    }

    /// Chain-related endpoints.
    pub mod chains {
        pub const CHAIN_ID: &str = "/chains/chain_id";
    }

    /// Checkpoint-related endpoints.
    pub mod checkpoints {
        pub const NUMBER: &str = "/checkpoints/number";
        pub const BY_NUMBER: &str = "/checkpoints/by_number";
        pub const BY_HASH: &str = "/checkpoints/by_hash";
    }

    /// Transaction-related endpoints.
    pub mod transactions {
        pub const PAYMENT: &str = "/transactions/payment";
        pub const BY_HASH: &str = "/transactions/by_hash";
        pub const RECEIPT_BY_HASH: &str = "/transactions/receipt/by_hash";
        pub const ESTIMATE_FEE: &str = "/transactions/estimate_fee";
    }

    /// Token-related endpoints.
    pub mod tokens {
        pub const MINT: &str = "/tokens/mint";
        pub const BURN: &str = "/tokens/burn";
        pub const GRANT_AUTHORITY: &str = "/tokens/grant_authority";
        pub const REVOKE_AUTHORITY: &str = "/tokens/revoke_authority";
        pub const UPDATE_METADATA: &str = "/tokens/update_metadata";
        pub const MANAGE_BLACKLIST: &str = "/tokens/manage_blacklist";
        pub const MANAGE_WHITELIST: &str = "/tokens/manage_whitelist";
        pub const PAUSE: &str = "/tokens/pause";
        pub const TOKEN_METADATA: &str = "/tokens/token_metadata";
    }

    /// State-related endpoints.
    pub mod states {
        pub const LATEST_EPOCH_CHECKPOINT: &str = "/states/latest_epoch_checkpoint";
    }
}
