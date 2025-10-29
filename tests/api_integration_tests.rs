//! Comprehensive API integration tests
//!
//! This file contains all API-related integration tests including:
//! - API endpoint constant verification
//! - API path construction tests
//! - Cross-module API call validation
//! - Data flow verification across different API endpoints

use onemoney_protocol::client::config::api_path;
use onemoney_protocol::client::config::endpoints::{
    accounts::*,
    chains::*,
    checkpoints::{BY_HASH as CHECKPOINT_BY_HASH, BY_NUMBER},
    tokens::*,
    transactions::{BY_HASH as TRANSACTION_BY_HASH, ESTIMATE_FEE, PAYMENT, RECEIPT_BY_HASH},
};

//
// ============================================================================
// ACCOUNTS API TESTS
// ============================================================================
//

#[test]
fn test_accounts_endpoint_constants() {
    // Test all account endpoint constants are correct (without version prefix)
    assert_eq!(NONCE, "/accounts/nonce");
    assert_eq!(TOKEN_ACCOUNT, "/accounts/token_account");
}

#[test]
fn test_accounts_api_path_construction() {
    // Test path construction for all account endpoints (with version prefix)
    let nonce_path = api_path(&format!("{}?address={}", NONCE, "0x1234"));
    assert!(nonce_path.contains("/v1/accounts/nonce"));
    assert!(nonce_path.contains("address=0x1234"));

    let token_path = api_path(&format!(
        "{}?owner={}&token={}",
        TOKEN_ACCOUNT, "0x1234", "0x5678"
    ));
    assert!(token_path.contains("/v1/accounts/token_account"));
    assert!(token_path.contains("owner=0x1234"));
    assert!(token_path.contains("token=0x5678"));
}

//
// ============================================================================
// CHAINS API TESTS
// ============================================================================
//

#[test]
fn test_chains_endpoint_constants() {
    // Test chain endpoint constants
    assert_eq!(CHAIN_ID, "/chains/chain_id");
}

#[test]
fn test_chains_api_path_construction() {
    // Test path construction for chain endpoints
    let chain_id_path = api_path(CHAIN_ID);
    assert_eq!(chain_id_path, "/v1/chains/chain_id");
}

//
// ============================================================================
// CHECKPOINTS API TESTS
// ============================================================================
//

#[test]
fn test_checkpoints_endpoint_constants() {
    // Test all checkpoint endpoint constants are correct (without version prefix)
    assert_eq!(BY_NUMBER, "/checkpoints/by_number");
    assert_eq!(CHECKPOINT_BY_HASH, "/checkpoints/by_hash");
}

#[test]
fn test_checkpoints_api_path_construction() {
    // Test get_checkpoint_by_number paths
    let path_without_full = api_path(&format!("{}?number={}&full={}", BY_NUMBER, 1500, false));
    assert!(path_without_full.contains("/v1/checkpoints/by_number"));
    assert!(path_without_full.contains("number=1500"));
    assert!(path_without_full.contains("full=false"));

    let path_with_full = api_path(&format!("{}?number={}&full={}", BY_NUMBER, 2000, true));
    assert!(path_with_full.contains("/v1/checkpoints/by_number"));
    assert!(path_with_full.contains("number=2000"));
    assert!(path_with_full.contains("full=true"));

    // Test get_checkpoint_by_hash path
    let hash_path_without_full = api_path(&format!(
        "{}?hash={}&full={}",
        CHECKPOINT_BY_HASH, "0x123abc", false
    ));
    assert!(hash_path_without_full.contains("/v1/checkpoints/by_hash"));
    assert!(hash_path_without_full.contains("hash=0x123abc"));
    assert!(hash_path_without_full.contains("full=false"));

    let hash_path_with_full = api_path(&format!(
        "{}?hash={}&full={}",
        CHECKPOINT_BY_HASH, "0x123abc", true
    ));
    assert!(hash_path_with_full.contains("/v1/checkpoints/by_hash"));
    assert!(hash_path_with_full.contains("hash=0x123abc"));
    assert!(hash_path_with_full.contains("full=true"));
}

