//! Comprehensive error coverage tests

use onemoney_protocol::error::*;
use std::array::TryFromSliceError;

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

    // Test RLP decoding error conversion
    let rlp_err = rlp::decode::<String>(&[0xff]).unwrap_err();
    let error: Error = rlp_err.into();
    assert!(format!("{}", error).contains("RLP encoding/decoding failed"));
}

#[test]
fn test_error_from_crypto_error() {
    let crypto_error = CryptoError::invalid_private_key("test");
    let error: Error = crypto_error.into();
    assert!(error.is_crypto_error());
    assert!(format!("{}", error).contains("test"));
}

#[test]
fn test_error_from_config_error() {
    let config_error = ConfigError::invalid_timeout("test");
    let error: Error = config_error.into();
    assert!(error.is_config_error());
    assert!(format!("{}", error).contains("test"));
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

#[test]
fn test_error_response_serialization() {
    let error_response = ErrorResponse {
        error_code: "INVALID_INPUT".to_string(),
        message: "The provided input is invalid".to_string(),
    };

    let json = serde_json::to_string(&error_response).unwrap();
    let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(error_response.error_code, deserialized.error_code);
    assert_eq!(error_response.message, deserialized.message);
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
