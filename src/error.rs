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
    pub fn response_deserialization<T: Into<String>>(format: T, error: T, response: T) -> Self {
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
    pub fn invalid_parameter<T: Into<String>>(parameter: T, message: T) -> Self {
        Self::InvalidParameter {
            parameter: parameter.into(),
            message: message.into(),
        }
    }

    /// Create a resource not found error.
    pub fn resource_not_found<T: Into<String>>(resource_type: T, identifier: T) -> Self {
        Self::ResourceNotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// Create a business logic error.
    pub fn business_logic<T: Into<String>>(operation: T, reason: T) -> Self {
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
            Error::invalid_parameter("request", &format!("Request error: {}", err))
        } else if err.is_decode() {
            Error::response_deserialization(
                "JSON",
                &err.to_string(),
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
