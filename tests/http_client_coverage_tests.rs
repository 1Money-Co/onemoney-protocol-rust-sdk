//! HTTP client coverage tests that only use public APIs

use onemoney_protocol::client::builder::ClientBuilder;
use onemoney_protocol::client::config::Network;
use onemoney_protocol::client::http::Client;
use onemoney_protocol::error::ErrorResponse;

#[test]
fn test_client_creation_methods() {
    // Test mainnet client creation
    let mainnet_client = Client::mainnet();
    let mainnet_debug = format!("{:?}", mainnet_client);
    assert!(mainnet_debug.contains("Client"));
    assert!(mainnet_debug.contains("base_url"));
    assert!(mainnet_debug.contains("hooks_count"));

    // Test testnet client creation
    let testnet_client = Client::testnet();
    let testnet_debug = format!("{:?}", testnet_client);
    assert!(testnet_debug.contains("Client"));

    // Test local client creation
    let local_client = Client::local();
    let local_debug = format!("{:?}", local_client);
    assert!(local_debug.contains("Client"));
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
fn test_client_debug_implementation() {
    let client = Client::mainnet();
    let debug_str = format!("{:?}", client);

    // Verify debug output contains expected fields
    assert!(debug_str.contains("Client"));
    assert!(debug_str.contains("base_url"));
    assert!(debug_str.contains("hooks_count"));

    // Test that hooks_count shows 0 for default client
    assert!(debug_str.contains("hooks_count: 0"));
}

#[test]
fn test_error_response_serialization() {
    let error_response = ErrorResponse {
        error_code: "test_error".to_string(),
        message: "Test error message".to_string(),
    };

    // Test serialization
    let json = serde_json::to_string(&error_response).unwrap();
    assert!(json.contains("test_error"));
    assert!(json.contains("Test error message"));

    // Test deserialization
    let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();
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
fn test_network_client_creation() {
    let mainnet = Client::mainnet();
    let testnet = Client::testnet();
    let local = Client::local();

    // Test that they're created successfully
    let mainnet_debug = format!("{:?}", mainnet);
    let testnet_debug = format!("{:?}", testnet);
    let local_debug = format!("{:?}", local);

    // All should be Client instances
    assert!(mainnet_debug.contains("Client"));
    assert!(testnet_debug.contains("Client"));
    assert!(local_debug.contains("Client"));
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

        let json = serde_json::to_string(&error_response).unwrap();
        assert!(json.contains(error_code));
        assert!(json.contains(message));

        let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(error_response.error_code, deserialized.error_code);
        assert_eq!(error_response.message, deserialized.message);
    }
}

#[test]
fn test_client_builder_timeout() {
    use std::time::Duration;

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
fn test_error_response_edge_cases() {
    // Test with empty strings
    let empty_error = ErrorResponse {
        error_code: String::new(),
        message: String::new(),
    };

    let json = serde_json::to_string(&empty_error).unwrap();
    let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(empty_error.error_code, deserialized.error_code);
    assert_eq!(empty_error.message, deserialized.message);

    // Test with long strings
    let long_error = ErrorResponse {
        error_code: "a".repeat(1000),
        message: "b".repeat(2000),
    };

    let json2 = serde_json::to_string(&long_error).unwrap();
    let deserialized2: ErrorResponse = serde_json::from_str(&json2).unwrap();
    assert_eq!(long_error.error_code, deserialized2.error_code);
    assert_eq!(long_error.message, deserialized2.message);

    // Test with special characters
    let special_error = ErrorResponse {
        error_code: "error_with_ñúmbers_123".to_string(),
        message: "Message with 🚀 emojis and \"quotes\"".to_string(),
    };

    let json3 = serde_json::to_string(&special_error).unwrap();
    let deserialized3: ErrorResponse = serde_json::from_str(&json3).unwrap();
    assert_eq!(special_error.error_code, deserialized3.error_code);
    assert_eq!(special_error.message, deserialized3.message);
}

#[test]
fn test_client_builder_invalid_timeout() {
    use std::time::Duration;

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

        let client = result.unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("Client"));
    }
}

#[test]
fn test_error_response_json_structure() {
    // Test that the JSON structure matches expected format
    let error_response = ErrorResponse {
        error_code: "test_code".to_string(),
        message: "test message".to_string(),
    };

    let json = serde_json::to_string(&error_response).unwrap();

    // Parse as generic JSON to verify structure
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

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
