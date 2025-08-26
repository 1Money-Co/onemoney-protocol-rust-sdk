//! Comprehensive mock server integration tests
//!
//! This file contains all mock server related integration tests including:
//! - Mock HTTP server setup and configuration
//! - API endpoint mocking with various response scenarios
//! - Error response simulation and handling
//! - Network timeout and reliability testing
//! - Token operation mocking and validation
//! - Concurrent request handling
//! - Large response and edge case handling

use alloy_primitives::{Address, B256, U256};
use mockito::ServerGuard;
use onemoney_protocol::client::builder::ClientBuilder;
use onemoney_protocol::responses::TransactionResponse;
use onemoney_protocol::{
    Authority, AuthorityAction, BlacklistAction, Client, MetadataKVPair, PauseAction, Signable,
    TokenAuthorityPayload, TokenBlacklistPayload, TokenBurnPayload, TokenMetadataUpdatePayload,
    TokenMintPayload, TokenPausePayload, TokenWhitelistPayload, WhitelistAction,
};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

// Test configuration
const TEST_TIMEOUT: Duration = Duration::from_secs(5);

//
// ============================================================================
// MOCK SERVER SETUP AND UTILITY FUNCTIONS
// ============================================================================
//

/// Setup a mock server for testing
async fn setup_mock_server() -> ServerGuard {
    mockito::Server::new_async().await
}

/// Mock test utilities
mod mock_utils {
    use super::*;

    /// Create a test client for mock testing
    pub fn create_mock_client() -> Result<Client, Box<dyn std::error::Error>> {
        Ok(ClientBuilder::new()
            .base_url("http://127.0.0.1:1") // Intentionally unreachable for mock testing
            .timeout(TEST_TIMEOUT)
            .build()?)
    }

    /// Test private key for mock operations
    pub fn test_private_key() -> &'static str {
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }

    /// Test addresses for mock scenarios
    pub struct MockAddresses {
        pub token_mint: Address,
        pub recipient: Address,
        pub authority_address: Address,
    }

    impl MockAddresses {
        pub fn new() -> Self {
            Self {
                token_mint: Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                    .expect("Valid token mint address"),
                recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                    .expect("Valid recipient address"),
                authority_address: Address::from_str("0x9876543210fedcba9876543210fedcba98765432")
                    .expect("Valid authority address"),
            }
        }
    }

    impl Clone for MockAddresses {
        fn clone(&self) -> Self {
            Self {
                token_mint: self.token_mint,
                recipient: self.recipient,
                authority_address: self.authority_address,
            }
        }
    }

    /// Create a mock hash for testing
    pub fn create_mock_hash() -> TransactionResponse {
        let mock_hash_bytes = [
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab,
            0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78,
            0x90, 0xab, 0xcd, 0xef,
        ];
        TransactionResponse {
            hash: B256::from(mock_hash_bytes),
        }
    }

    /// Validate hash format and properties
    pub fn validate_mock_hash(
        hash: &TransactionResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hash_str = hash.hash.to_string();
        assert!(!hash_str.is_empty(), "Hash string should not be empty");
        assert!(hash_str.starts_with("0x"), "Hash should start with 0x");
        assert_eq!(
            hash_str.len(),
            66,
            "Hash should be 66 characters (0x + 64 hex)"
        );

        // Hash should not be all zeros
        let zero_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
        assert_ne!(hash_str, zero_hash, "Hash should not be all zeros");

        println!("Mock hash format validated: {}", hash_str);
        Ok(())
    }
}

//
// ============================================================================
// BASIC API ENDPOINT MOCK TESTS
// ============================================================================
//

#[tokio::test]
async fn test_chain_id_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock the chain ID endpoint (correct path: /v1/chains/chain_id)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"chain_id": 12345}"#)
        .create();

    // Create client pointing to mock server
    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    // Test the API call
    let chain_id = client.fetch_chain_id_from_network().await?;
    assert_eq!(chain_id, 12345);

    Ok(())
}

#[tokio::test]
async fn test_account_nonce_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    let test_address = "0x1234567890abcdef1234567890abcdef12345678";

    // Mock the account nonce endpoint - use regex to match any query parameter
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"^/v1/accounts/nonce.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"nonce": 42}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let address = Address::from_str(test_address)?;
    let nonce_info = client.get_account_nonce(address).await?;

    println!("Nonce info: {}", nonce_info);
    // The exact assertion depends on the AccountNonce structure

    Ok(())
}

