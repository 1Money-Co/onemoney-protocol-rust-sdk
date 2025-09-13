//! Network configuration and API endpoints.

use std::{borrow::Cow, time::Duration};

/// Default mainnet API URL.
pub const MAINNET_URL: &str = "https://api.mainnet.1money.network";

/// Default testnet API URL.
pub const TESTNET_URL: &str = "https://api.testnet.1money.network";

/// Default local API URL.
pub const LOCAL_URL: &str = "http://127.0.0.1:18555";

// TODO:
//  Migrate to use [`ChainSpec`] from l1client repo as single source of truth instead of
//  duplicate chain definitions in SDK which would easily make inconsistency mistakes.
//  At present, the 1m/1m-e2e tool use `TESTNET` as default, requires a changes also.
/// Mainnet chain ID.
pub const MAINNET_CHAIN_ID: u64 = 21210;
/// Testnet chain ID.
pub const TESTNET_CHAIN_ID: u64 = 1_212_101;
/// Local chain ID (same as testnet).
// TODO: Local can be any chain id, should not be hardcoded.
pub const LOCAL_CHAIN_ID: u64 = TESTNET_CHAIN_ID;

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// API version prefix.
pub const API_VERSION: &str = "/v1";

/// Build an API path with version prefix.
pub fn api_path(path: &str) -> String {
    format!("{}{}", API_VERSION, path)
}

/// Network environment options.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Network {
    /// Mainnet environment.
    #[default]
    Mainnet,
    /// Testnet environment.
    Testnet,
    /// Local development environment.
    Local,
    /// Custom network environment.
    Custom(Cow<'static, str>),
}

impl Network {
    /// Get the base URL for this network.
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => MAINNET_URL,
            Network::Testnet => TESTNET_URL,
            Network::Local => LOCAL_URL,
            Network::Custom(s) => s,
        }
    }

    pub const fn predefined_chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => MAINNET_CHAIN_ID,
            Network::Testnet => TESTNET_CHAIN_ID,
            Network::Local => LOCAL_CHAIN_ID,
            Network::Custom(_) => panic!(
                "Custom network does not have a predefined chain ID. Must fetch from network instead."
            ),
        }
    }

    /// Check if this is a production network.
    pub fn is_production(&self) -> bool {
        matches!(self, Network::Mainnet)
    }

    /// Check if this is a test network.
    pub fn is_test(&self) -> bool {
        !self.is_production()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_urls() {
        assert_eq!(Network::Mainnet.url(), "https://api.mainnet.1money.network");
        assert_eq!(Network::Testnet.url(), "https://api.testnet.1money.network");
        assert_eq!(Network::Local.url(), "http://127.0.0.1:18555");
    }

    #[test]
    fn test_network_properties() {
        assert!(Network::Mainnet.is_production());
        assert!(!Network::Testnet.is_production());
        assert!(!Network::Local.is_production());

        assert!(!Network::Mainnet.is_test());
        assert!(Network::Testnet.is_test());
        assert!(Network::Local.is_test());
    }

    #[test]
    fn test_api_path_construction() {
        // Test basic API path construction
        let path = api_path("/test");
        assert_eq!(path, "/v1/test");
        assert!(path.contains("/v1/test"));

        // Test with leading slash
        let path_with_slash = api_path("/accounts/nonce");
        assert_eq!(path_with_slash, "/v1/accounts/nonce");

        // Test without leading slash
        let path_without_slash = api_path("chains/chain_id");
        assert_eq!(path_without_slash, "/v1chains/chain_id");
    }

    #[test]
    fn test_endpoint_constants() {
        // Test account endpoints
        assert_eq!(endpoints::accounts::NONCE, "/accounts/nonce");
        assert_eq!(
            endpoints::accounts::TOKEN_ACCOUNT,
            "/accounts/token_account"
        );

        // Test chain endpoints
        assert_eq!(endpoints::chains::CHAIN_ID, "/chains/chain_id");

        // Test state endpoints
        assert_eq!(
            endpoints::states::LATEST_EPOCH_CHECKPOINT,
            "/states/latest_epoch_checkpoint"
        );

        // Test checkpoint endpoints
        assert_eq!(endpoints::checkpoints::NUMBER, "/checkpoints/number");
        assert_eq!(endpoints::checkpoints::BY_NUMBER, "/checkpoints/by_number");
        assert_eq!(endpoints::checkpoints::BY_HASH, "/checkpoints/by_hash");

        // Test transaction endpoints
        assert_eq!(endpoints::transactions::PAYMENT, "/transactions/payment");
        assert_eq!(endpoints::transactions::BY_HASH, "/transactions/by_hash");
        assert_eq!(
            endpoints::transactions::RECEIPT_BY_HASH,
            "/transactions/receipt/by_hash"
        );
        assert_eq!(
            endpoints::transactions::ESTIMATE_FEE,
            "/transactions/estimate_fee"
        );

        // Test token endpoints
        assert_eq!(endpoints::tokens::MINT, "/tokens/mint");
        assert_eq!(endpoints::tokens::BURN, "/tokens/burn");
        assert_eq!(
            endpoints::tokens::GRANT_AUTHORITY,
            "/tokens/grant_authority"
        );
        assert_eq!(
            endpoints::tokens::UPDATE_METADATA,
            "/tokens/update_metadata"
        );
        assert_eq!(
            endpoints::tokens::MANAGE_BLACKLIST,
            "/tokens/manage_blacklist"
        );
        assert_eq!(
            endpoints::tokens::MANAGE_WHITELIST,
            "/tokens/manage_whitelist"
        );
        assert_eq!(endpoints::tokens::PAUSE, "/tokens/pause");
        assert_eq!(endpoints::tokens::TOKEN_METADATA, "/tokens/token_metadata");
    }

    #[test]
    fn test_network_default() {
        let default_network = Network::default();
        assert_eq!(default_network, Network::Mainnet);
    }

    #[test]
    fn test_network_chain_ids() {
        assert_eq!(Network::Mainnet.predefined_chain_id(), 21210);
        assert_eq!(Network::Testnet.predefined_chain_id(), 1_212_101);
        assert_eq!(Network::Local.predefined_chain_id(), 1_212_101);
    }

    #[test]
    #[should_panic(expected = "Custom network does not have a predefined chain ID")]
    fn test_predefined_chain_id_panics_for_custom() {
        let n = Network::Custom("http://localhost:18555".into());
        let _ = n.predefined_chain_id();
    }

    #[test]
    fn test_constants() {
        assert_eq!(API_VERSION, "/v1");
        assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));

        // Verify URL constants
        assert!(MAINNET_URL.starts_with("https://"));
        assert!(TESTNET_URL.starts_with("https://"));
        assert!(LOCAL_URL.starts_with("http://"));
    }
}
