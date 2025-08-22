//! Comprehensive HTTP integration tests
//!
//! This file contains all HTTP-related integration tests including:
//! - HTTP client creation and configuration
//! - Error response handling and serialization
//! - HTTP error classification and mapping
//! - Network configuration and client builder functionality
//! - HTTP transport error handling

use onemoney_protocol::client::builder::ClientBuilder;
use onemoney_protocol::client::config::Network;
use onemoney_protocol::client::http::Client;
use onemoney_protocol::error::ErrorResponse;
use onemoney_protocol::{Error, Result};
use std::time::Duration;

//
// ============================================================================
// HTTP CLIENT CREATION AND CONFIGURATION TESTS
// ============================================================================
//

#[test]
fn test_client_creation_methods() -> Result<()> {
    // Test mainnet client creation
    let mainnet_client = Client::mainnet()?;
    let mainnet_debug = format!("{:?}", mainnet_client);
    assert!(mainnet_debug.contains("Client"));
    assert!(mainnet_debug.contains("base_url"));
    assert!(mainnet_debug.contains("hooks_count"));

    // Test testnet client creation
    let testnet_client = Client::testnet()?;
    let testnet_debug = format!("{:?}", testnet_client);
    assert!(testnet_debug.contains("Client"));

    // Test local client creation
    let local_client = Client::local()?;
    let local_debug = format!("{:?}", local_client);
    assert!(local_debug.contains("Client"));

    Ok(())
}

#[test]
fn test_client_builder_creation() {
    // Test ClientBuilder with different networks
    let mainnet_result = ClientBuilder::new().network(Network::Mainnet).build();
    assert!(mainnet_result.is_ok());

    let testnet_result = ClientBuilder::new().network(Network::Testnet).build();
    assert!(testnet_result.is_ok());

    let local_result = ClientBuilder::new().network(Network::Local).build();
    assert!(local_result.is_ok());
}

#[test]
fn test_client_debug_implementation() -> Result<()> {
    let client = Client::mainnet()?;
    let debug_str = format!("{:?}", client);

    // Verify debug output contains expected fields
    assert!(debug_str.contains("Client"));
    assert!(debug_str.contains("base_url"));
    assert!(debug_str.contains("hooks_count"));

    // Test that hooks_count shows 0 for default client
    assert!(debug_str.contains("hooks_count: 0"));

    Ok(())
}

#[test]
fn test_network_client_creation() -> Result<()> {
    let mainnet = Client::mainnet()?;
    let testnet = Client::testnet()?;
    let local = Client::local()?;

    // Test that they're created successfully
    let mainnet_debug = format!("{:?}", mainnet);
    let testnet_debug = format!("{:?}", testnet);
    let local_debug = format!("{:?}", local);

    // All should be Client instances
    assert!(mainnet_debug.contains("Client"));
    assert!(testnet_debug.contains("Client"));
    assert!(local_debug.contains("Client"));

    Ok(())
}

#[test]
fn test_client_builder_timeout() {
    // Test with different timeout values
    let timeouts = [
        Duration::from_secs(10),
        Duration::from_secs(30),
        Duration::from_secs(60),
        Duration::from_millis(5000),
    ];

    for timeout in timeouts {
        let result = ClientBuilder::new()
            .network(Network::Testnet)
            .timeout(timeout)
            .build();
        assert!(result.is_ok());
    }
}

#[test]
fn test_client_builder_invalid_timeout() {
    // Test with zero timeout - should still work but might cause issues in real usage
    let result = ClientBuilder::new()
        .network(Network::Local)
        .timeout(Duration::from_secs(0))
        .build();
    assert!(result.is_ok());
}

#[test]
fn test_network_enum_coverage() {
    // Test all network variants
    let networks = [Network::Mainnet, Network::Testnet, Network::Local];

    for network in networks {
        let result = ClientBuilder::new().network(network).build();
        assert!(result.is_ok());

        let client = result.expect("Test data should be valid");
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("Client"));
    }
}

