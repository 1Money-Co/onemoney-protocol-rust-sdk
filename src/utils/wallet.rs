//! EVM wallet utilities for key generation.

use super::address::public_key_to_address;
use crate::Result;
use alloy_primitives::Address;
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
    pub address: Address,
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wallet = EvmWallet::generate_random()?;
    /// println!("Generated wallet: {}", wallet);
    /// println!("Address: {}", wallet.address);
    /// println!("Private key: {}", wallet.private_key);
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate_random() -> Result<Self> {
        // Generate a random private key
        let signing_key = SigningKey::random(&mut OsRng);

        // Get private key bytes
        let private_key_bytes = signing_key.to_bytes();
        #[allow(deprecated)]
        let private_key = format!("0x{}", hex_encode(private_key_bytes.as_slice()));

        // Get public key
        let verifying_key = VerifyingKey::from(&signing_key);
        let public_key_point = verifying_key.to_encoded_point(false);
        let public_key_bytes = public_key_point.as_bytes();
        let public_key = format!("0x{}", hex_encode(public_key_bytes));

        // Generate address from public key
        let address = public_key_to_address(&public_key)?;

        Ok(EvmWallet {
            private_key,
            public_key,
            address,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::is_valid_address_format;

    use super::*;

    #[test]
    fn test_generate_random_wallet() {
        let wallet = EvmWallet::generate_random().expect("Failed to generate random wallet");

        // Check that all fields are populated
        assert!(!wallet.private_key.is_empty());
        assert!(wallet.private_key.starts_with("0x"));
        assert_eq!(wallet.private_key.len(), 66); // 0x + 64 hex chars

        assert!(!wallet.public_key.is_empty());
        assert!(wallet.public_key.starts_with("0x"));
        assert_eq!(wallet.public_key.len(), 132); // 0x + 130 hex chars (65 bytes * 2)

        assert_ne!(wallet.address, Address::ZERO);

        println!("Generated wallet: {}", wallet);
    }

    #[test]
    fn test_wallet_generation_uniqueness() {
        // Test from coverage_tests.rs - ensure each generation creates unique wallets
        let wallet1 = EvmWallet::generate_random().expect("Failed to generate first wallet");
        let wallet2 = EvmWallet::generate_random().expect("Failed to generate second wallet");

        // Ensure different wallets are generated
        assert_ne!(wallet1.private_key, wallet2.private_key);
        assert_ne!(wallet1.public_key, wallet2.public_key);
        assert_ne!(wallet1.address, wallet2.address);
    }

    #[test]
    fn test_wallet_format_validation() {
        let wallet = EvmWallet::generate_random().expect("Failed to generate wallet");

        // Validate private key format
        assert!(wallet.private_key.starts_with("0x"));
        let private_key_hex = &wallet.private_key[2..];
        assert_eq!(private_key_hex.len(), 64);
        assert!(private_key_hex.chars().all(|c| c.is_ascii_hexdigit()));

        // Validate public key format
        assert!(wallet.public_key.starts_with("0x04")); // Uncompressed format
        let public_key_hex = &wallet.public_key[2..];
        assert_eq!(public_key_hex.len(), 130);
        assert!(public_key_hex.chars().all(|c| c.is_ascii_hexdigit()));

        let address_str = wallet.address.to_string();
        assert!(is_valid_address_format(&address_str));
    }

    #[test]
    fn test_wallet_display() {
        let wallet = EvmWallet::generate_random().expect("Failed to generate wallet");
        let display_str = format!("{}", wallet);

        assert!(display_str.contains("EVM Wallet:"));
        assert!(display_str.contains("address:"));
        assert!(display_str.contains("private_key:"));
        assert!(display_str.contains("public_key:"));
        assert!(display_str.contains(&wallet.address.to_string()));
    }

    #[test]
    fn test_wallet_serialization() {
        let wallet = EvmWallet::generate_random().expect("Failed to generate wallet");

        // Test JSON serialization/deserialization
        let json = serde_json::to_string(&wallet).expect("Failed to serialize wallet");
        let deserialized: EvmWallet =
            serde_json::from_str(&json).expect("Failed to deserialize wallet");

        assert_eq!(wallet.private_key, deserialized.private_key);
        assert_eq!(wallet.public_key, deserialized.public_key);
        assert_eq!(wallet.address, deserialized.address);
    }
}
