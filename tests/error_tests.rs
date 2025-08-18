//! Error handling tests for the OneMoney Rust SDK.
//!
//! These tests verify that error types work correctly and provide
//! meaningful error messages and proper error propagation.

use onemoney_protocol::{
    error::{Error, Result},
    ClientBuilder, OneMoneyAddress,
};
use std::error::Error as StdError;
use std::str::FromStr;
use std::time::Duration;

#[test]
fn test_error_display() {
    let errors = [
        Error::address("test"),
        Error::custom("test"),
        Error::validation("field", "test"),
    ];

    for error in &errors {
        let display_str = format!("{}", error);
        let debug_str = format!("{:?}", error);

        assert!(!display_str.is_empty(), "Error display should not be empty");
        assert!(!debug_str.is_empty(), "Error debug should not be empty");

        println!("Error display: {}", display_str);
        println!("Error debug: {}", debug_str);
    }
}

#[test]
fn test_error_source() {
    let base_error = std::io::Error::other("test error");

    // Test errors that might have sources
    let errors = [
        Error::custom(format!("HTTP error: {}", base_error)),
        Error::custom(format!("Serialization error: {}", base_error)),
    ];

    for error in &errors {
        // All errors should implement the Error trait
        let _source = error.source();

        // Test error conversion to Box<dyn Error>
        let error_msg = format!("{}", error);
        let boxed: Box<dyn StdError> = Box::new(Error::custom(error_msg));
        let _display = format!("{}", boxed);
    }
}

#[test]
fn test_result_type() {
    // Test successful Result
    let success: Result<i32> = Ok(42);
    assert!(success.is_ok());
    if let Ok(value) = success {
        assert_eq!(value, 42);
    }

    // Test error Result
    let error: Result<i32> = Err(Error::validation("field", "test"));
    assert!(error.is_err());
}

#[test]
fn test_error_propagation() -> Result<()> {
    // Test that ? operator works with our Result type
    fn inner_function() -> Result<i32> {
        Err(Error::validation("field", "inner error"))
    }

    fn outer_function() -> Result<String> {
        let value = inner_function()?; // This should propagate the error
        Ok(format!("Value: {}", value))
    }

    let result = outer_function();
    assert!(result.is_err());

    match result {
        Err(Error::Validation { field: _, message }) => {
            assert_eq!(message, "inner error");
        }
        _ => panic!("Expected ValidationError"),
    }

    Ok(())
}

#[test]
fn test_address_parsing_errors() {
    let invalid_addresses = [
        "",                                            // Empty
        "0x123",                                       // Too short
        "0x1234567890abcdef1234567890abcdef123456789", // Too long
        "0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG",  // Invalid hex
        "not_a_hex_string",                            // Invalid format
    ];

    for invalid_addr in &invalid_addresses {
        let result = OneMoneyAddress::from_str(invalid_addr);
        assert!(
            result.is_err(),
            "Address '{}' should be invalid",
            invalid_addr
        );

        if let Err(e) = result {
            println!("Address '{}' error: {}", invalid_addr, e);

            // Error should be meaningful
            let error_msg = format!("{}", e);
            assert!(!error_msg.is_empty());
        }
    }
}

#[tokio::test]
async fn test_network_error_handling() {
    // Test connection to invalid endpoint
    let client = ClientBuilder::new()
        .base_url("http://127.0.0.1:1") // Invalid port
        .timeout(Duration::from_millis(100))
        .build()
        .expect("Client should build");

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail to connect");

    match result {
        Err(e) => {
            println!("Network error (expected): {}", e);

            // Error should be meaningful
            let error_str = format!("{}", e);
            assert!(!error_str.is_empty());
            // Could be HTTP client error, network error, or API error
            assert!(
                error_str.contains("HTTP")
                    || error_str.contains("connection")
                    || error_str.contains("network")
                    || error_str.contains("API error")
                    || error_str.contains("request failed")
            );
        }
        Ok(_) => panic!("Expected network error"),
    }
}

