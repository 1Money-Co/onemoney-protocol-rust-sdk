//! Comprehensive tokens API coverage tests

use alloy_primitives::{Address, U256};
use onemoney_protocol::client::config::api_path;
use onemoney_protocol::client::config::endpoints::tokens::*;
use onemoney_protocol::client::http::Client;
use onemoney_protocol::*;
use std::str::FromStr;

#[test]
fn test_token_endpoints_constants() {
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
fn test_token_api_path_construction() {
    // Test path construction for all token endpoints (with version prefix)
    let mint_path = api_path(MINT);
    assert_eq!(mint_path, "/v1/tokens/mint");

    let burn_path = api_path(BURN);
    assert_eq!(burn_path, "/v1/tokens/burn");

    let authority_path = api_path(GRANT_AUTHORITY);
    assert_eq!(authority_path, "/v1/tokens/grant_authority");

    let pause_path = api_path(PAUSE);
    assert_eq!(pause_path, "/v1/tokens/pause");

    let blacklist_path = api_path(MANAGE_BLACKLIST);
    assert_eq!(blacklist_path, "/v1/tokens/manage_blacklist");

    let whitelist_path = api_path(MANAGE_WHITELIST);
    assert_eq!(whitelist_path, "/v1/tokens/manage_whitelist");

    let update_path = api_path(UPDATE_METADATA);
    assert_eq!(update_path, "/v1/tokens/update_metadata");
}

#[test]
fn test_token_metadata_path_construction() {
    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");
    let path = api_path(&format!("{}?token={}", TOKEN_METADATA, token_address));

    assert!(path.contains("/v1/tokens/token_metadata"));
    assert!(path.contains("token=0x1234567890AbcdEF1234567890aBcdef12345678"));
    assert!(path.contains("?"));
}

#[test]
fn test_token_mint_payload_comprehensive() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    // Test all fields
    assert_eq!(payload.recent_epoch, 100);
    assert_eq!(payload.recent_checkpoint, 200);
    assert_eq!(payload.chain_id, 1212101);
    assert_eq!(payload.nonce, 5);
    assert_eq!(payload.recipient, recipient);
    assert_eq!(payload.value, U256::from(1000000000000000000u64));
    assert_eq!(payload.token, token);

    // Test serialization
    let json = serde_json::to_string(&payload).expect("Test data should be valid");
    let deserialized: TokenMintPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
    assert_eq!(payload.recipient, deserialized.recipient);
    assert_eq!(payload.token, deserialized.token);

    // Test signature hash
    let hash = payload.signature_hash();
    assert_ne!(hash, alloy_primitives::B256::default());

    // Test deterministic hash
    let hash2 = payload.signature_hash();
    assert_eq!(hash, hash2);
}

#[test]
fn test_token_burn_payload_comprehensive() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = TokenBurnPayload {
        recent_epoch: 150,
        recent_checkpoint: 250,
        chain_id: 1212101,
        nonce: 10,
        recipient,
        value: U256::from(500000000000000000u64),
        token,
    };

    // Test serialization/deserialization
    let json = serde_json::to_string(&payload).expect("Test data should be valid");
    let deserialized: TokenBurnPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
    assert_eq!(payload.recent_checkpoint, deserialized.recent_checkpoint);
    assert_eq!(payload.chain_id, deserialized.chain_id);
    assert_eq!(payload.nonce, deserialized.nonce);
    assert_eq!(payload.recipient, deserialized.recipient);
    assert_eq!(payload.value, deserialized.value);
    assert_eq!(payload.token, deserialized.token);

    // Test signature hash
    let hash = payload.signature_hash();
    assert_ne!(hash, alloy_primitives::B256::default());
}

#[test]
fn test_token_authority_payload_comprehensive() {
    let authority_address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test Grant action
    let grant_payload = TokenAuthorityPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        action: AuthorityAction::Grant,
        authority_type: Authority::MintBurnTokens,
        authority_address,
        token,
        value: U256::from(1000000000000000000u64),
    };

    // Test Revoke action
    let revoke_payload = TokenAuthorityPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        action: AuthorityAction::Revoke,
        authority_type: Authority::Pause,
        authority_address,
        token,
        value: U256::from(1000000000000000000u64),
    };

    // Test different authority types
    let authority_types = [
        Authority::MasterMintBurn,
        Authority::MintBurnTokens,
        Authority::Pause,
        Authority::ManageList,
        Authority::UpdateMetadata,
    ];

    for auth_type in authority_types {
        let payload = TokenAuthorityPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 7,
            action: AuthorityAction::Grant,
            authority_type: auth_type,
            authority_address,
            token,
            value: U256::from(1000000000000000000u64),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");
        assert!(json.contains("Grant"));

        let hash = payload.signature_hash();
        assert_ne!(hash, alloy_primitives::B256::default());
    }

    // Test serialization
    let json = serde_json::to_string(&grant_payload).expect("Test data should be valid");
    let deserialized: TokenAuthorityPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(grant_payload.action, deserialized.action);
    assert_eq!(grant_payload.authority_type, deserialized.authority_type);
    assert_eq!(
        grant_payload.authority_address,
        deserialized.authority_address
    );

    // Test hashes are different for different actions
    let grant_hash = grant_payload.signature_hash();
    let revoke_hash = revoke_payload.signature_hash();
    assert_ne!(grant_hash, revoke_hash);
}