#[test]
fn test_checkpoints_query_parameter_formatting() {
    // Test different checkpoint numbers
    let test_numbers = [0, 1, 100, 1000, 999999];
    for number in test_numbers {
        let path = api_path(&format!("{}?number={}&full=false", BY_NUMBER, number));
        assert!(path.contains(&format!("number={}", number)));
    }

    // Test boolean parameter formatting with checkpoint by hash
    for full_param in [true, false] {
        let path = api_path(&format!(
            "{}?hash={}&full={}",
            CHECKPOINT_BY_HASH, "0x123", full_param
        ));
        assert!(path.contains(&format!("full={}", full_param)));
        assert!(path.contains("hash=0x123"));
    }
}

//
// ============================================================================
// TOKENS API TESTS
// ============================================================================
//

#[test]
fn test_tokens_endpoint_constants() {
    // Test all endpoint constants are correct (without version prefix)
    assert_eq!(MINT, "/tokens/mint");
    assert_eq!(BURN, "/tokens/burn");
    assert_eq!(GRANT_AUTHORITY, "/tokens/grant_authority");
    assert_eq!(TOKEN_METADATA, "/tokens/token_metadata");
    assert_eq!(PAUSE, "/tokens/pause");
    assert_eq!(MANAGE_BLACKLIST, "/tokens/manage_blacklist");
    assert_eq!(MANAGE_WHITELIST, "/tokens/manage_whitelist");
    assert_eq!(UPDATE_METADATA, "/tokens/update_metadata");
}

#[test]
fn test_tokens_api_path_construction() {
    // Test path construction for all token endpoints (with version prefix)
    let mint_path = api_path(MINT);
    assert_eq!(mint_path, "/v1/tokens/mint");

    let burn_path = api_path(BURN);
    assert_eq!(burn_path, "/v1/tokens/burn");

    let grant_authority_path = api_path(GRANT_AUTHORITY);
    assert_eq!(grant_authority_path, "/v1/tokens/grant_authority");

    let token_metadata_path = api_path(TOKEN_METADATA);
    assert_eq!(token_metadata_path, "/v1/tokens/token_metadata");

    let pause_path = api_path(PAUSE);
    assert_eq!(pause_path, "/v1/tokens/pause");

    let blacklist_path = api_path(MANAGE_BLACKLIST);
    assert_eq!(blacklist_path, "/v1/tokens/manage_blacklist");

    let whitelist_path = api_path(MANAGE_WHITELIST);
    assert_eq!(whitelist_path, "/v1/tokens/manage_whitelist");

    let update_metadata_path = api_path(UPDATE_METADATA);
    assert_eq!(update_metadata_path, "/v1/tokens/update_metadata");
}

#[test]
fn test_tokens_query_parameter_construction() {
    // Test token metadata query with address
    let test_address = "0x1234567890abcdef1234567890abcdef12345678";
    let metadata_query_path = api_path(&format!("{}?address={}", TOKEN_METADATA, test_address));
    assert!(metadata_query_path.contains("/v1/tokens/token_metadata"));
    assert!(metadata_query_path.contains(&format!("address={}", test_address)));

    // Test different address formats
    let address_formats = [
        "0x1234567890abcdef1234567890abcdef12345678",
        "0xabcdef1234567890abcdef1234567890abcdef12",
        "0x0000000000000000000000000000000000000001",
    ];

    for address in address_formats {
        let path = api_path(&format!("{}?address={}", TOKEN_METADATA, address));
        assert!(path.contains(&format!("address={}", address)));
    }
}

//
// ============================================================================
// TRANSACTIONS API TESTS
// ============================================================================
//

#[test]
fn test_transactions_endpoint_constants() {
    // Test all endpoint constants are correct (without version prefix)
    assert_eq!(PAYMENT, "/transactions/payment");
    assert_eq!(TRANSACTION_BY_HASH, "/transactions/by_hash");
    assert_eq!(ESTIMATE_FEE, "/transactions/estimate_fee");
    assert_eq!(RECEIPT_BY_HASH, "/transactions/receipt/by_hash");
}

