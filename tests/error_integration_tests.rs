//! Comprehensive error handling integration tests
//!
//! This file contains all error handling related integration tests including:
//! - Error type creation and validation
//! - Error display and debug formatting
//! - Error propagation and conversion
//! - HTTP error response handling
//! - Cross-error-type compatibility
//! - Error serialization and deserialization

use alloy_primitives::Address;
use onemoney_protocol::client::builder::ClientBuilder;
use onemoney_protocol::{Network, error::*};
use std::array::TryFromSliceError;
use std::error::Error as StdError;
use std::str::FromStr;
use std::time::Duration;

//
// ============================================================================
// BASIC ERROR CREATION AND DISPLAY TESTS
// ============================================================================
//

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

//
// ============================================================================
// COMPREHENSIVE ERROR TYPE CREATION TESTS
// ============================================================================
//

#[test]
fn test_error_api_creation() {
    let error = Error::api(
        404,
        "NOT_FOUND".to_string(),
        "Resource not found".to_string(),
    );
    assert!(error.is_api_error());
    assert_eq!(error.status_code(), Some(404));
    assert_eq!(error.error_code(), Some("NOT_FOUND"));
    assert!(format!("{}", error).contains("404"));
    assert!(format!("{}", error).contains("NOT_FOUND"));
    assert!(format!("{}", error).contains("Resource not found"));
}

#[test]
fn test_error_address_creation() {
    let error = Error::address("Invalid checksum");
    assert!(format!("{}", error).contains("Invalid checksum"));
    assert!(!error.is_api_error());
    assert!(!error.is_config_error());
    assert!(!error.is_crypto_error());
}

#[test]
fn test_error_array_conversion() {
    let error = Error::array_conversion(32, 16);
    assert!(format!("{}", error).contains("expected length 32"));
    assert!(format!("{}", error).contains("got 16"));
}

#[test]
fn test_error_validation_creation() {
    let error = Error::validation("amount", "must be positive");
    assert!(format!("{}", error).contains("amount"));
    assert!(format!("{}", error).contains("must be positive"));
}

#[test]
fn test_error_custom_creation() {
    let error = Error::custom("Something went wrong");
    assert!(format!("{}", error).contains("Something went wrong"));
}

#[test]
fn test_error_http_transport() {
    let error = Error::http_transport("Connection refused", Some(500));
    assert!(format!("{}", error).contains("Connection refused"));
}

#[test]
fn test_error_request_timeout() {
    let error = Error::request_timeout("/api/test", 5000);
    assert!(format!("{}", error).contains("/api/test"));
    assert!(format!("{}", error).contains("5000ms"));
}

#[test]
fn test_error_connection() {
    let error = Error::connection("Network unreachable");
    assert!(format!("{}", error).contains("Network unreachable"));
}

#[test]
fn test_error_dns_resolution() {
    let error = Error::dns_resolution("Host not found");
    assert!(format!("{}", error).contains("Host not found"));
}

#[test]
fn test_error_response_deserialization() {
    let error = Error::response_deserialization("JSON", "Invalid syntax", "{broken");
    assert!(format!("{}", error).contains("JSON"));
    assert!(format!("{}", error).contains("Invalid syntax"));
    assert!(format!("{}", error).contains("{broken"));
}

#[test]
fn test_error_authentication() {
    let error = Error::authentication("Invalid credentials");
    assert!(format!("{}", error).contains("Invalid credentials"));
}

#[test]
fn test_error_authorization() {
    let error = Error::authorization("Insufficient permissions");
    assert!(format!("{}", error).contains("Insufficient permissions"));
}

#[test]
fn test_error_rate_limit_exceeded() {
    let error = Error::rate_limit_exceeded(Some(60));
    assert!(format!("{}", error).contains("Rate limit exceeded"));

    let error2 = Error::rate_limit_exceeded(None);
    assert!(format!("{}", error2).contains("Rate limit exceeded"));
}