#[test]
fn test_token_pause_payload_comprehensive() {
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test Pause action
    let pause_payload = TokenPausePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        action: PauseAction::Pause,
        token,
    };

    // Test Unpause action
    let unpause_payload = TokenPausePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        action: PauseAction::Unpause,
        token,
    };

    // Test serialization for both actions
    for payload in [&pause_payload, &unpause_payload] {
        let json = serde_json::to_string(payload).expect("Test data should be valid");
        let deserialized: TokenPausePayload =
            serde_json::from_str(&json).expect("Test data should be valid");
        assert_eq!(payload.action, deserialized.action);
        assert_eq!(payload.token, deserialized.token);
        assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
        assert_eq!(payload.recent_checkpoint, deserialized.recent_checkpoint);
        assert_eq!(payload.chain_id, deserialized.chain_id);
        assert_eq!(payload.nonce, deserialized.nonce);

        // Test signature hash
        let hash = payload.signature_hash();
        assert_ne!(hash, alloy_primitives::B256::default());
    }

    // Test different actions produce different hashes
    let pause_hash = pause_payload.signature_hash();
    let unpause_hash = unpause_payload.signature_hash();
    assert_ne!(pause_hash, unpause_hash);
}

#[test]
fn test_token_blacklist_payload_comprehensive() {
    let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test Add action
    let add_payload = TokenBlacklistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        action: BlacklistAction::Add,
        address,
        token,
    };

    // Test Remove action
    let remove_payload = TokenBlacklistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        action: BlacklistAction::Remove,
        address,
        token,
    };

    // Test serialization for both actions
    for payload in [&add_payload, &remove_payload] {
        let json = serde_json::to_string(payload).expect("Test data should be valid");
        let deserialized: TokenBlacklistPayload =
            serde_json::from_str(&json).expect("Test data should be valid");
        assert_eq!(payload.action, deserialized.action);
        assert_eq!(payload.address, deserialized.address);
        assert_eq!(payload.token, deserialized.token);
        assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
        assert_eq!(payload.recent_checkpoint, deserialized.recent_checkpoint);
        assert_eq!(payload.chain_id, deserialized.chain_id);
        assert_eq!(payload.nonce, deserialized.nonce);

        // Test signature hash
        let hash = payload.signature_hash();
        assert_ne!(hash, alloy_primitives::B256::default());
    }

    // Test different actions produce different hashes
    let add_hash = add_payload.signature_hash();
    let remove_hash = remove_payload.signature_hash();
    assert_ne!(add_hash, remove_hash);
}

#[test]
fn test_token_whitelist_payload_comprehensive() {
    let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test Add action
    let add_payload = TokenWhitelistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        action: WhitelistAction::Add,
        address,
        token,
    };

    // Test Remove action
    let remove_payload = TokenWhitelistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        action: WhitelistAction::Remove,
        address,
        token,
    };

    // Test serialization for both actions
    for payload in [&add_payload, &remove_payload] {
        let json = serde_json::to_string(payload).expect("Test data should be valid");
        let deserialized: TokenWhitelistPayload =
            serde_json::from_str(&json).expect("Test data should be valid");
        assert_eq!(payload.action, deserialized.action);
        assert_eq!(payload.address, deserialized.address);
        assert_eq!(payload.token, deserialized.token);
        assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
        assert_eq!(payload.recent_checkpoint, deserialized.recent_checkpoint);
        assert_eq!(payload.chain_id, deserialized.chain_id);
        assert_eq!(payload.nonce, deserialized.nonce);

        // Test signature hash
        let hash = payload.signature_hash();
        assert_ne!(hash, alloy_primitives::B256::default());
    }

    // Test different actions produce different hashes
    let add_hash = add_payload.signature_hash();
    let remove_hash = remove_payload.signature_hash();
    assert_ne!(add_hash, remove_hash);
}

