//! Cryptographic key operations and address derivation.

use crate::{CryptoError, OneMoneyAddress, Result};
use alloy_primitives::{Address, keccak256};
use hex::decode as hex_decode;
use k256::ecdsa::{SigningKey, VerifyingKey};

/// Convert a private key hex string to an address.
///
/// # Arguments
///
/// * `private_key_hex` - The private key as a hex string (with or without 0x prefix)
///
/// # Returns
///
/// The corresponding Ethereum-style address as a hex string.
pub fn private_key_to_address(private_key_hex: &str) -> Result<String> {
    let private_key_hex = private_key_hex
        .strip_prefix("0x")
        .unwrap_or(private_key_hex);
    let private_key_bytes = hex_decode(private_key_hex)
        .map_err(|e| CryptoError::invalid_private_key(format!("Invalid hex format: {}", e)))?;

    if private_key_bytes.len() != 32 {
        return Err(
            CryptoError::invalid_private_key("Private key must be exactly 32 bytes").into(),
        );
    }

    let key_array: [u8; 32] = private_key_bytes
        .try_into()
        .map_err(|_| CryptoError::invalid_private_key("Private key must be exactly 32 bytes"))?;

    let signing_key = SigningKey::from_bytes(&key_array.into()).map_err(|e| {
        CryptoError::invalid_private_key(format!("Invalid private key format: {}", e))
    })?;

    let verifying_key = VerifyingKey::from(&signing_key);
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();

    // Skip the 0x04 prefix, take the 64 bytes of coordinates
    let hash = keccak256(&public_key_bytes[1..]);

    // Take the last 20 bytes as the address
    let address_bytes = &hash[12..];
    let address = Address::from_slice(address_bytes);

    Ok(address.to_checksum(None))
}

/// Derive a token account address from wallet and mint addresses.
///
/// # Arguments
///
/// * `wallet_address` - The wallet owner address
/// * `mint_address` - The token mint address
///
/// # Returns
///
/// The derived token account address.
pub fn derive_token_account_address(
    wallet_address: OneMoneyAddress,
    mint_address: OneMoneyAddress,
) -> OneMoneyAddress {
    let mut data = Vec::new();
    data.extend_from_slice(&wallet_address[..]);
    data.extend_from_slice(&mint_address[..]);
    data.extend_from_slice(b"token_account");

    let hash = keccak256(&data);
    Address::from_slice(&hash[12..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_private_key_to_address() {
        // Non-sensitive test vector: well-known pattern used across crypto libraries for testing
        // This is NOT a real private key and should never be used with actual funds
        let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = private_key_to_address(private_key);
        assert!(result.is_ok());

        // Test without 0x prefix (same non-sensitive test vector)
        let private_key_no_prefix =
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result2 = private_key_to_address(private_key_no_prefix);
        assert!(result2.is_ok());
        assert_eq!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_derive_token_account_address() {
        let wallet = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();
        let mint = Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12").unwrap();

        let token_account = derive_token_account_address(wallet, mint);
        assert_ne!(token_account, Address::ZERO);
    }
}