#[test]
fn test_error_invalid_parameter() {
    let error = Error::invalid_parameter("timeout", "must be positive");
    assert!(format!("{}", error).contains("timeout"));
    assert!(format!("{}", error).contains("must be positive"));
}

#[test]
fn test_error_resource_not_found() {
    let error = Error::resource_not_found("transaction", "0x123abc");
    assert!(format!("{}", error).contains("transaction"));
    assert!(format!("{}", error).contains("0x123abc"));
}

#[test]
fn test_error_business_logic() {
    let error = Error::business_logic("transfer", "insufficient balance");
    assert!(format!("{}", error).contains("transfer"));
    assert!(format!("{}", error).contains("insufficient balance"));
}

//
// ============================================================================
// CRYPTO ERROR TESTS
// ============================================================================
//

#[test]
fn test_crypto_error_creation() {
    let error = CryptoError::invalid_private_key("Wrong length");
    assert!(format!("{}", error).contains("Wrong length"));

    let error = CryptoError::invalid_public_key("Invalid format");
    assert!(format!("{}", error).contains("Invalid format"));

    let error = CryptoError::signature_failed("Key not found");
    assert!(format!("{}", error).contains("Key not found"));

    let error = CryptoError::verification_failed("Signature mismatch");
    assert!(format!("{}", error).contains("Signature mismatch"));

    let error = CryptoError::key_derivation("Derivation failed");
    assert!(format!("{}", error).contains("Derivation failed"));
}

#[test]
fn test_error_from_crypto_error() {
    let crypto_error = CryptoError::invalid_private_key("test");
    let error: Error = crypto_error.into();
    assert!(error.is_crypto_error());
    assert!(format!("{}", error).contains("test"));
}

//
// ============================================================================
// CONFIG ERROR TESTS
// ============================================================================
//

#[test]
fn test_config_error_creation() {
    let error = ConfigError::invalid_timeout("negative value");
    assert!(format!("{}", error).contains("negative value"));

    let error = ConfigError::invalid_network("unknown network");
    assert!(format!("{}", error).contains("unknown network"));

    let error = ConfigError::missing_config("API key required");
    assert!(format!("{}", error).contains("API key required"));

    let error = ConfigError::client_builder("TLS error");
    assert!(format!("{}", error).contains("TLS error"));
}

#[test]
fn test_error_from_config_error() {
    let config_error = ConfigError::invalid_timeout("test");
    let error: Error = config_error.into();
    assert!(error.is_config_error());
    assert!(format!("{}", error).contains("test"));
}

//
// ============================================================================
// ERROR CONVERSION TESTS
// ============================================================================
//

#[test]
fn test_error_from_conversions() {
    // Test JSON error conversion
    let json_err = serde_json::from_str::<serde_json::Value>("{invalid").unwrap_err();
    let error: Error = json_err.into();
    assert!(format!("{}", error).contains("JSON parsing failed"));

    // Test URL parsing error conversion
    let url_err = url::Url::parse("not-a-url").unwrap_err();
    let error: Error = url_err.into();
    assert!(format!("{}", error).contains("Invalid URL"));

    // Test hex decoding error conversion
    let hex_err = hex::decode("not-hex").unwrap_err();
    let error: Error = hex_err.into();
    assert!(format!("{}", error).contains("Hex decoding failed"));
}

#[test]
fn test_error_from_try_from_slice_error() {
    // Create a TryFromSliceError by trying to convert wrong sized slice
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let result: std::result::Result<[u8; 32], TryFromSliceError> = data.as_slice().try_into();
    let slice_error = result.unwrap_err();

    let error: Error = slice_error.into();
    assert!(format!("{}", error).contains("Array conversion failed"));
    assert!(format!("{}", error).contains("expected length 32"));
}

//
// ============================================================================
// ADDRESS PARSING ERROR TESTS
// ============================================================================
//

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
        let result = Address::from_str(invalid_addr);
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

