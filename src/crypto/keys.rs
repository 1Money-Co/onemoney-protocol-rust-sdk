//! Cryptographic key operations and address derivation.

use crate::{CryptoError, Result};
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
pub fn derive_token_account_address(wallet_address: Address, mint_address: Address) -> Address {
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

        let address1 = result.expect("Valid address conversion");
        let address2 = result2.expect("Valid address conversion");
        assert_eq!(address1, address2);

        // Test address format validation using the first address
        assert_ne!(address1, String::default());
        assert!(address1.starts_with("0x"));
        assert_eq!(address1.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    fn test_private_key_to_address_error_cases() {
        // Test invalid hex
        let invalid_hex = "0xzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
        let result = private_key_to_address(invalid_hex);
        assert!(result.is_err());

        // Test wrong length
        let too_short = "0x1234";
        let result = private_key_to_address(too_short);
        assert!(result.is_err());

        // Test empty string
        let empty = "";
        let result = private_key_to_address(empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_token_account_address() {
        let wallet = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Valid wallet address");
        let mint = Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
            .expect("Valid mint address");

        let token_account = derive_token_account_address(wallet, mint);
        assert_ne!(token_account, Address::ZERO);
    }

    #[test]
    fn test_derive_token_account_deterministic() {
        // Test with the same addresses from coverage_tests.rs
        let owner = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Valid owner address");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Valid token address");

        let token_account = derive_token_account_address(owner, token);
        assert_ne!(token_account, Address::ZERO);
        assert_ne!(token_account, owner);
        assert_ne!(token_account, token);

        // Test deterministic behavior - should always return the same result
        let token_account2 = derive_token_account_address(owner, token);
        assert_eq!(token_account, token_account2);
    }

    #[test]
    fn test_derive_token_account_different_inputs() {
        let wallet1 = Address::from_str("0x1111111111111111111111111111111111111111")
            .expect("Valid wallet address");
        let wallet2 = Address::from_str("0x2222222222222222222222222222222222222222")
            .expect("Valid wallet address");
        let mint = Address::from_str("0x3333333333333333333333333333333333333333")
            .expect("Valid mint address");

        let account1 = derive_token_account_address(wallet1, mint);
        let account2 = derive_token_account_address(wallet2, mint);

        // Different wallets should produce different token accounts
        assert_ne!(account1, account2);
    }
}
