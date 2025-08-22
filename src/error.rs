//! Error types for the OneMoney SDK.

use serde::{Deserialize, Serialize};
use std::array::TryFromSliceError;
use std::result::Result as StdResult;
use thiserror::Error;

/// Result type alias for OneMoney SDK operations.
pub type Result<T> = StdResult<T, Error>;

/// Main error type for the OneMoney SDK.
#[derive(Error, Debug)]
pub enum Error {
    /// JSON serialization/deserialization error.
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// API error returned by the server.
    #[error("API error {status_code}: {error_code} - {message}")]
    Api {
        status_code: u16,
        error_code: String,
        message: String,
    },

    /// HTTP transport error with optional status code.
    #[error("HTTP transport error: {message}")]
    HttpTransport {
        message: String,
        status_code: Option<u16>,
    },

    /// Request timeout error.
    #[error("Request timeout after {timeout_ms}ms to {endpoint}")]
    RequestTimeout { endpoint: String, timeout_ms: u64 },

    /// Connection error.
    #[error("Connection failed: {0}")]
    Connection(String),

    /// DNS resolution error.
    #[error("DNS resolution failed: {0}")]
    DnsResolution(String),

    /// Response deserialization error.
    #[error("Failed to deserialize {format} response: {error} - Response: {response}")]
    ResponseDeserialization {
        format: String,
        error: String,
        response: String,
    },

    /// Authentication error.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Authorization error.
    #[error("Authorization failed: {0}")]
    Authorization(String),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded")]
    RateLimitExceeded { retry_after_seconds: Option<u64> },

    /// Invalid request parameter.
    #[error("Invalid parameter '{parameter}': {message}")]
    InvalidParameter { parameter: String, message: String },

    /// Resource not found.
    #[error("Resource not found: {resource_type} with {identifier}")]
    ResourceNotFound {
        resource_type: String,
        identifier: String,
    },

    /// Business logic error.
    #[error("Business logic error: {operation} failed - {reason}")]
    BusinessLogic { operation: String, reason: String },

    /// Cryptographic operation errors.
    #[error("Cryptographic operation failed: {0}")]
    Crypto(#[from] CryptoError),

    /// Client configuration errors.
    #[error("Client configuration error: {0}")]
    Config(#[from] ConfigError),

    /// URL parsing error.
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Hex decoding error.
    #[error("Hex decoding failed: {0}")]
    Hex(#[from] hex::FromHexError),

    /// RLP encoding/decoding error.
    #[error("RLP encoding/decoding failed: {0}")]
    Rlp(#[from] rlp::DecoderError),

    /// Address parsing error.
    #[error("Invalid address format: {0}")]
    Address(String),

    /// Array conversion error.
    #[error("Array conversion failed: expected length {expected}, got {actual}")]
    ArrayConversion { expected: usize, actual: usize },

    /// Validation error for input parameters.
    #[error("Validation failed: {field} - {message}")]
    Validation { field: String, message: String },

    /// Generic error with custom message.
    #[error("{0}")]
    Custom(String),
}

/// Cryptographic operation errors.
#[derive(Error, Debug)]
pub enum CryptoError {
    /// Invalid private key format or content.
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Invalid public key format or content.
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    /// Signature creation failed.
    #[error("Failed to create signature: {0}")]
    SignatureFailed(String),

    /// Signature verification failed.
    #[error("Signature verification failed: {0}")]
    VerificationFailed(String),

    /// Key derivation error.
    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),
}

/// Client configuration errors.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid timeout value.
    #[error("Invalid timeout: {0}")]
    InvalidTimeout(String),

    /// Invalid network configuration.
    #[error("Invalid network configuration: {0}")]
    InvalidNetwork(String),

    /// Missing required configuration.
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),

    /// HTTP client builder failed.
    #[error("Failed to build HTTP client: {0}")]
    ClientBuilder(String),
}

/// API error response structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

impl Error {
    /// Create a new API error.
    pub fn api(status_code: u16, error_code: String, message: String) -> Self {
        Self::Api {
            status_code,
            error_code,
            message,
        }
    }

    /// Create an address parsing error.
    pub fn address<T: Into<String>>(msg: T) -> Self {
        Self::Address(msg.into())
    }