//
// ============================================================================
// ERROR CATEGORIES AND TYPE CHECKS
// ============================================================================
//

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
fn test_error_type_checks() {
    let api_error = Error::api(
        404,
        "NOT_FOUND".to_string(),
        "Resource not found".to_string(),
    );
    assert!(api_error.is_api_error());
    assert!(!api_error.is_config_error());
    assert!(!api_error.is_crypto_error());

    let crypto_error: Error = CryptoError::invalid_private_key("test").into();
    assert!(!crypto_error.is_api_error());
    assert!(!crypto_error.is_config_error());
    assert!(crypto_error.is_crypto_error());

    let config_error: Error = ConfigError::invalid_timeout("test").into();
    assert!(!config_error.is_api_error());
    assert!(config_error.is_config_error());
    assert!(!config_error.is_crypto_error());
}

#[test]
fn test_error_status_code_methods() {
    let api_error = Error::api(
        500,
        "SERVER_ERROR".to_string(),
        "Internal error".to_string(),
    );
    assert_eq!(api_error.status_code(), Some(500));
    assert_eq!(api_error.error_code(), Some("SERVER_ERROR"));

    let custom_error = Error::custom("Not an API error");
    assert_eq!(custom_error.status_code(), None);
    assert_eq!(custom_error.error_code(), None);
}

//
// ============================================================================
// ERROR MESSAGE VALIDATION TESTS
// ============================================================================
//

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

//
// ============================================================================
// ERROR SERIALIZATION TESTS
// ============================================================================
//

#[test]
fn test_error_response_serialization() {
    let error_response = ErrorResponse {
        error_code: "INVALID_INPUT".to_string(),
        message: "The provided input is invalid".to_string(),
    };

    let json = serde_json::to_string(&error_response).expect("Test data should be valid");
    let deserialized: ErrorResponse =
        serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(error_response.error_code, deserialized.error_code);
    assert_eq!(error_response.message, deserialized.message);
}

//
// ============================================================================
// HETEROGENEOUS ERROR TYPE TESTS
// ============================================================================
//

#[test]
fn test_error_constructors_heterogeneous_types() {
    // Test response_deserialization with mixed &str and String types
    let format_str = "JSON";
    let error_string = String::from("Parse error occurred");
    let response_data = "{\"invalid\": syntax}";

    let error = Error::response_deserialization(format_str, error_string, response_data);
    let error_display = format!("{}", error);
    assert!(error_display.contains("JSON"));
    assert!(error_display.contains("Parse error occurred"));
    assert!(error_display.contains("{\"invalid\": syntax}"));

    // Test invalid_parameter with mixed types
    let param_name = "timeout";
    let message = format!("Value must be between {} and {}", 1, 3600);

    let error = Error::invalid_parameter(param_name, message);
    let error_display = format!("{}", error);
    assert!(error_display.contains("timeout"));
    assert!(error_display.contains("Value must be between 1 and 3600"));

    // Test resource_not_found with mixed types
    let resource_type = String::from("Transaction");
    let identifier = "0x1234abcd";

    let error = Error::resource_not_found(resource_type, identifier);
    let error_display = format!("{}", error);
    assert!(error_display.contains("Transaction"));
    assert!(error_display.contains("0x1234abcd"));

    // Test business_logic with mixed types
    let operation = "token_transfer";
    let reason = format!("Insufficient balance: {} < {}", 100, 500);

    let error = Error::business_logic(operation, reason);
    let error_display = format!("{}", error);
    assert!(error_display.contains("token_transfer"));
    assert!(error_display.contains("Insufficient balance: 100 < 500"));
}

//
// ============================================================================
// ERROR TYPES THREAD SAFETY TESTS
// ============================================================================
//

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
    .expect("Test data should be valid");
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

//
// ============================================================================
// HTTP ERROR INTEGRATION TESTS
// ============================================================================
//

