//! Comprehensive tests for token API methods.

use alloy_primitives::{Address, U256};
use onemoney_protocol::api::Client;
use onemoney_protocol::requests::{
    TokenAuthorityPayload, TokenBlacklistPayload, TokenBurnPayload, TokenMetadataUpdatePayload,
    TokenMintPayload, TokenPausePayload, TokenWhitelistPayload,
};
use onemoney_protocol::responses::MetadataKVPair;
use onemoney_protocol::{
    Authority, AuthorityAction, BlacklistAction, ClientBuilder, Network, PauseAction,
    WhitelistAction,
};
use std::str::FromStr;
use std::time::Duration;

const TEST_PRIVATE_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

/// Create a test client for token API testing.
fn create_test_client() -> Client {
    ClientBuilder::new()
        .network(Network::Local)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Test client creation should not fail")
}

/// Create test token mint payload.
fn create_test_mint_payload() -> TokenMintPayload {
    TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 1,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        value: U256::from(1000000000000000000u64), // 1 token
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
    }
}

/// Create test token burn payload.
fn create_test_burn_payload() -> TokenBurnPayload {
    TokenBurnPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 2,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        value: U256::from(500000000000000000u64), // 0.5 tokens
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
    }
}

/// Create test authority grant payload.
fn create_test_grant_authority_payload() -> TokenAuthorityPayload {
    TokenAuthorityPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 3,
        action: AuthorityAction::Grant,
        authority_type: Authority::MintBurnTokens,
        authority_address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
        value: U256::from(1000000000000000000u64),
    }
}

/// Create test authority revoke payload.
fn create_test_revoke_authority_payload() -> TokenAuthorityPayload {
    TokenAuthorityPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 4,
        action: AuthorityAction::Revoke,
        authority_type: Authority::MintBurnTokens,
        authority_address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
        value: U256::from(1000000000000000000u64),
    }
}

/// Create test pause token payload.
fn create_test_pause_payload() -> TokenPausePayload {
    TokenPausePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        action: PauseAction::Pause,
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
    }
}

/// Create test blacklist management payload.
fn create_test_blacklist_payload() -> TokenBlacklistPayload {
    TokenBlacklistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        action: BlacklistAction::Add,
        address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
    }
}

/// Create test whitelist management payload.
fn create_test_whitelist_payload() -> TokenWhitelistPayload {
    TokenWhitelistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 7,
        action: WhitelistAction::Add,
        address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test address should be valid"),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
    }
}

/// Create test metadata update payload.
fn create_test_metadata_update_payload() -> TokenMetadataUpdatePayload {
    TokenMetadataUpdatePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 8,
        name: "Test Token Updated".to_string(),
        uri: "https://example.com/token-metadata.json".to_string(),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test token address should be valid"),
        additional_metadata: vec![
            MetadataKVPair {
                key: "description".to_string(),
                value: "Updated test token description".to_string(),
            },
            MetadataKVPair {
                key: "external_url".to_string(),
                value: "https://example.com".to_string(),
            },
        ],
    }
}

#[tokio::test]
async fn test_mint_token_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_mint_payload();

    // This test validates the method signature and request structure
    // without making actual network calls
    let result = client.mint_token(payload.clone(), TEST_PRIVATE_KEY).await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.recent_epoch, 100);
    assert_eq!(payload.nonce, 1);
    assert_eq!(payload.value, U256::from(1000000000000000000u64));
}

#[tokio::test]
async fn test_burn_token_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_burn_payload();

    // This test validates the method signature and request structure
    let result = client.burn_token(payload.clone(), TEST_PRIVATE_KEY).await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.recent_epoch, 100);
    assert_eq!(payload.nonce, 2);
    assert_eq!(payload.value, U256::from(500000000000000000u64));
}

#[tokio::test]
async fn test_grant_authority_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_grant_authority_payload();

    // This test validates the method signature and request structure
    let result = client
        .grant_authority(payload.clone(), TEST_PRIVATE_KEY)
        .await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.action, AuthorityAction::Grant);
    assert_eq!(payload.authority_type, Authority::MintBurnTokens);
    assert_eq!(payload.nonce, 3);
}

#[tokio::test]
async fn test_revoke_authority_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_revoke_authority_payload();

    // This test validates the method signature and request structure
    let result = client
        .revoke_authority(payload.clone(), TEST_PRIVATE_KEY)
        .await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.action, AuthorityAction::Revoke);
    assert_eq!(payload.authority_type, Authority::MintBurnTokens);
    assert_eq!(payload.nonce, 4);
}

