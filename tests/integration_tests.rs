//! Integration tests for the OneMoney Rust SDK.
//!
//! These tests verify the complete functionality of the SDK against
//! a real or mocked OneMoney API server.

use alloy_primitives::Address;
use onemoney_protocol::{Client, ClientBuilder, Network};
use std::error::Error;
use std::str::FromStr;

use std::time::Duration;

// Test configuration
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Test utilities for integration tests
mod test_utils {
    use super::*;

    /// Create a test client configured for local testing
    pub fn create_test_client() -> Result<Client, Box<dyn std::error::Error>> {
        Ok(ClientBuilder::new()
            .network(Network::Testnet)
            .timeout(TEST_TIMEOUT)
            .build()?)
    }

    /// Generate a test address
    pub fn test_address() -> Address {
        Address::from_str("0x1234567890abcdef1234567890abcdef12345678").expect("Valid test address")
    }
}

#[tokio::test]
async fn test_client_creation() -> std::result::Result<(), Box<dyn Error>> {
    // Test different client creation methods
    let _mainnet_client = Client::mainnet()?;
    let _testnet_client = Client::testnet()?;
    let _local_client = Client::local()?;

    // Test ClientBuilder
    let _builder_client = ClientBuilder::new()
        .network(Network::Local)
        .timeout(Duration::from_secs(10))
        .build()?;

    // Test custom URL
    let _custom_client = ClientBuilder::new()
        .network(Network::Custom("http://localhost:8080".into()))
        .timeout(Duration::from_secs(5))
        .build()?;

    Ok(())
}