    /// Create an array conversion error.
    pub fn array_conversion(expected: usize, actual: usize) -> Self {
        Self::ArrayConversion { expected, actual }
    }

    /// Create a validation error.
    pub fn validation<T: Into<String>, U: Into<String>>(field: T, message: U) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a custom error.
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Self::Custom(msg.into())
    }

    /// Check if this is an API error.
    pub fn is_api_error(&self) -> bool {
        matches!(self, Self::Api { .. })
    }

    /// Check if this is a configuration error.
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::Config(_))
    }

    /// Check if this is a cryptographic error.
    pub fn is_crypto_error(&self) -> bool {
        matches!(self, Self::Crypto(_))
    }

    /// Get the status code if this is an API error.
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status_code, .. } => Some(*status_code),
            _ => None,
        }
    }

    /// Get the error code if this is an API error.
    pub fn error_code(&self) -> Option<&str> {
        match self {
            Self::Api { error_code, .. } => Some(error_code),
            _ => None,
        }
    }

    /// Create an HTTP transport error.
    pub fn http_transport<T: Into<String>>(message: T, status_code: Option<u16>) -> Self {
        Self::HttpTransport {
            message: message.into(),
            status_code,
        }
    }

    /// Create a request timeout error.
    pub fn request_timeout<T: Into<String>>(endpoint: T, timeout_ms: u64) -> Self {
        Self::RequestTimeout {
            endpoint: endpoint.into(),
            timeout_ms,
        }
    }

    /// Create a connection error.
    pub fn connection<T: Into<String>>(message: T) -> Self {
        Self::Connection(message.into())
    }

    /// Create a DNS resolution error.
    pub fn dns_resolution<T: Into<String>>(message: T) -> Self {
        Self::DnsResolution(message.into())
    }

    /// Create a response deserialization error.
    pub fn response_deserialization<A: Into<String>, B: Into<String>, C: Into<String>>(
        format: A,
        error: B,
        response: C,
    ) -> Self {
        Self::ResponseDeserialization {
            format: format.into(),
            error: error.into(),
            response: response.into(),
        }
    }

    /// Create an authentication error.
    pub fn authentication<T: Into<String>>(message: T) -> Self {
        Self::Authentication(message.into())
    }

    /// Create an authorization error.
    pub fn authorization<T: Into<String>>(message: T) -> Self {
        Self::Authorization(message.into())
    }

    /// Create a rate limit exceeded error.
    pub fn rate_limit_exceeded(retry_after_seconds: Option<u64>) -> Self {
        Self::RateLimitExceeded {
            retry_after_seconds,
        }
    }

    /// Create an invalid parameter error.
    pub fn invalid_parameter<A: Into<String>, B: Into<String>>(parameter: A, message: B) -> Self {
        Self::InvalidParameter {
            parameter: parameter.into(),
            message: message.into(),
        }
    }

    /// Create a resource not found error.
    pub fn resource_not_found<A: Into<String>, B: Into<String>>(
        resource_type: A,
        identifier: B,
    ) -> Self {
        Self::ResourceNotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// Create a business logic error.
    pub fn business_logic<A: Into<String>, B: Into<String>>(operation: A, reason: B) -> Self {
        Self::BusinessLogic {
            operation: operation.into(),
            reason: reason.into(),
        }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_err: TryFromSliceError) -> Self {
        Self::ArrayConversion {
            expected: 32, // Most common case for crypto operations
            actual: 0,    // We don't have the actual length in TryFromSliceError
        }
    }
}

/// Enhanced reqwest error mapping with L1 compatibility.
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::request_timeout(
                err.url()
                    .map(|u| u.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                30000, // Default timeout assumption
            )
        } else if err.is_connect() {
            Error::connection(format!("Connection failed: {}", err))
        } else if err.is_request() {
            Error::invalid_parameter("request", format!("Request error: {}", err))
        } else if err.is_decode() {
            Error::response_deserialization(
                "JSON",
                err.to_string(),
                "Failed to decode response body",
            )
        } else {
            // Check for specific HTTP status codes if available
            if let Some(status) = err.status() {
                Error::http_transport(err.to_string(), Some(status.as_u16()))
            } else {
                Error::http_transport(err.to_string(), None)
            }
        }
    }
}