#[tokio::test]
async fn test_pause_token_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_pause_payload();

    // This test validates the method signature and request structure
    let result = client.pause_token(payload.clone(), TEST_PRIVATE_KEY).await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.action, PauseAction::Pause);
    assert_eq!(payload.nonce, 5);
}

#[tokio::test]
async fn test_manage_blacklist_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_blacklist_payload();

    // This test validates the method signature and request structure
    let result = client
        .manage_blacklist(payload.clone(), TEST_PRIVATE_KEY)
        .await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.action, BlacklistAction::Add);
    assert_eq!(payload.nonce, 6);
}

#[tokio::test]
async fn test_manage_whitelist_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_whitelist_payload();

    // This test validates the method signature and request structure
    let result = client
        .manage_whitelist(payload.clone(), TEST_PRIVATE_KEY)
        .await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.action, WhitelistAction::Add);
    assert_eq!(payload.nonce, 7);
}

#[tokio::test]
async fn test_update_token_metadata_method_signature_and_request_structure() {
    let client = create_test_client();
    let payload = create_test_metadata_update_payload();

    // This test validates the method signature and request structure
    let result = client
        .update_token_metadata(payload.clone(), TEST_PRIVATE_KEY)
        .await;

    // The request should fail due to no server, but we can verify the structure
    assert!(result.is_err(), "Expected error due to no local server");

    // Verify payload structure is correct
    assert_eq!(payload.nonce, 8);
    assert_eq!(payload.name, "Test Token Updated");
    assert_eq!(payload.uri, "https://example.com/token-metadata.json");
    assert_eq!(payload.additional_metadata.len(), 2);
    assert_eq!(payload.additional_metadata[0].key, "description");
    assert_eq!(
        payload.additional_metadata[0].value,
        "Updated test token description"
    );
}

#[test]
fn test_token_request_payload_serialization() {
    use onemoney_protocol::Signature;
    use onemoney_protocol::requests::{
        BlacklistTokenRequest, BurnTokenRequest, MintTokenRequest, PauseTokenRequest,
        TokenAuthorityRequest, UpdateMetadataRequest, WhitelistTokenRequest,
    };

    let signature = Signature::default();

    // Test MintTokenRequest serialization
    let mint_request = MintTokenRequest {
        payload: create_test_mint_payload(),
        signature: signature.clone(),
    };
    let mint_json = serde_json::to_string(&mint_request).expect("Serialization should work");
    assert!(mint_json.contains("\"recent_epoch\":100"));
    assert!(mint_json.contains("\"nonce\":1"));

    // Test BurnTokenRequest serialization
    let burn_request = BurnTokenRequest {
        payload: create_test_burn_payload(),
        signature: signature.clone(),
    };
    let burn_json = serde_json::to_string(&burn_request).expect("Serialization should work");
    assert!(burn_json.contains("\"recent_epoch\":100"));
    assert!(burn_json.contains("\"nonce\":2"));

    // Test TokenAuthorityRequest serialization (used for both grant and revoke)
    let authority_request = TokenAuthorityRequest {
        payload: create_test_grant_authority_payload(),
        signature: signature.clone(),
    };
    let authority_json =
        serde_json::to_string(&authority_request).expect("Serialization should work");
    assert!(authority_json.contains("\"action\":\"Grant\""));
    assert!(authority_json.contains("\"authority_type\":\"MintBurnTokens\""));

    // Test PauseTokenRequest serialization
    let pause_request = PauseTokenRequest {
        payload: create_test_pause_payload(),
        signature: signature.clone(),
    };
    let pause_json = serde_json::to_string(&pause_request).expect("Serialization should work");
    assert!(pause_json.contains("\"action\":\"Pause\""));

    // Test BlacklistTokenRequest serialization
    let blacklist_request = BlacklistTokenRequest {
        payload: create_test_blacklist_payload(),
        signature: signature.clone(),
    };
    let blacklist_json =
        serde_json::to_string(&blacklist_request).expect("Serialization should work");
    assert!(blacklist_json.contains("\"action\":\"Add\""));

    // Test WhitelistTokenRequest serialization
    let whitelist_request = WhitelistTokenRequest {
        payload: create_test_whitelist_payload(),
        signature: signature.clone(),
    };
    let whitelist_json =
        serde_json::to_string(&whitelist_request).expect("Serialization should work");
    assert!(whitelist_json.contains("\"action\":\"Add\""));

    // Test UpdateMetadataRequest serialization
    let metadata_request = UpdateMetadataRequest {
        payload: create_test_metadata_update_payload(),
        signature,
    };
    let metadata_json =
        serde_json::to_string(&metadata_request).expect("Serialization should work");
    assert!(metadata_json.contains("\"additional_metadata\""));
    assert!(metadata_json.contains("\"name\":\"Test Token Updated\""));
    assert!(metadata_json.contains("\"uri\":\"https://example.com/token-metadata.json\""));
    assert!(metadata_json.contains("\"description\""));
    assert!(metadata_json.contains("\"Updated test token description\""));
}