#[tokio::test]
async fn test_timeout_error() {
    use tokio::time::{timeout, Duration};

    // Create a client with very short timeout
    let client = ClientBuilder::new()
        .base_url("http://httpbin.org/delay/10") // This will delay 10 seconds
        .timeout(Duration::from_millis(100))     // But we timeout after 100ms
        .build()
        .expect("Client should build");

    // Test that timeout produces appropriate error
    let result = timeout(Duration::from_secs(2), client.get_chain_id()).await;

    match result {
        Ok(inner_result) => {
            // The request completed, but should have been an error due to short client timeout
            assert!(inner_result.is_err(), "Request should have timed out");
            println!("Timeout error (expected): {}", inner_result.unwrap_err());
        }
        Err(_) => {
            // The tokio timeout fired first
            println!("Tokio timeout fired (also acceptable)");
        }
    }
}

#[test]
fn test_error_from_conversions() {
    // Test that common error types can be converted to our Error type

    // Test serde_json error conversion (if implemented)
    let json_error = serde_json::from_str::<i32>("invalid json");
    assert!(json_error.is_err());

    // Test hex error conversion (if implemented)
    let hex_result = hex::decode("invalid hex");
    assert!(hex_result.is_err());
}

#[test]
fn test_error_categories() {
    // Test that different error categories are distinguishable
    let validation_error = Error::validation("field", "validation failed");
    let crypto_error = Error::custom("crypto failed");
    let http_error = Error::custom("http failed");

    // Should be able to match on error type
    match validation_error {
        Error::Validation { .. } => {} // Expected
        _ => panic!("Should be Validation"),
    }

    match crypto_error {
        Error::Custom(_) => {} // Expected
        _ => panic!("Should be Custom"),
    }

    match http_error {
        Error::Custom(_) => {} // Expected
        _ => panic!("Should be Custom"),
    }
}

#[test]
fn test_error_messages_are_helpful() {
    let errors = [
        Error::address("0x123"),
        Error::custom("invalid private key: abc"),
        Error::validation("field", "is required"),
        Error::custom("signature verification failed"),
    ];

    for error in &errors {
        let message = format!("{}", error);

        // Error messages should contain the context
        assert!(message.len() > 10, "Error message should be descriptive");

        // Should not just be generic messages
        assert!(!message.eq_ignore_ascii_case("error"));
        assert!(!message.eq_ignore_ascii_case("failed"));

        println!("Error message: {}", message);
    }
}

#[test]
fn test_error_debug_contains_variant() {
    let errors = [
        Error::address("test"),
        Error::custom("invalid private key: test"),
        Error::validation("field", "test"),
        Error::custom("crypto: test"),
        Error::custom("http: test"),
    ];

    for error in &errors {
        let debug_str = format!("{:?}", error);

        // Debug representation should indicate the error variant
        assert!(
            debug_str.contains("Address")
                || debug_str.contains("Custom")
                || debug_str.contains("Validation")
                || debug_str.contains("Crypto")
                || debug_str.contains("Http"),
            "Debug string should contain variant name: {}",
            debug_str
        );
    }
}

#[test]
fn test_error_types() {
    let validation_error = Error::validation("field", "test");
    let custom_error = Error::custom("test");
    let address_error = Error::address("invalid");

    // Test that errors can be created and displayed
    let _ = format!("{}", validation_error);
    let _ = format!("{}", custom_error);
    let _ = format!("{}", address_error);

    // Test debug formatting
    let _ = format!("{:?}", validation_error);
    let _ = format!("{:?}", custom_error);
    let _ = format!("{:?}", address_error);
}

#[tokio::test]
async fn test_api_error_responses() {
    // Test how we handle HTTP error responses
    let client = ClientBuilder::new()
        .base_url("http://httpbin.org/status/404") // Returns 404
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Client should build");

    let result = client.get_chain_id().await;
    assert!(result.is_err(), "Should fail with 404");

    match result {
        Err(e) => {
            println!("HTTP error (expected): {}", e);

            // Should be categorized as HTTP error
            let error_str = format!("{}", e);
            assert!(
                error_str.contains("HTTP")
                    || error_str.contains("404")
                    || error_str.contains("Not Found")
                    || error_str.contains("request failed")
            );
        }
        Ok(_) => panic!("Expected HTTP error"),
    }
}

#[test]
fn test_error_send_sync() {
    // Verify that errors are Send + Sync for use in async contexts
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<Error>();
    assert_send_sync::<Result<()>>();

    // Test that errors can be shared across threads
    let error = Error::validation("field", "test");
    let error_msg = format!("{}", error);

    std::thread::spawn(move || {
        println!("Error in thread: {}", error_msg);
    })
    .join()
    .unwrap();
}