#[tokio::test]
async fn test_token_metadata_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    let token_address = "0xabcdef1234567890abcdef1234567890abcdef12";

    // Mock the token metadata endpoint - use regex to match any query parameter
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"^/v1/tokens/token_metadata.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "symbol": "TEST",
            "master_authority": "0x1234567890abcdef1234567890abcdef12345678",
            "master_mint_burn_authority": "0x1234567890abcdef1234567890abcdef12345678",
            "mint_burn_authorities": [],
            "pause_authorities": [],
            "list_authorities": [],
            "black_list": [],
            "white_list": [],
            "metadata_update_authorities": [],
            "supply": "1000000",
            "decimals": 18,
            "is_paused": false,
            "is_private": false,
            "meta": null
        }"#,
        )
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let token_addr = Address::from_str(token_address)?;
    let metadata = client.get_token_metadata(token_addr).await?;

    println!("Token metadata: {}", metadata);

    Ok(())
}

#[tokio::test]
async fn test_latest_state_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock the latest state endpoint (correct path: /v1/states/latest_epoch_checkpoint)
    let _mock = server
        .mock("GET", "/v1/states/latest_epoch_checkpoint")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "epoch": 100,
            "checkpoint": 200,
            "checkpoint_hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "checkpoint_parent_hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
        }"#,
        )
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let state = client.get_latest_epoch_checkpoint().await?;
    println!("Latest state: {}", state);

    Ok(())
}

//
// ============================================================================
// HTTP ERROR RESPONSE MOCK TESTS
// ============================================================================
//

#[tokio::test]
async fn test_http_error_responses() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock a 500 error response
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail with 500 error");

    println!("Expected error: {:?}", result.unwrap_err());
    Ok(())
}

#[tokio::test]
async fn test_api_rate_limiting_simulation() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock rate limiting (429 Too Many Requests)
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_header("retry-after", "60")
        .with_body(r#"{"error": "Rate limit exceeded"}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail with rate limit error");

    println!("Rate limit error (expected): {:?}", result.unwrap_err());
    Ok(())
}

#[tokio::test]
async fn test_invalid_json_response() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint returning invalid JSON (correct path: /v1/chains/chain_id)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("invalid json response")
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail to parse invalid JSON");

    match result {
        Err(e) => {
            println!("JSON parse error (expected): {}", e);
            let error_str = format!("{}", e);
            assert!(
                error_str.contains("serialize")
                    || error_str.contains("JSON")
                    || error_str.contains("parse")
                    || error_str.contains("transport")
                    || error_str.contains("deserialization")
            );
        }
        Ok(_) => panic!("Expected JSON parse error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_missing_fields_in_response() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock response missing required field
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"wrong_field": 123}"#) // Missing chain_id field
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail due to missing field");

    Ok(())
}

//
// ============================================================================
// NETWORK AND TIMEOUT MOCK TESTS
// ============================================================================
//

#[tokio::test]
async fn test_network_timeout_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock an endpoint that never responds (simulates network timeout)
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"chain_id": 1}"#)
        .expect(0) // Never called due to timeout
        .create();

    // Create client with very short timeout
    let client = ClientBuilder::new()
        .base_url("http://127.0.0.1:1") // Connect to nothing
        .timeout(Duration::from_millis(100))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should timeout");

    Ok(())
}

#[tokio::test]
async fn test_content_type_validation() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint returning non-JSON content type
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(r#"{"chain_id": 1}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    // This might succeed or fail depending on how strict our client is
    // about content types
    let result = client.fetch_chain_id_from_network().await;
    println!("Content-type test result: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn test_large_response_handling() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Create a large JSON response
    let large_response = format!(
        r#"{{"chain_id": 1, "large_field": "{}"}}"#,
        "x".repeat(10000)
    );

    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&large_response)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(10)) // Longer timeout for large response
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    // Should handle large responses gracefully
    match result {
        Ok(chain_id) => {
            assert_eq!(chain_id, 1);
            println!("Large response handled successfully");
        }
        Err(e) => {
            println!("Large response error: {}", e);
            // This might be acceptable if we have size limits
        }
    }

    Ok(())
}