impl CryptoError {
    /// Create an invalid private key error.
    pub fn invalid_private_key<T: Into<String>>(msg: T) -> Self {
        Self::InvalidPrivateKey(msg.into())
    }

    /// Create an invalid public key error.
    pub fn invalid_public_key<T: Into<String>>(msg: T) -> Self {
        Self::InvalidPublicKey(msg.into())
    }

    /// Create a signature failed error.
    pub fn signature_failed<T: Into<String>>(msg: T) -> Self {
        Self::SignatureFailed(msg.into())
    }

    /// Create a verification failed error.
    pub fn verification_failed<T: Into<String>>(msg: T) -> Self {
        Self::VerificationFailed(msg.into())
    }

    /// Create a key derivation error.
    pub fn key_derivation<T: Into<String>>(msg: T) -> Self {
        Self::KeyDerivation(msg.into())
    }
}

impl ConfigError {
    /// Create an invalid timeout error.
    pub fn invalid_timeout<T: Into<String>>(msg: T) -> Self {
        Self::InvalidTimeout(msg.into())
    }

    /// Create an invalid network error.
    pub fn invalid_network<T: Into<String>>(msg: T) -> Self {
        Self::InvalidNetwork(msg.into())
    }

    /// Create a missing config error.
    pub fn missing_config<T: Into<String>>(msg: T) -> Self {
        Self::MissingConfig(msg.into())
    }