#[tokio::test]
async fn test_api_error_responses() {
    // Test how we handle HTTP error responses
    let client = ClientBuilder::new()
        .network(Network::Custom("http://httpbin.org/status/404".into())) // Returns 404
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Client should build");

    let result = client.fetch_chain_id_from_network().await;
    assert!(result.is_err(), "Should fail");
}

//
// ============================================================================
// REQWEST ERROR INTEGRATION TESTS
// ============================================================================
//

#[cfg(test)]
mod reqwest_error_tests {
    use super::*;

    #[test]
    fn test_reqwest_error_conversion_with_mock() {
        // We can't easily create reqwest::Error instances in tests,
        // but we can test the conversion logic by creating mock HTTP errors

        // Test timeout scenario - create a timeout error via reqwest
        let client = reqwest::Client::new();
        let response = tokio_test::block_on(async {
            client
                .get("http://httpbin.org/delay/10")
                .timeout(std::time::Duration::from_millis(1))
                .send()
                .await
        });

        if let Err(reqwest_error) = response {
            let error: Error = reqwest_error.into();
            // Should be converted to request timeout error
            let error_str = format!("{}", error);
            assert!(error_str.contains("timeout") || error_str.contains("Request"));
        }
    }
}

//
// ============================================================================
// ERROR INTEGRATION CHAIN TESTS
// ============================================================================
//

#[test]
fn test_error_chain_integration() {
    // Test a chain of error conversions and propagations
    fn level_3() -> Result<String> {
        Err(Error::validation("input", "invalid format"))
    }

    fn level_2() -> Result<String> {
        level_3().map_err(|e| Error::custom(format!("Level 2 processing failed: {}", e)))
    }

    fn level_1() -> Result<String> {
        level_2().map_err(|e| Error::custom(format!("Level 1 operation failed: {}", e)))
    }

    let result = level_1();
    assert!(result.is_err());

    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(error_message.contains("Level 1 operation failed"));
            assert!(error_message.contains("Level 2 processing failed"));
            assert!(error_message.contains("invalid format"));
            println!("Chained error message: {}", error_message);
        }
        Ok(_) => panic!("Expected chained error"),
    }
}

#[test]
fn test_error_context_preservation() {
    // Test that error contexts are preserved through conversions
    let original_message = "original context information";
    let validation_error = Error::validation("field", original_message);

    // Convert through different error types
    let custom_error = Error::custom(format!("Wrapper: {}", validation_error));
    let final_error = Error::custom(format!("Final: {}", custom_error));

    let final_message = format!("{}", final_error);
    assert!(final_message.contains("original context information"));
    assert!(final_message.contains("Wrapper"));
    assert!(final_message.contains("Final"));

    println!("Context preserved: {}", final_message);
}

#[test]
fn test_cross_error_type_compatibility() {
    // Test that different error types can be used together
    let errors: Vec<Error> = vec![
        Error::api(
            404,
            "NOT_FOUND".to_string(),
            "Resource not found".to_string(),
        ),
        CryptoError::invalid_private_key("Invalid key").into(),
        ConfigError::invalid_timeout("Negative timeout").into(),
        Error::validation("amount", "must be positive"),
        Error::custom("Generic error"),
    ];

    for (i, error) in errors.iter().enumerate() {
        let display_str = format!("{}", error);
        let debug_str = format!("{:?}", error);

        assert!(
            !display_str.is_empty(),
            "Error {} display should not be empty",
            i
        );
        assert!(
            !debug_str.is_empty(),
            "Error {} debug should not be empty",
            i
        );

        // Test that all errors implement std::error::Error
        let _source = error.source();

        println!("Error {}: {}", i, display_str);
    }

    // Test that errors can be collected and processed uniformly
    let error_messages: Vec<String> = errors.iter().map(|e| format!("{}", e)).collect();
    assert_eq!(error_messages.len(), 5);

    for message in error_messages {
        assert!(!message.is_empty());
        assert!(message.len() > 5); // Should be descriptive
    }
}