//
// ============================================================================
// ERROR RESPONSE SERIALIZATION TESTS
// ============================================================================
//

#[test]
fn test_error_response_serialization() {
    let error_response = ErrorResponse {
        error_code: "test_error".to_string(),
        message: "Test error message".to_string(),
    };

    // Test serialization
    let json = serde_json::to_string(&error_response).expect("Test data should be valid");
    assert!(json.contains("test_error"));
    assert!(json.contains("Test error message"));

    // Test deserialization
    let deserialized: ErrorResponse =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(error_response.error_code, deserialized.error_code);
    assert_eq!(error_response.message, deserialized.message);

    // Test clone
    let cloned = error_response.clone();
    assert_eq!(error_response.error_code, cloned.error_code);
    assert_eq!(error_response.message, cloned.message);

    // Test debug
    let debug_str = format!("{:?}", error_response);
    assert!(debug_str.contains("ErrorResponse"));
    assert!(debug_str.contains("test_error"));
}

#[test]
fn test_error_response_various_codes() {
    let error_codes = [
        ("validation_amount", "Amount must be positive"),
        ("invalid_credentials", "Invalid API key"),
        ("insufficient_permissions", "Access denied"),
        ("resource_transaction", "Transaction not found"),
        ("business_transfer", "Insufficient balance"),
        ("rate_limit_exceeded", "Too many requests"),
        ("system_database", "Database connection failed"),
        ("request_timeout", "Request took too long"),
        ("unknown_error", "Something went wrong"),
    ];

    for (error_code, message) in error_codes {
        let error_response = ErrorResponse {
            error_code: error_code.to_string(),
            message: message.to_string(),
        };

        let json = serde_json::to_string(&error_response).expect("Test data should be valid");
        assert!(json.contains(error_code));
        assert!(json.contains(message));

        let deserialized: ErrorResponse =
            serde_json::from_str(&json).expect("Test data should be valid");
        assert_eq!(error_response.error_code, deserialized.error_code);
        assert_eq!(error_response.message, deserialized.message);
    }
}

#[test]
fn test_error_response_edge_cases() {
    // Test with empty strings
    let empty_error = ErrorResponse {
        error_code: String::new(),
        message: String::new(),
    };

    let json = serde_json::to_string(&empty_error).expect("Test data should be valid");
    let deserialized: ErrorResponse =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(empty_error.error_code, deserialized.error_code);
    assert_eq!(empty_error.message, deserialized.message);

    // Test with long strings
    let long_error = ErrorResponse {
        error_code: "a".repeat(1000),
        message: "b".repeat(2000),
    };

    let json2 = serde_json::to_string(&long_error).expect("Test data should be valid");
    let deserialized2: ErrorResponse =
        serde_json::from_str(&json2).expect("Test data should be valid");
    assert_eq!(long_error.error_code, deserialized2.error_code);
    assert_eq!(long_error.message, deserialized2.message);

    // Test with special characters
    let special_error = ErrorResponse {
        error_code: "error_with_numbers_123".to_string(),
        message: "Message with special characters and \"quotes\"".to_string(),
    };

    let json3 = serde_json::to_string(&special_error).expect("Test data should be valid");
    let deserialized3: ErrorResponse =
        serde_json::from_str(&json3).expect("Test data should be valid");
    assert_eq!(special_error.error_code, deserialized3.error_code);
    assert_eq!(special_error.message, deserialized3.message);
}