    /// Create a client builder error.
    pub fn client_builder<T: Into<String>>(msg: T) -> Self {
        Self::ClientBuilder(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_error_creation_methods() {
        // Test API error creation
        let api_error = Error::api(
            404,
            "resource_not_found".to_string(),
            "Transaction not found".to_string(),
        );
        assert!(matches!(
            api_error,
            Error::Api {
                status_code: 404,
                ..
            }
        ));
        assert_eq!(api_error.status_code(), Some(404));
        assert_eq!(api_error.error_code(), Some("resource_not_found"));

        // Test address error creation
        let addr_error = Error::address("Invalid address format");
        assert!(matches!(addr_error, Error::Address(_)));

        // Test array conversion error creation
        let array_error = Error::array_conversion(32, 16);
        assert!(matches!(
            array_error,
            Error::ArrayConversion {
                expected: 32,
                actual: 16
            }
        ));

        // Test validation error creation
        let validation_error = Error::validation("email", "Invalid email format");
        assert!(matches!(validation_error, Error::Validation { .. }));

        // Test custom error creation
        let custom_error = Error::custom("Custom error message");
        assert!(matches!(custom_error, Error::Custom(_)));
    }

    #[test]
    fn test_http_transport_error_creation() {
        let error_with_status = Error::http_transport("Connection failed", Some(500));
        assert!(matches!(
            error_with_status,
            Error::HttpTransport {
                status_code: Some(500),
                ..
            }
        ));

        let error_without_status = Error::http_transport("Connection failed", None);
        assert!(matches!(
            error_without_status,
            Error::HttpTransport {
                status_code: None,
                ..
            }
        ));
    }

    #[test]
    fn test_request_timeout_error_creation() {
        let timeout_error = Error::request_timeout("/api/transactions", 30000);
        assert!(matches!(
            timeout_error,
            Error::RequestTimeout {
                timeout_ms: 30000,
                ..
            }
        ));
    }

    #[test]
    fn test_authentication_and_authorization_errors() {
        let auth_error = Error::authentication("Invalid signature");
        assert!(matches!(auth_error, Error::Authentication(_)));

        let authz_error = Error::authorization("Insufficient permissions");
        assert!(matches!(authz_error, Error::Authorization(_)));
    }

    #[test]
    fn test_rate_limit_error_creation() {
        let rate_limit_with_retry = Error::rate_limit_exceeded(Some(60));
        assert!(matches!(
            rate_limit_with_retry,
            Error::RateLimitExceeded {
                retry_after_seconds: Some(60)
            }
        ));

        let rate_limit_without_retry = Error::rate_limit_exceeded(None);
        assert!(matches!(
            rate_limit_without_retry,
            Error::RateLimitExceeded {
                retry_after_seconds: None
            }
        ));
    }

    #[test]
    fn test_parameter_and_resource_errors() {
        let param_error = Error::invalid_parameter("amount", "Amount must be positive");
        assert!(matches!(param_error, Error::InvalidParameter { .. }));

        let resource_error = Error::resource_not_found("transaction", "0x123abc");
        assert!(matches!(resource_error, Error::ResourceNotFound { .. }));
    }

    #[test]
    fn test_business_logic_error_creation() {
        let business_error = Error::business_logic("transfer", "Insufficient balance");
        assert!(matches!(business_error, Error::BusinessLogic { .. }));
    }

    #[test]
    fn test_connection_and_dns_errors() {
        let conn_error = Error::connection("Failed to connect to server");
        assert!(matches!(conn_error, Error::Connection(_)));

        let dns_error = Error::dns_resolution("Could not resolve hostname");
        assert!(matches!(dns_error, Error::DnsResolution(_)));
    }

    #[test]
    fn test_response_deserialization_error() {
        let deser_error =
            Error::response_deserialization("JSON", "unexpected end of input", "{\"invalid\":");
        assert!(matches!(deser_error, Error::ResponseDeserialization { .. }));
    }

    #[test]
    fn test_error_type_checking_methods() {
        let api_error = Error::api(
            500,
            "server_error".to_string(),
            "Internal server error".to_string(),
        );
        assert!(api_error.is_api_error());
        assert!(!api_error.is_config_error());
        assert!(!api_error.is_crypto_error());

        let config_error =
            Error::Config(ConfigError::InvalidTimeout("Timeout too large".to_string()));
        assert!(!config_error.is_api_error());
        assert!(config_error.is_config_error());
        assert!(!config_error.is_crypto_error());

        let crypto_error = Error::Crypto(CryptoError::InvalidPrivateKey(
            "Invalid key format".to_string(),
        ));
        assert!(!crypto_error.is_api_error());
        assert!(!crypto_error.is_config_error());
        assert!(crypto_error.is_crypto_error());
    }

    #[test]
    fn test_status_code_and_error_code_extraction() {
        let api_error = Error::api(
            422,
            "business_logic_error".to_string(),
            "Invalid operation".to_string(),
        );
        assert_eq!(api_error.status_code(), Some(422));
        assert_eq!(api_error.error_code(), Some("business_logic_error"));

        let non_api_error = Error::custom("Not an API error");
        assert_eq!(non_api_error.status_code(), None);
        assert_eq!(non_api_error.error_code(), None);
    }

    #[test]
    fn test_crypto_error_creation() {
        let invalid_private_key = CryptoError::invalid_private_key("Key too short");
        assert!(matches!(
            invalid_private_key,
            CryptoError::InvalidPrivateKey(_)
        ));

        let invalid_public_key = CryptoError::invalid_public_key("Invalid format");
        assert!(matches!(
            invalid_public_key,
            CryptoError::InvalidPublicKey(_)
        ));

        let signature_failed = CryptoError::signature_failed("Could not create signature");
        assert!(matches!(signature_failed, CryptoError::SignatureFailed(_)));

        let verification_failed = CryptoError::verification_failed("Signature mismatch");
        assert!(matches!(
            verification_failed,
            CryptoError::VerificationFailed(_)
        ));

        let key_derivation = CryptoError::key_derivation("Derivation failed");
        assert!(matches!(key_derivation, CryptoError::KeyDerivation(_)));
    }

    #[test]
    fn test_config_error_creation() {
        let invalid_timeout = ConfigError::invalid_timeout("Timeout cannot be zero");
        assert!(matches!(invalid_timeout, ConfigError::InvalidTimeout(_)));

        let invalid_network = ConfigError::invalid_network("Unknown network");
        assert!(matches!(invalid_network, ConfigError::InvalidNetwork(_)));

        let missing_config = ConfigError::missing_config("API key required");
        assert!(matches!(missing_config, ConfigError::MissingConfig(_)));

        let client_builder = ConfigError::client_builder("Failed to build HTTP client");
        assert!(matches!(client_builder, ConfigError::ClientBuilder(_)));
    }

    #[test]
    fn test_error_display_formatting() {
        // Test different error display formats
        let api_error = Error::api(
            404,
            "not_found".to_string(),
            "Resource not found".to_string(),
        );
        let display_str = format!("{}", api_error);
        assert!(display_str.contains("API error 404"));
        assert!(display_str.contains("not_found"));
        assert!(display_str.contains("Resource not found"));

        let timeout_error = Error::request_timeout("/api/test", 5000);
        let timeout_str = format!("{}", timeout_error);
        assert!(timeout_str.contains("Request timeout after 5000ms"));
        assert!(timeout_str.contains("/api/test"));

        let param_error = Error::invalid_parameter("amount", "Must be positive");
        let param_str = format!("{}", param_error);
        assert!(param_str.contains("Invalid parameter 'amount'"));
        assert!(param_str.contains("Must be positive"));
    }

    #[test]
    fn test_error_from_conversions() {
        // Test From<CryptoError> conversion
        let crypto_error = CryptoError::invalid_private_key("Invalid key");
        let error: Error = crypto_error.into();
        assert!(matches!(error, Error::Crypto(_)));

        // Test From<ConfigError> conversion
        let config_error = ConfigError::invalid_timeout("Invalid timeout");
        let error: Error = config_error.into();
        assert!(matches!(error, Error::Config(_)));

        // Test From<TryFromSliceError> conversion
        // Create a TryFromSliceError by attempting to convert a slice that's too short
        let result: StdResult<[u8; 4], std::array::TryFromSliceError> =
            [0u8; 2].as_slice().try_into();
        let slice_error = result.unwrap_err();
        let error: Error = slice_error.into();
        assert!(matches!(
            error,
            Error::ArrayConversion {
                expected: 32,
                actual: 0
            }
        ));
    }

    #[test]
    fn test_error_response_structure() {
        let error_response = ErrorResponse {
            error_code: "validation_error".to_string(),
            message: "Invalid input parameters".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&error_response).expect("Should serialize");
        assert!(json.contains("validation_error"));
        assert!(json.contains("Invalid input parameters"));

        // Test deserialization
        let deserialized: ErrorResponse = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.error_code, "validation_error");
        assert_eq!(deserialized.message, "Invalid input parameters");
    }

    #[test]
    fn test_reqwest_error_conversion() {
        // Note: These tests use mock errors since we can't easily create real reqwest errors
        // In practice, reqwest errors would be converted automatically via the From trait

        // Test that the From<reqwest::Error> implementation exists and compiles
        // This ensures the conversion logic is syntactically correct
        // The implementation is tested indirectly through other integration tests
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = Error::api(
            500,
            "server_error".to_string(),
            "Internal error".to_string(),
        );
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Api"));
        assert!(debug_str.contains("status_code: 500"));

        let crypto_error = CryptoError::invalid_private_key("Invalid format");
        let crypto_debug = format!("{:?}", crypto_error);
        assert!(crypto_debug.contains("InvalidPrivateKey"));

        let config_error = ConfigError::invalid_network("Unknown network");
        let config_debug = format!("{:?}", config_error);
        assert!(config_debug.contains("InvalidNetwork"));
    }

    #[test]
    fn test_result_type_alias() {
        // Test that our Result type alias works correctly
        let success_result: Result<String> = Ok("success".to_string());
        assert!(success_result.is_ok());
        if let Ok(value) = success_result {
            assert_eq!(value, "success");
        }

        let error_result: Result<String> = Err(Error::custom("test error"));
        assert!(error_result.is_err());
        if let Err(error) = error_result {
            assert!(matches!(error, Error::Custom(_)));
        }
    }

    #[test]
    fn test_error_source_chain() {
        // Test that errors can be chained properly using the source() method from std::error::Error
        let crypto_error = CryptoError::invalid_private_key("Base crypto error");
        let main_error = Error::Crypto(crypto_error);

        // The main error should have the crypto error as its source
        assert!(main_error.source().is_some());

        let config_error = ConfigError::invalid_timeout("Base config error");
        let main_error = Error::Config(config_error);

        // The main error should have the config error as its source
        assert!(main_error.source().is_some());
    }
}