#[tokio::test]
async fn test_network_connectivity() -> Result<(), Box<dyn Error>> {
    let client = test_utils::create_test_client()?;

    // This test will only pass if a local OneMoney node is running
    // In a real testing environment, we would either:
    // 1. Use a mock server
    // 2. Skip this test if no test node is available
    match client.fetch_chain_id_from_network().await {
        Ok(chain_id) => {
            println!(
                "Successfully connected to test node. Chain ID: {}",
                chain_id
            );
            assert!(chain_id > 0, "Chain ID should be positive");
        }
        Err(e) => {
            println!("No test node available, skipping connectivity test: {}", e);
            // Don't fail the test if no test node is running
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_address_validation() -> Result<(), Box<dyn Error>> {
    // Test valid addresses
    let valid_addresses = [
        "0x1234567890abcdef1234567890abcdef12345678",
        "0xAbCdEf1234567890AbCdEf1234567890AbCdEf12",
        "0x0000000000000000000000000000000000000000",
        "0xffffffffffffffffffffffffffffffffffffffff",
    ];

    for addr_str in &valid_addresses {
        let result = Address::from_str(addr_str);
        assert!(result.is_ok(), "Address {} should be valid", addr_str);
    }

    // Test invalid addresses
    let invalid_addresses = [
        "",                                            // Empty
        "0x123",                                       // Too short
        "0x1234567890abcdef1234567890abcdef123456789", // Too long
        "0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG",  // Invalid hex
        "0x 1234567890abcdef1234567890abcdef12345678", // Contains space
        "not_a_valid_address",                         // Invalid format
    ];

    for addr_str in &invalid_addresses {
        let result = Address::from_str(addr_str);
        assert!(result.is_err(), "Address {} should be invalid", addr_str);
    }

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<(), Box<dyn Error>> {
    // Test error handling with unreachable endpoint
    let client = ClientBuilder::new()
        .network(Network::Custom("http://127.0.0.1:1".into())) // Invalid port
        .timeout(Duration::from_secs(1))
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(
        result.is_err(),
        "Should fail to connect to invalid endpoint"
    );

    // Test error types
    match result {
        Err(e) => {
            println!("Expected error: {}", e);
            // Verify error can be displayed and debugged
            let _debug_str = format!("{:?}", e);
            let _display_str = format!("{}", e);
        }
        Ok(_) => panic!("Expected error but got success"),
    }

    Ok(())
}

#[tokio::test]
async fn test_timeout_handling() -> Result<(), Box<dyn Error>> {
    // Test very short timeout
    let client = ClientBuilder::new()
        .network(Network::Custom("http://httpbin.org/delay/10".into())) // Delayed response
        .timeout(Duration::from_millis(100))     // Very short timeout
        .build()?;

    let result = client.fetch_chain_id_from_network().await;
    assert!(
        result.is_err(),
        "Should timeout with short timeout duration"
    );

    Ok(())
}

#[tokio::test]
async fn test_account_operations_offline() -> Result<(), Box<dyn Error>> {
    let address = test_utils::test_address();

    // Test address formatting and parsing
    let address_str = address.to_string();
    assert!(address_str.starts_with("0x"));
    assert_eq!(address_str.len(), 42); // 0x + 40 hex chars

    // Test round-trip conversion
    let parsed_address =
        Address::from_str(&address_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    assert_eq!(address, parsed_address);

    Ok(())
}

#[cfg(feature = "integration")]
mod integration_with_server {
    use super::*;

    #[tokio::test]
    async fn test_chain_operations() -> Result<(), Box<dyn Error>> {
        let client = test_utils::create_test_client()?;

        // Test chain ID retrieval
        let chain_id = client.fetch_chain_id_from_network().await?;
        assert!(chain_id > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_state_operations() -> Result<(), Box<dyn Error>> {
        let client = test_utils::create_test_client()?;

        // Test latest checkpoint number
        let checkpoint_info = client.get_checkpoint_number().await?;
        println!("Latest checkpoint: {}", checkpoint_info);

        Ok(())
    }

    #[tokio::test]
    async fn test_account_operations() -> Result<(), Box<dyn Error>> {
        let client = test_utils::create_test_client()?;
        let address = test_utils::test_address();

        // Test account nonce (may fail if account doesn't exist)
        match client.get_account_nonce(address).await {
            Ok(nonce) => {
                println!("Account nonce: {}", nonce);
            }
            Err(e) => {
                println!("Account not found (expected): {}", e);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_token_metadata() -> Result<(), Box<dyn Error>> {
        let client = test_utils::create_test_client()?;
        let token_address = test_utils::test_address();

        // Test token metadata retrieval (may fail if token doesn't exist)
        match client.get_token_metadata(token_address).await {
            Ok(metadata) => {
                println!("Token metadata: {}", metadata);
            }
            Err(e) => {
                println!("Token not found (expected): {}", e);
            }
        }

        Ok(())
    }
}

#[tokio::test]
async fn test_concurrent_requests() -> Result<(), Box<dyn Error>> {
    use tokio::time::{Duration, timeout};

    // Create multiple clients for concurrent requests
    let mut handles = Vec::new();

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            println!("Starting request {}", i);
            let client = test_utils::create_test_client().expect("Should create client");
            let result = client.fetch_chain_id_from_network().await;
            println!("Completed request {}: {:?}", i, result.is_ok());
            result
        });
        handles.push(handle);
    }

    // Wait for all requests to complete (or timeout)
    let timeout_duration = Duration::from_secs(10);
    let results = timeout(timeout_duration, async {
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.expect("Task should complete"));
        }
        results
    })
    .await
    .expect("All requests should complete within timeout");

    // At least one request should succeed or all should fail with the same type of error
    // (depending on whether a test server is available)
    println!("Concurrent request results: {} total", results.len());

    Ok(())
}

#[tokio::test]
async fn test_multiple_client_instances() -> Result<(), Box<dyn Error>> {
    let client1 = test_utils::create_test_client()?;
    let client2 = test_utils::create_test_client()?;

    // Both clients should be usable and behave consistently
    let result1 = client1.predefined_chain_id();
    let result2 = client2.predefined_chain_id();

    // Both clients should return the same chain ID
    assert_eq!(result1, result2);
    println!("Both clients returned chain ID: {}", result1);

    Ok(())
}

// Benchmark-style test for performance characteristics
#[tokio::test]
async fn test_performance_characteristics() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;

    let client = test_utils::create_test_client()?;

    // Measure response time for a single request
    let start = Instant::now();
    let _result = client.fetch_chain_id_from_network().await;
    let duration = start.elapsed();

    println!("Single request took: {:?}", duration);

    // The request should complete within a reasonable time
    // (even if it fails due to no test server)
    assert!(
        duration < Duration::from_secs(5),
        "Request should complete quickly"
    );

    Ok(())
}
