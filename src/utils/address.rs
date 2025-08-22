//! Address utilities and validation functions.

use crate::{CryptoError, Result};
use alloy_primitives::Address;
use alloy_primitives::keccak256;

/// Convert a public key to an Ethereum address.
///
/// # Arguments
///
/// * `public_key_hex` - Public key as hex string (with or without 0x prefix)
///
/// # Returns
///
/// The corresponding Ethereum address.
pub fn public_key_to_address(public_key_hex: &str) -> Result<Address> {
    let public_key_hex = public_key_hex.strip_prefix("0x").unwrap_or(public_key_hex);

    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CryptoError::invalid_private_key(format!("Invalid public key hex: {}", e)))?;

    if public_key_bytes.is_empty() || public_key_bytes[0] != 0x04 {
        return Err(CryptoError::invalid_private_key(
            "Public key must start with 0x04 (uncompressed format)",
        )
        .into());
    }

    if public_key_bytes.len() != 65 {
        return Err(CryptoError::invalid_private_key(
            "Public key must be 65 bytes (uncompressed format)",
        )
        .into());
    }

    // Hash the public key (skip the 0x04 prefix)
    let hash = keccak256(&public_key_bytes[1..]);

    // Take the last 20 bytes as the address
    let address_bytes = &hash[12..];
    let address = Address::from_slice(address_bytes);

    Ok(address)
}

/// Validate if a string is a valid Ethereum address format.
///
/// # Arguments
///
/// * `address` - Address string to validate
///
/// # Returns
///
/// True if the address format is valid, false otherwise.
pub fn is_valid_address_format(address: &str) -> bool {
    let address = address.strip_prefix("0x").unwrap_or(address);

    // Must be exactly 40 hex characters
    address.len() == 40 && address.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_address_format() {
        // Valid addresses
        assert!(is_valid_address_format(
            "0x1234567890abcdef1234567890abcdef12345678"
        ));
        assert!(is_valid_address_format(
            "1234567890abcdef1234567890abcdef12345678"
        ));
        assert!(is_valid_address_format(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ));

        // Invalid addresses
        assert!(!is_valid_address_format("0x123")); // Too short
        assert!(!is_valid_address_format(
            "0x1234567890abcdef1234567890abcdef123456789"
        )); // Too long
        assert!(!is_valid_address_format(
            "0x1234567890abcdef1234567890abcdef1234567g"
        )); // Invalid char
        assert!(!is_valid_address_format("")); // Empty
    }
}