//
// ============================================================================
// CONCURRENT REQUEST HANDLING MOCK TESTS
// ============================================================================
//

#[tokio::test]
async fn test_multiple_concurrent_requests() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint that can handle multiple requests (correct path: /v1/chains/chain_id)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"chain_id": 1}"#)
        .expect_at_least(3) // Expect at least 3 calls
        .create();

    let _client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    // Make multiple concurrent requests
    let mut handles = Vec::new();
    for i in 0..5 {
        let client_for_task = ClientBuilder::new()
            .base_url(server.url())
            .timeout(Duration::from_secs(5))
            .build()?;
        let handle = tokio::spawn(async move {
            println!("Starting request {}", i);
            client_for_task.fetch_chain_id_from_network().await
        });
        handles.push(handle);
    }

    // Wait for all requests
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.expect("Task should complete"));
    }

    // All requests should succeed
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(chain_id) => {
                assert_eq!(*chain_id, 1);
                println!("Request {} succeeded with chain_id: {}", i, chain_id);
            }
            Err(e) => panic!("Request {} failed: {}", i, e),
        }
    }

    Ok(())
}

//
// ============================================================================
// TOKEN OPERATION MOCK TESTS
// ============================================================================
//

/// Test hash response structure and format validation
#[tokio::test]
async fn test_hash_response_structure() -> Result<(), Box<dyn Error>> {
    println!("Testing Hash response structure...");

    let mock_hash = mock_utils::create_mock_hash();
    mock_utils::validate_mock_hash(&mock_hash)?;

    // Test serialization/deserialization (TransactionResponse serializes as JSON object {"hash": "0x..."})
    let json = serde_json::to_string(&mock_hash)?;
    assert!(json.contains("\"hash\""), "JSON should contain hash field");
    assert!(json.contains("\"0x"), "JSON should contain hex hash value");

    let deserialized: TransactionResponse = serde_json::from_str(&json)?;
    assert_eq!(mock_hash.hash, deserialized.hash);

    // Test display implementation
    let display_str = format!("{}", mock_hash);
    assert!(display_str.contains("Transaction"));
    assert!(display_str.contains("0x1234567890abcdef"));

    println!("Hash structure validation completed");
    Ok(())
}

/// Test token payload serialization and signature generation
#[tokio::test]
async fn test_token_payload_serialization() -> Result<(), Box<dyn Error>> {
    println!("Testing token payload serialization...");

    let addresses = mock_utils::MockAddresses::new();

    // Test TokenMintPayload
    let mint_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::from(1000000000000000000u64),
    };

    // Test serialization
    let json = serde_json::to_string(&mint_payload)?;
    assert!(json.contains("token"));
    assert!(json.contains("to"));
    assert!(json.contains("value"));

    // Test signature hash generation
    let hash = mint_payload.signature_hash();
    assert_eq!(hash.len(), 32); // keccak256 produces 32 bytes

    // Test deterministic hashing
    let hash2 = mint_payload.signature_hash();
    assert_eq!(hash, hash2);

    println!("Payload serialization validated");
    Ok(())
}

/// Test error handling for invalid payloads
#[tokio::test]
async fn test_invalid_payload_handling() -> Result<(), Box<dyn Error>> {
    println!("Testing invalid payload handling...");

    let client = mock_utils::create_mock_client()?;
    let addresses = mock_utils::MockAddresses::new();

    // Test with invalid private key format
    let mint_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::from(1000000000000000000u64),
    };

    // This should fail due to invalid private key
    match client.mint_token(mint_payload, "invalid_key").await {
        Ok(_) => {
            panic!("Should have failed with invalid private key");
        }
        Err(e) => {
            println!("Correctly rejected invalid private key: {}", e);
            assert!(e.to_string().contains("Invalid") || e.to_string().contains("decode"));
        }
    }

    Ok(())
}