#[test]
fn test_token_metadata_update_payload_comprehensive() {
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = TokenMetadataUpdatePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        token,
        name: "Updated Token Name".to_string(),
        uri: "https://example.com/token-metadata.json".to_string(),
        additional_metadata: vec![
            MetadataKVPair {
                key: "description".to_string(),
                value: "Updated token description".to_string(),
            },
            MetadataKVPair {
                key: "image".to_string(),
                value: "https://example.com/token-image.png".to_string(),
            },
        ],
    };

    // Test serialization/deserialization
    let json = serde_json::to_string(&payload).expect("Test data should be valid");
    let deserialized: TokenMetadataUpdatePayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payload.token, deserialized.token);
    assert_eq!(payload.name, deserialized.name);
    assert_eq!(payload.uri, deserialized.uri);
    assert_eq!(
        payload.additional_metadata,
        deserialized.additional_metadata
    );

    // Test signature hash
    let hash = payload.signature_hash();
    assert_ne!(hash, alloy_primitives::B256::default());

    // Test with empty additional metadata
    let minimal_payload = TokenMetadataUpdatePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6,
        token,
        name: "Minimal Token".to_string(),
        uri: "https://example.com/minimal.json".to_string(),
        additional_metadata: vec![],
    };

    let json2 = serde_json::to_string(&minimal_payload).expect("Test data should be valid");
    let deserialized2: TokenMetadataUpdatePayload =
        serde_json::from_str(&json2).expect("Test data should be valid");
    assert_eq!(minimal_payload.additional_metadata.len(), 0);
    assert_eq!(deserialized2.additional_metadata.len(), 0);

    // Test different payloads produce different hashes
    let hash1 = payload.signature_hash();
    let hash2 = minimal_payload.signature_hash();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_mint_info_response_structure() {
    let master_authority = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let master_mint_burn = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let mint_info = MintInfo {
        symbol: "TEST".to_string(),
        master_authority,
        master_mint_burn_authority: master_mint_burn,
        mint_burn_authorities: vec![MinterAllowance {
            minter: Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                .expect("Test data should be valid"),
            allowance: "1000000000000000000000".to_string(),
        }],
        pause_authorities: vec![master_authority],
        list_authorities: vec![master_authority],
        black_list: vec![],
        white_list: vec![],
        metadata_update_authorities: vec![master_authority],
        supply: "1000000000000000000000".to_string(),
        decimals: 18,
        is_paused: false,
        is_private: false,
        meta: Some(TokenMetadata {
            name: "Test Token".to_string(),
            uri: "https://example.com/test.json".to_string(),
            additional_metadata: vec![MetadataKVPair {
                key: "description".to_string(),
                value: "A test token for testing".to_string(),
            }],
        }),
    };

    // Test serialization/deserialization
    let json = serde_json::to_string(&mint_info).expect("Test data should be valid");
    let deserialized: MintInfo = serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(mint_info.symbol, deserialized.symbol);
    assert_eq!(mint_info.master_authority, deserialized.master_authority);
    assert_eq!(mint_info.decimals, deserialized.decimals);
    assert_eq!(mint_info.supply, deserialized.supply);
    assert_eq!(mint_info.is_paused, deserialized.is_paused);
    assert_eq!(mint_info.is_private, deserialized.is_private);

    // Test display implementation
    let display_str = format!("{}", mint_info);
    assert!(display_str.contains("TEST"));
    assert!(display_str.contains("18"));
    assert!(display_str.contains("1000000000000000000000"));
}

#[test]
fn test_token_requests_serialization() {
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let signature = Signature {
        r: U256::from(12345u64),
        s: U256::from(67890u64),
        v: 27,
    };

    // Test MintTokenRequest
    let mint_request = MintTokenRequest {
        payload: TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient,
            value: U256::from(1000000000000000000u64),
            token,
        },
        signature: signature.clone(),
    };

    let json = serde_json::to_string(&mint_request).expect("Test data should be valid");
    assert!(json.contains("recent_epoch"));
    assert!(json.contains("100"));
    assert!(json.contains("signature"));

    // Test BurnTokenRequest
    let burn_request = BurnTokenRequest {
        payload: TokenBurnPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient,
            value: U256::from(500000000000000000u64),
            token,
        },
        signature: signature.clone(),
    };

    let json2 = serde_json::to_string(&burn_request).expect("Test data should be valid");
    assert!(json2.contains("recent_epoch"));
    assert!(json2.contains("signature"));
}

#[test]
fn test_client_creation_for_token_operations() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    // Test that clients can be created for token operations
    let mainnet_client = Client::mainnet()?;
    let testnet_client = Client::testnet()?;
    let local_client = Client::local()?;

    // Verify they're client instances (via debug output)
    let mainnet_debug = format!("{:?}", mainnet_client);
    let testnet_debug = format!("{:?}", testnet_client);
    let local_debug = format!("{:?}", local_client);

    assert!(mainnet_debug.contains("Client"));
    assert!(testnet_debug.contains("Client"));
    assert!(local_debug.contains("Client"));

    Ok(())
}

#[test]
fn test_payload_edge_cases() {
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");

    // Test with zero values
    let zero_payload = TokenMintPayload {
        recent_epoch: 0,
        recent_checkpoint: 0,
        chain_id: 1212101,
        nonce: 0,
        recipient,
        value: U256::ZERO,
        token,
    };

    let hash_zero = zero_payload.signature_hash();
    assert_ne!(hash_zero, alloy_primitives::B256::default());

    // Test with maximum values
    let max_payload = TokenMintPayload {
        recent_epoch: u64::MAX,
        recent_checkpoint: u64::MAX,
        chain_id: 1212101,
        nonce: u64::MAX,
        recipient,
        value: U256::MAX,
        token,
    };

    let hash_max = max_payload.signature_hash();
    assert_ne!(hash_max, alloy_primitives::B256::default());
    assert_ne!(hash_zero, hash_max);
}