#[test]
fn test_error_response_json_structure() {
    // Test that the JSON structure matches expected format
    let error_response = ErrorResponse {
        error_code: "test_code".to_string(),
        message: "test message".to_string(),
    };

    let json = serde_json::to_string(&error_response).expect("Test data should be valid");

    // Parse as generic JSON to verify structure
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Test data should be valid");

    assert!(parsed.is_object());
    assert!(parsed.get("error_code").is_some());
    assert!(parsed.get("message").is_some());

    if let Some(error_code) = parsed.get("error_code") {
        assert_eq!(error_code.as_str(), Some("test_code"));
    }

    if let Some(message) = parsed.get("message") {
        assert_eq!(message.as_str(), Some("test message"));
    }
}

//
// ============================================================================
// HTTP ERROR CLASSIFICATION INTEGRATION TESTS
// ============================================================================
//

/// Test case structure for table-driven testing.
#[derive(Debug, Clone)]
struct ErrorClassificationTestCase {
    /// Test case name for clear identification.
    name: &'static str,
    /// HTTP status code to test.
    status_code: u16,
    /// Error code from the API response.
    error_code: &'static str,
    /// Message from the API response.
    message: &'static str,
    /// Expected error variant and specific assertions.
    expected_assertion: fn(&Error),
}

/// Helper function to create a test client.
fn create_test_client() -> Client {
    ClientBuilder::new()
        .network(Network::Local)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Test client creation should not fail")
}

/// Helper function to simulate an error response and test classification.
fn test_error_classification(test_case: &ErrorClassificationTestCase) {
    let client = create_test_client();

    // Create an ErrorResponse for structured error testing
    let error_response = ErrorResponse {
        error_code: test_case.error_code.to_string(),
        message: test_case.message.to_string(),
    };

    // Serialize to JSON to simulate API response
    let response_body =
        serde_json::to_string(&error_response).expect("Test ErrorResponse should serialize");

    // Test the classify_error method via handle_error_response
    let result = client.test_handle_error_response(test_case.status_code, &response_body);

    // Apply the expected assertion
    (test_case.expected_assertion)(&result);
}

/// Helper function to test fallback error classification with non-JSON body.
fn test_fallback_error_classification(
    status_code: u16,
    body: &str,
    expected_assertion: fn(&Error),
) {
    let client = create_test_client();
    let result = client.test_handle_error_response(status_code, body);
    expected_assertion(&result);
}