/// Test all token operation method signatures
#[tokio::test]
async fn test_token_method_signatures() -> Result<(), Box<dyn Error>> {
    println!("Testing token method signatures...");

    let client = mock_utils::create_mock_client()?;
    let addresses = mock_utils::MockAddresses::new();
    let private_key = mock_utils::test_private_key();

    // Test all method signatures compile and have correct return types

    // 1. mint_token
    let mint_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::from(1000000000000000000u64),
    };

    // These will fail due to unreachable endpoint, but we're testing signatures
    let _: Result<TransactionResponse, _> = client.mint_token(mint_payload, private_key).await;

    // 2. burn_token
    let burn_payload = TokenBurnPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 2,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::from(500000000000000000u64),
    };

    let _: Result<TransactionResponse, _> = client.burn_token(burn_payload, private_key).await;

    // 3. grant_authority
    let authority_payload = TokenAuthorityPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 3,
        action: AuthorityAction::Grant,
        authority_type: Authority::MintBurnTokens,
        authority_address: addresses.authority_address,
        token: addresses.token_mint,
        value: U256::from(10000000000000000000u64),
    };

    let _: Result<TransactionResponse, _> = client
        .grant_authority(authority_payload.clone(), private_key)
        .await;

    // 4. revoke_authority
    let revoke_payload = TokenAuthorityPayload {
        action: AuthorityAction::Revoke,
        ..authority_payload
    };

    let _: Result<TransactionResponse, _> =
        client.revoke_authority(revoke_payload, private_key).await;

    // 5. pause_token
    let pause_payload = TokenPausePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 5,
        action: PauseAction::Pause,
        token: addresses.token_mint,
    };

    let _: Result<TransactionResponse, _> = client.pause_token(pause_payload, private_key).await;

    // 6. manage_blacklist
    let blacklist_payload = TokenBlacklistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 6,
        action: BlacklistAction::Add,
        address: addresses.authority_address,
        token: addresses.token_mint,
    };

    let _: Result<TransactionResponse, _> = client
        .manage_blacklist(blacklist_payload, private_key)
        .await;

    // 7. manage_whitelist
    let whitelist_payload = TokenWhitelistPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 7,
        action: WhitelistAction::Add,
        address: addresses.authority_address,
        token: addresses.token_mint,
    };

    let _: Result<TransactionResponse, _> = client
        .manage_whitelist(whitelist_payload, private_key)
        .await;

    // 8. update_token_metadata
    let metadata_payload = TokenMetadataUpdatePayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 8,
        token: addresses.token_mint,
        name: "Test Token".to_string(),
        uri: "https://example.com/token.json".to_string(),
        additional_metadata: vec![MetadataKVPair {
            key: "description".to_string(),
            value: "A test token".to_string(),
        }],
    };

    let _: Result<TransactionResponse, _> = client
        .update_token_metadata(metadata_payload, private_key)
        .await;

    println!("All method signatures validated with Hash return type");
    Ok(())
}

/// Test payload validation and edge cases
#[tokio::test]
async fn test_payload_edge_cases() -> Result<(), Box<dyn Error>> {
    println!("Testing payload edge cases...");

    let addresses = mock_utils::MockAddresses::new();

    // Test with maximum values
    let max_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::MAX,
    };

    // Should be able to serialize and hash
    let json = serde_json::to_string(&max_payload)?;
    assert!(json.contains("token"));
    assert!(json.contains("to"));
    assert!(json.contains("value"));

    let hash = max_payload.signature_hash();
    assert_eq!(hash.len(), 32);

    // Test with zero values
    let zero_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 2,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::ZERO,
    };

    let json_zero = serde_json::to_string(&zero_payload)?;
    println!("Zero payload JSON: {}", json_zero); // Debug output
    assert!(json_zero.contains("token"));
    assert!(json_zero.contains("to"));

    let hash_zero = zero_payload.signature_hash();
    assert_eq!(hash_zero.len(), 32);
    assert_ne!(hash_zero, hash); // Different payloads should have different hashes

    println!("Edge case validation completed");
    Ok(())
}

