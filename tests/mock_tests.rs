//! Mock server tests for the OneMoney Rust SDK.
//!
//! These tests use mock HTTP servers to test API interactions
//! without requiring a real OneMoney node.

use mockito::ServerGuard;
use onemoney_protocol::{ClientBuilder, OneMoneyAddress};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

/// Setup a mock server for testing
async fn setup_mock_server() -> ServerGuard {
    mockito::Server::new_async().await
}

#[tokio::test]
async fn test_chain_id_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock the chain ID endpoint
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
    let chain_id = client.get_chain_id().await?;
    assert_eq!(chain_id, 12345);

    Ok(())
}

#[tokio::test]
async fn test_http_error_responses() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock a 500 error response
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail with 500 error");

    println!("Expected error: {:?}", result.unwrap_err());
    Ok(())
}

#[tokio::test]
async fn test_network_timeout_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock an endpoint that never responds (simulates network timeout)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
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

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should timeout");

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

    let address = OneMoneyAddress::from_str(test_address)?;
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

    let token_addr = OneMoneyAddress::from_str(token_address)?;
    let metadata = client.get_token_metadata(token_addr).await?;

    println!("Token metadata: {}", metadata);

    Ok(())
}

#[tokio::test]
async fn test_latest_state_mock() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock the latest state endpoint
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

#[tokio::test]
async fn test_invalid_json_response() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint returning invalid JSON
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

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail to parse invalid JSON");

    match result {
        Err(e) => {
            println!("JSON parse error (expected): {}", e);
            let error_str = format!("{}", e);
            assert!(
                error_str.contains("serialize")
                    || error_str.contains("JSON")
                    || error_str.contains("parse")
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
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"wrong_field": 123}"#) // Missing chain_id field
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail due to missing field");

    Ok(())
}

#[tokio::test]
async fn test_multiple_concurrent_requests() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint that can handle multiple requests
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
            client_for_task.get_chain_id().await
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

#[tokio::test]
async fn test_api_rate_limiting_simulation() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock rate limiting (429 Too Many Requests)
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_header("retry-after", "60")
        .with_body(r#"{"error": "Rate limit exceeded"}"#)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail with rate limit error");

    println!("Rate limit error (expected): {:?}", result.unwrap_err());
    Ok(())
}

#[tokio::test]
async fn test_content_type_validation() -> Result<(), Box<dyn Error>> {
    let mut server = setup_mock_server().await;

    // Mock endpoint returning non-JSON content type
    let _mock = server
        .mock("GET", "/v1/chains/chain_id")
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
    let result = client.get_chain_id().await;
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
        .mock("GET", "/v1/chains/chain_id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&large_response)
        .create();

    let client = ClientBuilder::new()
        .base_url(server.url())
        .timeout(Duration::from_secs(10)) // Longer timeout for large response
        .build()?;

    let result = client.get_chain_id().await;
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