#[test]
fn test_transactions_api_path_construction() {
    // Test path construction for all transaction endpoints (with version prefix)
    let payment_path = api_path(PAYMENT);
    assert_eq!(payment_path, "/v1/transactions/payment");

    let by_hash_path = api_path(TRANSACTION_BY_HASH);
    assert_eq!(by_hash_path, "/v1/transactions/by_hash");

    let estimate_path = api_path(ESTIMATE_FEE);
    assert_eq!(estimate_path, "/v1/transactions/estimate_fee");

    let receipt_path = api_path(RECEIPT_BY_HASH);
    assert_eq!(receipt_path, "/v1/transactions/receipt/by_hash");
}

#[test]
fn test_transactions_query_parameter_construction() {
    // Test transaction by hash with various hash formats
    let test_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12";
    let by_hash_query = api_path(&format!("{}?hash={}", TRANSACTION_BY_HASH, test_hash));
    assert!(by_hash_query.contains("/v1/transactions/by_hash"));
    assert!(by_hash_query.contains(&format!("hash={}", test_hash)));

    // Test receipt by hash
    let receipt_query = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, test_hash));
    assert!(receipt_query.contains("/v1/transactions/receipt/by_hash"));
    assert!(receipt_query.contains(&format!("hash={}", test_hash)));

    // Test with different hash formats
    let hash_formats = [
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12",
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef",
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    ];

    for hash in hash_formats {
        let by_hash_path = api_path(&format!("{}?hash={}", TRANSACTION_BY_HASH, hash));
        assert!(by_hash_path.contains(&format!("hash={}", hash)));

        let receipt_path = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, hash));
        assert!(receipt_path.contains(&format!("hash={}", hash)));
    }
}

//
// ============================================================================
// CROSS-MODULE API INTEGRATION TESTS
// ============================================================================
//

#[test]
fn test_api_version_consistency() {
    // Test that all API paths have consistent versioning
    let all_paths = [
        api_path(NONCE),
        api_path(TOKEN_ACCOUNT),
        api_path(CHAIN_ID),
        api_path(BY_NUMBER),
        api_path(CHECKPOINT_BY_HASH),
        api_path(MINT),
        api_path(BURN),
        api_path(GRANT_AUTHORITY),
        api_path(TOKEN_METADATA),
        api_path(PAUSE),
        api_path(MANAGE_BLACKLIST),
        api_path(MANAGE_WHITELIST),
        api_path(UPDATE_METADATA),
        api_path(PAYMENT),
        api_path(TRANSACTION_BY_HASH),
        api_path(ESTIMATE_FEE),
        api_path(RECEIPT_BY_HASH),
    ];

    // All paths should start with /v1/
    for path in &all_paths {
        assert!(
            path.starts_with("/v1/"),
            "Path '{}' should start with '/v1/'",
            path
        );
    }
}

#[test]
fn test_api_path_uniqueness() {
    // Test that all API paths are unique
    let all_paths = [
        api_path(NONCE),
        api_path(TOKEN_ACCOUNT),
        api_path(CHAIN_ID),
        api_path(BY_NUMBER),
        api_path(CHECKPOINT_BY_HASH),
        api_path(MINT),
        api_path(BURN),
        api_path(GRANT_AUTHORITY),
        api_path(TOKEN_METADATA),
        api_path(PAUSE),
        api_path(MANAGE_BLACKLIST),
        api_path(MANAGE_WHITELIST),
        api_path(UPDATE_METADATA),
        api_path(PAYMENT),
        api_path(TRANSACTION_BY_HASH),
        api_path(ESTIMATE_FEE),
        api_path(RECEIPT_BY_HASH),
    ];

    // Convert to set and verify length is preserved (no duplicates)
    let mut unique_paths = std::collections::HashSet::new();
    for path in &all_paths {
        assert!(
            unique_paths.insert(path),
            "Duplicate API path found: {}",
            path
        );
    }
    assert_eq!(unique_paths.len(), all_paths.len());
}