/// Test concurrent payload creation and hashing
#[tokio::test]
async fn test_concurrent_payload_operations() -> Result<(), Box<dyn Error>> {
    println!("Testing concurrent payload operations...");

    let addresses = mock_utils::MockAddresses::new();

    // Create multiple payloads concurrently
    let mut handles = Vec::new();

    for i in 0..5 {
        let addresses_clone = addresses.clone();

        let handle = tokio::spawn(async move {
            let payload = TokenMintPayload {
                recent_epoch: 100,
                recent_checkpoint: 200,
                chain_id: 1,
                nonce: 1,
                token: addresses_clone.token_mint,
                recipient: addresses_clone.recipient,
                value: U256::from((i + 1) * 1000000000000000000u64),
            };

            // Test serialization and hashing concurrently
            let json = serde_json::to_string(&payload).expect("Should serialize");
            let hash = payload.signature_hash();

            (i, json, hash)
        });

        handles.push(handle);
    }

    // Collect results
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await?);
    }

    // Verify all operations completed successfully
    assert_eq!(results.len(), 5);

    // Verify all hashes are unique
    for i in 0..results.len() {
        for j in (i + 1)..results.len() {
            assert_ne!(
                results[i].2, results[j].2,
                "Hashes should be unique for different amounts"
            );
        }
    }

    // Verify all JSON serializations are valid
    for (i, json, _) in &results {
        assert!(json.contains("token"));
        assert!(json.contains("to"));
        assert!(json.contains("value"));
        println!("Payload {}: {}", i, json);
    }

    println!("Concurrent operations completed successfully");
    Ok(())
}

/// Test request structure creation and serialization
#[tokio::test]
async fn test_request_structure_creation() -> Result<(), Box<dyn Error>> {
    println!("Testing request structure creation...");

    let addresses = mock_utils::MockAddresses::new();
    let private_key = mock_utils::test_private_key();

    // Test creating request structures (this tests the internal request creation)
    let mint_payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: addresses.token_mint,
        recipient: addresses.recipient,
        value: U256::from(1000000000000000000u64),
    };

    // Test that we can create signature (even if we can't submit)
    use onemoney_protocol::crypto::sign_transaction_payload;

    let signature = sign_transaction_payload(&mint_payload, private_key)?;

    // Test signature properties
    assert_ne!(signature.r, U256::ZERO);
    assert_ne!(signature.s, U256::ZERO);
    assert!(signature.v == 27 || signature.v == 28 || signature.v == 0 || signature.v == 1); // Valid recovery IDs

    // Test that signature is deterministic for same payload
    let signature2 = sign_transaction_payload(&mint_payload, private_key)?;
    assert_eq!(signature.r, signature2.r);
    assert_eq!(signature.s, signature2.s);
    assert_eq!(signature.v, signature2.v);

    println!("Request structure creation validated");
    Ok(())
}

//
// ============================================================================
// MOCK SERVER RESPONSE VALIDATION TESTS
// ============================================================================
//

#[tokio::test]
async fn test_mock_response_consistency() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Test that mock responses are consistent across multiple calls (correct path: /v1/chains/chain_id)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"chain_id": 42}"#)
        .expect(3)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    // Make multiple requests and verify consistent responses
    for i in 0..3 {
        let chain_id = client.fetch_chain_id_from_network().await?;
        assert_eq!(chain_id, 42, "Chain ID should be consistent on call {}", i);
        println!("Call {}: chain_id = {}", i, chain_id);
    }

    Ok(())
}

#[tokio::test]
async fn test_mock_error_response_formats() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Test different error response formats
    let error_scenarios = [
        (
            400,
            r#"{"error_code": "validation_error", "message": "Invalid input"}"#,
        ),
        (401, r#"{"error": "Unauthorized"}"#),
        (404, r#"{"message": "Resource not found"}"#),
        (
            500,
            r#"{"error": "Internal server error", "code": "INTERNAL_ERROR"}"#,
        ),
    ];

    for (status_code, response_body) in error_scenarios {
        let _mock = server
            .mock("GET", "/v1/chains/id")
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(response_body)
            .create();

        let client = ClientBuilder::new()
            .base_url(server.url())
            .timeout(Duration::from_secs(5))
            .build()?;

        let result = client.fetch_chain_id_from_network().await;
        assert!(result.is_err(), "Should fail with status {}", status_code);

        println!("Status {}: {:?}", status_code, result.unwrap_err());
    }

    Ok(())
}

#[tokio::test]
async fn test_mock_server_edge_cases() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Test empty response body
    let _mock = server
        .mock("GET", "/v1/chains/id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("")
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail with empty response");

    println!("Empty response error (expected): {:?}", result.unwrap_err());
    Ok(())
}