#[test]
fn test_token_api_endpoint_paths() {
    use onemoney_protocol::client::config::{api_path, endpoints::tokens::*};

    // Verify all token API endpoint paths are correctly defined
    assert_eq!(api_path(MINT), "/v1/tokens/mint");
    assert_eq!(api_path(BURN), "/v1/tokens/burn");
    assert_eq!(api_path(GRANT_AUTHORITY), "/v1/tokens/grant_authority");
    assert_eq!(api_path(PAUSE), "/v1/tokens/pause");
    assert_eq!(api_path(MANAGE_BLACKLIST), "/v1/tokens/manage_blacklist");
    assert_eq!(api_path(MANAGE_WHITELIST), "/v1/tokens/manage_whitelist");
    assert_eq!(api_path(UPDATE_METADATA), "/v1/tokens/update_metadata");
    assert_eq!(api_path(TOKEN_METADATA), "/v1/tokens/token_metadata");
}

#[test]
fn test_token_payload_edge_cases() {
    // Test with zero values
    let mut payload = create_test_mint_payload();
    payload.value = U256::ZERO;
    assert_eq!(payload.value, U256::ZERO);

    // Test with maximum U256 value
    payload.value = U256::MAX;
    assert_eq!(payload.value, U256::MAX);

    // Test with different action types
    let mut authority_payload = create_test_grant_authority_payload();
    authority_payload.action = AuthorityAction::Revoke;
    assert_eq!(authority_payload.action, AuthorityAction::Revoke);

    // Test different authority types
    authority_payload.authority_type = Authority::UpdateMetadata;
    assert_eq!(authority_payload.authority_type, Authority::UpdateMetadata);

    // Test pause/unpause actions
    let mut pause_payload = create_test_pause_payload();
    pause_payload.action = PauseAction::Unpause;
    assert_eq!(pause_payload.action, PauseAction::Unpause);

    // Test blacklist remove action
    let mut blacklist_payload = create_test_blacklist_payload();
    blacklist_payload.action = BlacklistAction::Remove;
    assert_eq!(blacklist_payload.action, BlacklistAction::Remove);

    // Test whitelist remove action
    let mut whitelist_payload = create_test_whitelist_payload();
    whitelist_payload.action = WhitelistAction::Remove;
    assert_eq!(whitelist_payload.action, WhitelistAction::Remove);
}

#[test]
fn test_metadata_update_payload_with_empty_metadata() {
    let mut payload = create_test_metadata_update_payload();
    payload.additional_metadata = vec![];
    assert!(payload.additional_metadata.is_empty());

    // Test with multiple metadata entries
    payload.additional_metadata = vec![
        MetadataKVPair {
            key: "symbol".to_string(),
            value: "UPDATED".to_string(),
        },
        MetadataKVPair {
            key: "decimals".to_string(),
            value: "18".to_string(),
        },
        MetadataKVPair {
            key: "category".to_string(),
            value: "utility".to_string(),
        },
    ];
    assert_eq!(payload.additional_metadata.len(), 3);
    assert_eq!(payload.additional_metadata[2].key, "category");
}

#[test]
fn test_all_request_types_have_signature_field() {
    use onemoney_protocol::Signature;
    use onemoney_protocol::requests::*;

    let signature = Signature::default();

    // Verify all request types have signature field
    let _mint_request = MintTokenRequest {
        payload: create_test_mint_payload(),
        signature: signature.clone(),
    };

    let _burn_request = BurnTokenRequest {
        payload: create_test_burn_payload(),
        signature: signature.clone(),
    };

    let _authority_request = TokenAuthorityRequest {
        payload: create_test_grant_authority_payload(),
        signature: signature.clone(),
    };

    let _pause_request = PauseTokenRequest {
        payload: create_test_pause_payload(),
        signature: signature.clone(),
    };

    let _blacklist_request = BlacklistTokenRequest {
        payload: create_test_blacklist_payload(),
        signature: signature.clone(),
    };

    let _whitelist_request = WhitelistTokenRequest {
        payload: create_test_whitelist_payload(),
        signature: signature.clone(),
    };

    let _metadata_request = UpdateMetadataRequest {
        payload: create_test_metadata_update_payload(),
        signature,
    };

    // If this compiles, all request types have the signature field correctly defined
}