#[test]
fn test_api_module_organization() {
    // Test that API paths are properly organized by module

    // Account APIs should contain "/accounts/"
    assert!(api_path(NONCE).contains("/accounts/"));
    assert!(api_path(TOKEN_ACCOUNT).contains("/accounts/"));

    // Chain APIs should contain "/chains/"
    assert!(api_path(CHAIN_ID).contains("/chains/"));

    // Checkpoint APIs should contain "/checkpoints/"
    assert!(api_path(BY_NUMBER).contains("/checkpoints/"));
    assert!(api_path(CHECKPOINT_BY_HASH).contains("/checkpoints/"));

    // Token APIs should contain "/tokens/"
    assert!(api_path(MINT).contains("/tokens/"));
    assert!(api_path(BURN).contains("/tokens/"));
    assert!(api_path(GRANT_AUTHORITY).contains("/tokens/"));
    assert!(api_path(TOKEN_METADATA).contains("/tokens/"));
    assert!(api_path(PAUSE).contains("/tokens/"));
    assert!(api_path(MANAGE_BLACKLIST).contains("/tokens/"));
    assert!(api_path(MANAGE_WHITELIST).contains("/tokens/"));
    assert!(api_path(UPDATE_METADATA).contains("/tokens/"));

    // Transaction APIs should contain "/transactions/"
    assert!(api_path(PAYMENT).contains("/transactions/"));
    assert!(api_path(TRANSACTION_BY_HASH).contains("/transactions/"));
    assert!(api_path(ESTIMATE_FEE).contains("/transactions/"));
    assert!(api_path(RECEIPT_BY_HASH).contains("/transactions/"));
}

//
// ============================================================================
// DATA FLOW VALIDATION TESTS
// ============================================================================
//

#[test]
fn test_address_parameter_validation() {
    // Test that address parameters are properly formatted in API paths
    let test_addresses = [
        "0x0000000000000000000000000000000000000000",
        "0x1234567890abcdef1234567890abcdef12345678",
        "0xffffffffffffffffffffffffffffffffffffffff",
    ];

    for address in test_addresses {
        // Test account nonce endpoint
        let nonce_path = api_path(&format!("{}?address={}", NONCE, address));
        assert!(nonce_path.contains(&format!("address={}", address)));

        // Test token account endpoint
        let token_path = api_path(&format!(
            "{}?owner={}&token={}",
            TOKEN_ACCOUNT, address, address
        ));
        assert!(token_path.contains(&format!("owner={}", address)));
        assert!(token_path.contains(&format!("token={}", address)));

        // Test token metadata endpoint
        let metadata_path = api_path(&format!("{}?address={}", TOKEN_METADATA, address));
        assert!(metadata_path.contains(&format!("address={}", address)));
    }
}

#[test]
fn test_hash_parameter_validation() {
    // Test that hash parameters are properly formatted in API paths
    let test_hashes = [
        "0x0000000000000000000000000000000000000000000000000000000000000000",
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ];

    for hash in test_hashes {
        // Test transaction by hash
        let by_hash_path = api_path(&format!("{}?hash={}", TRANSACTION_BY_HASH, hash));
        assert!(by_hash_path.contains(&format!("hash={}", hash)));

        // Test receipt by hash
        let receipt_path = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, hash));
        assert!(receipt_path.contains(&format!("hash={}", hash)));
    }
}

#[test]
fn test_numeric_parameter_validation() {
    // Test that numeric parameters are properly formatted in API paths
    let test_numbers = [0, 1, 100, 1000, 999999, u64::MAX];

    for number in test_numbers {
        // Test checkpoint by number
        let checkpoint_path = api_path(&format!("{}?number={}&full=false", BY_NUMBER, number));
        assert!(checkpoint_path.contains(&format!("number={}", number)));
    }
}
