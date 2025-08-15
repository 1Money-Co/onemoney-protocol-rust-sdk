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
    /// HTTP client error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

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
}

impl From<TryFromSliceError> for Error {
    fn from(_err: TryFromSliceError) -> Self {
        Self::ArrayConversion {
            expected: 32, // Most common case for crypto operations
            actual: 0,    // We don't have the actual length in TryFromSliceError
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
