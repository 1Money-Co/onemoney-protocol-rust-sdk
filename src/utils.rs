//! Utility functions for address and key generation.

use crate::{CryptoError, OneMoneyAddress, Result};
use alloy_primitives::keccak256;
use hex::encode as hex_encode;
use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::elliptic_curve::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A complete EVM wallet containing private key, public key, and address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmWallet {
    /// Private key as hex string (with 0x prefix).
    pub private_key: String,
    /// Public key as hex string (with 0x prefix).
    pub public_key: String,
    /// Ethereum-style address as hex string (with 0x prefix).
    pub address: OneMoneyAddress,
}

impl Display for EvmWallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "EVM Wallet:\n  address: {}\n  private_key: {}\n  public_key: {}",
            self.address, self.private_key, self.public_key
        )
    }
}

impl EvmWallet {
    /// Generate a new random EVM wallet.
    ///
    /// This creates a new private key using cryptographically secure randomness,
    /// derives the public key and Ethereum address from it.
    ///
    /// # Returns
    ///
    /// A new `EvmWallet` with randomly generated keys and address.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onemoney_protocol::utils::EvmWallet;
    ///
    /// let wallet = EvmWallet::generate_random().unwrap();
    /// println!("Generated wallet: {}", wallet);
    /// println!("Address: {}", wallet.address);
    /// println!("Private key: {}", wallet.private_key);
    /// ```
    pub fn generate_random() -> Result<Self> {
        // Generate a random private key
        let signing_key = SigningKey::random(&mut OsRng);

        // Get private key bytes
        let private_key_bytes = signing_key.to_bytes();
        let private_key = format!("0x{}", hex_encode(private_key_bytes.as_slice()));

        // Get public key
        let verifying_key = VerifyingKey::from(&signing_key);
        let public_key_point = verifying_key.to_encoded_point(false);
        let public_key_bytes = public_key_point.as_bytes();
        let public_key = format!("0x{}", hex_encode(public_key_bytes));

        // Generate address from public key
        let address = Self::public_key_to_address(&public_key)?;

        Ok(EvmWallet {
            private_key,
            public_key,
            address,
        })
    }

    /// Convert a public key to an Ethereum address.
    ///
    /// # Arguments
    ///
    /// * `public_key_hex` - Public key as hex string (with or without 0x prefix)
    ///
    /// # Returns
    ///
    /// The corresponding Ethereum address.
    fn public_key_to_address(public_key_hex: &str) -> Result<OneMoneyAddress> {
        let public_key_hex = public_key_hex.strip_prefix("0x").unwrap_or(public_key_hex);

        let public_key_bytes = hex::decode(public_key_hex).map_err(|e| {
            CryptoError::invalid_private_key(format!("Invalid public key hex: {}", e))
        })?;

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
        let address = alloy_primitives::Address::from_slice(address_bytes);

        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_wallet() {
        let wallet = EvmWallet::generate_random().unwrap();

        // Check that all fields are populated
        assert!(!wallet.private_key.is_empty());
        assert!(wallet.private_key.starts_with("0x"));
        assert_eq!(wallet.private_key.len(), 66); // 0x + 64 hex chars

        assert!(!wallet.public_key.is_empty());
        assert!(wallet.public_key.starts_with("0x"));
        assert_eq!(wallet.public_key.len(), 132); // 0x + 130 hex chars (65 bytes * 2)

        assert_ne!(wallet.address, OneMoneyAddress::ZERO);

        println!("Generated wallet: {}", wallet);
    }
}