#[test]
fn test_validation_parameter_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "validation_username",
            status_code: 400,
            error_code: "validation_username",
            message: "Username must be at least 3 characters",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "username");
                    assert_eq!(message, "Username must be at least 3 characters");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_email",
            status_code: 400,
            error_code: "validation_email",
            message: "Invalid email format",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "email");
                    assert_eq!(message, "Invalid email format");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_amount",
            status_code: 400,
            error_code: "validation_amount",
            message: "Amount must be positive",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "amount");
                    assert_eq!(message, "Amount must be positive");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_address",
            status_code: 400,
            error_code: "validation_address",
            message: "Invalid address format",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "address");
                    assert_eq!(message, "Invalid address format");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_authentication_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "unauthorized_invalid_token",
            status_code: 401,
            error_code: "invalid_token",
            message: "The provided token is invalid",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "The provided token is invalid");
                }
                _ => panic!("Expected Authentication error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "unauthorized_expired_token",
            status_code: 401,
            error_code: "expired_token",
            message: "Token has expired",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "Token has expired");
                }
                _ => panic!("Expected Authentication error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_authorization_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "forbidden_insufficient_permissions",
            status_code: 403,
            error_code: "insufficient_permissions",
            message: "You do not have permission to access this resource",
            expected_assertion: |error| match error {
                Error::Authorization(message) => {
                    assert_eq!(
                        message,
                        "You do not have permission to access this resource"
                    );
                }
                _ => panic!("Expected Authorization error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "forbidden_access_denied",
            status_code: 403,
            error_code: "access_denied",
            message: "Access denied for this operation",
            expected_assertion: |error| match error {
                Error::Authorization(message) => {
                    assert_eq!(message, "Access denied for this operation");
                }
                _ => panic!("Expected Authorization error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_resource_not_found_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "resource_transaction_not_found",
            status_code: 404,
            error_code: "resource_transaction",
            message: "Transaction with hash 0x123 not found",
            expected_assertion: |error| match error {
                Error::ResourceNotFound {
                    resource_type,
                    identifier,
                } => {
                    assert_eq!(resource_type, "transaction");
                    assert_eq!(identifier, "Transaction with hash 0x123 not found");
                }
                _ => panic!("Expected ResourceNotFound error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "resource_account_not_found",
            status_code: 404,
            error_code: "resource_account",
            message: "Account not found",
            expected_assertion: |error| match error {
                Error::ResourceNotFound {
                    resource_type,
                    identifier,
                } => {
                    assert_eq!(resource_type, "account");
                    assert_eq!(identifier, "Account not found");
                }
                _ => panic!("Expected ResourceNotFound error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_business_logic_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "business_transfer_failed",
            status_code: 422,
            error_code: "business_transfer",
            message: "Insufficient balance for transfer",
            expected_assertion: |error| match error {
                Error::BusinessLogic { operation, reason } => {
                    assert_eq!(operation, "transfer");
                    assert_eq!(reason, "Insufficient balance for transfer");
                }
                _ => panic!("Expected BusinessLogic error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "business_mint_failed",
            status_code: 422,
            error_code: "business_mint",
            message: "Cannot mint to blacklisted address",
            expected_assertion: |error| match error {
                Error::BusinessLogic { operation, reason } => {
                    assert_eq!(operation, "mint");
                    assert_eq!(reason, "Cannot mint to blacklisted address");
                }
                _ => panic!("Expected BusinessLogic error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_rate_limit_errors() {
    let test_case = ErrorClassificationTestCase {
        name: "rate_limit_exceeded_exact_match",
        status_code: 429,
        error_code: "rate_limit_exceeded",
        message: "Too many requests",
        expected_assertion: |error| match error {
            Error::RateLimitExceeded {
                retry_after_seconds,
            } => {
                assert_eq!(*retry_after_seconds, None);
            }
            _ => panic!("Expected RateLimitExceeded error, got: {:?}", error),
        },
    };
    test_error_classification(&test_case);
}

#[test]
fn test_server_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "system_database_error",
            status_code: 500,
            error_code: "system_database",
            message: "Database connection failed",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "Database connection failed");
                    assert_eq!(*status_code, Some(500));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "system_service_unavailable",
            status_code: 503,
            error_code: "system_service",
            message: "Service temporarily unavailable",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "Service temporarily unavailable");
                    assert_eq!(*status_code, Some(503));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_fallback_error_classification_cases() {
    // Test fallback behavior when response body is not valid JSON

    // 400 Bad Request fallback
    test_fallback_error_classification(400, "Invalid request", |error| match error {
        Error::InvalidParameter { parameter, message } => {
            assert_eq!(parameter, "request");
            assert_eq!(message, "Invalid request");
        }
        _ => panic!(
            "Expected InvalidParameter for 400 fallback, got: {:?}",
            error
        ),
    });

    // 401 Unauthorized fallback
    test_fallback_error_classification(401, "Unauthorized access", |error| match error {
        Error::Authentication(message) => {
            assert_eq!(message, "Unauthorized access");
        }
        _ => panic!("Expected Authentication for 401 fallback, got: {:?}", error),
    });

    // 404 Not Found fallback
    test_fallback_error_classification(404, "Resource not found", |error| match error {
        Error::ResourceNotFound {
            resource_type,
            identifier,
        } => {
            assert_eq!(resource_type, "unknown");
            assert_eq!(identifier, "Resource not found");
        }
        _ => panic!(
            "Expected ResourceNotFound for 404 fallback, got: {:?}",
            error
        ),
    });

    // 500 Internal Server Error fallback
    test_fallback_error_classification(500, "Internal server error", |error| match error {
        Error::HttpTransport {
            message,
            status_code,
        } => {
            assert_eq!(message, "Internal server error");
            assert_eq!(*status_code, Some(500));
        }
        _ => panic!("Expected HttpTransport for 500 fallback, got: {:?}", error),
    });
}

#[test]
fn test_error_classification_edge_cases() {
    // Test various edge cases for error classification

    let edge_cases = vec![
        ErrorClassificationTestCase {
            name: "empty_error_code",
            status_code: 400,
            error_code: "",
            message: "Empty error code",
            expected_assertion: |error| match error {
                Error::Api {
                    status_code,
                    error_code,
                    message,
                } => {
                    assert_eq!(*status_code, 400);
                    assert_eq!(error_code, "");
                    assert_eq!(message, "Empty error code");
                }
                _ => panic!("Expected Api error for empty error code, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_empty_suffix",
            status_code: 400,
            error_code: "validation_",
            message: "Parameter validation failed",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "");
                    assert_eq!(message, "Parameter validation failed");
                }
                _ => panic!(
                    "Expected InvalidParameter error with empty param, got: {:?}",
                    error
                ),
            },
        },
        ErrorClassificationTestCase {
            name: "system_error_range_500",
            status_code: 500,
            error_code: "system_test",
            message: "System error occurred",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "System error occurred");
                    assert_eq!(*status_code, Some(500));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
    ];

    for test_case in edge_cases {
        println!("Running edge case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

//
// ============================================================================
// HTTP TRANSPORT INTEGRATION TESTS
// ============================================================================
//

#[test]
fn test_http_transport_error_status_codes() {
    // Test HTTP transport errors with various status codes in 500-599 range
    let system_status_codes = [500, 501, 502, 503, 504, 505, 550, 599];

    for &status_code in &system_status_codes {
        let test_case = ErrorClassificationTestCase {
            name: "system_error_boundary",
            status_code,
            error_code: "system_test",
            message: "System error occurred",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code: status,
                } => {
                    assert_eq!(message, "System error occurred");
                    assert!(status.unwrap() >= 500 && status.unwrap() <= 599);
                }
                _ => panic!(
                    "Expected HttpTransport error for system_ error, got: {:?}",
                    error
                ),
            },
        };
        test_error_classification(&test_case);
    }
}

#[test]
fn test_http_client_cross_network_functionality() -> Result<()> {
    // Test that different network clients can be created and configured
    let networks_and_expected_differences = [
        (Network::Mainnet, "mainnet"),
        (Network::Testnet, "testnet"),
        (Network::Local, "local"),
    ];

    for (network, expected_name) in networks_and_expected_differences {
        let client = ClientBuilder::new()
            .network(network)
            .timeout(Duration::from_secs(30))
            .build()?;

        let debug_output = format!("{:?}", client);

        // All clients should have basic structure
        assert!(debug_output.contains("Client"));
        assert!(debug_output.contains("base_url"));
        assert!(debug_output.contains("hooks_count: 0"));

        // Network-specific tests could be added here if URLs contain network identifiers
        println!("Created {} client successfully", expected_name);
    }

    Ok(())
}

#[test]
fn test_timeout_configuration_integration() {
    // Test various timeout configurations
    let timeout_scenarios = [
        ("very_short", Duration::from_millis(100)),
        ("short", Duration::from_secs(1)),
        ("medium", Duration::from_secs(10)),
        ("long", Duration::from_secs(60)),
        ("very_long", Duration::from_secs(300)),
    ];

    for (scenario_name, timeout) in timeout_scenarios {
        let result = ClientBuilder::new()
            .network(Network::Local)
            .timeout(timeout)
            .build();

        assert!(
            result.is_ok(),
            "Failed to create client with {} timeout",
            scenario_name
        );

        let client = result.unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("Client"));

        println!(
            "Successfully created client with {} timeout: {:?}",
            scenario_name, timeout
        );
    }
}
