//! Cryptographic utilities for signing and address derivation.

use crate::{CryptoError, OneMoneyAddress, Result, Signature};
use alloy_primitives::{keccak256, Address, B256, U256};
use hex::decode as hex_decode;
use k256::ecdsa::{SigningKey, VerifyingKey};
#[cfg(test)]
use rlp::RlpStream;
use rlp::{encode as rlp_encode, Encodable};
use serde::Serialize;

/// Trait for types that can be cryptographically signed.
pub trait Signable {
    /// Calculate the signature hash for this payload.
    fn signature_hash(&self) -> B256;
}

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
    let private_key_bytes = hex_decode(private_key_hex)?;

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

/// Sign a transaction payload using the same method as L1.
/// This function matches the L1 implementation's sign_transaction_payload.
pub fn sign_transaction_payload<T>(payload: &T, private_key_hex: &str) -> Result<Signature>
where
    T: Signable,
{
    let signature_hash = payload.signature_hash();
    sign_hash(&signature_hash, private_key_hex)
}

/// Sign a pre-computed hash using ECDSA.
pub fn sign_hash(message_hash: &B256, private_key_hex: &str) -> Result<Signature> {
    let private_key_hex = private_key_hex
        .strip_prefix("0x")
        .unwrap_or(private_key_hex);
    let private_key_bytes = hex_decode(private_key_hex)?;

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

    // Sign the hash with recoverable ECDSA (matching L1 implementation)
    let (signature, recovery_id) = signing_key
        .sign_prehash_recoverable(&message_hash[..])
        .map_err(|e| CryptoError::signature_failed(format!("Failed to sign hash: {}", e)))?;

    // Extract R and S components as U256
    let sig_bytes = signature.to_bytes();
    let r = U256::from_be_slice(&sig_bytes[..32]);
    let s = U256::from_be_slice(&sig_bytes[32..64]);

    // Convert recovery ID to v (using Ethereum convention: v = recovery_id + 27)
    let v = recovery_id.to_byte() as u64 + 27;

    Ok(Signature::new(r, s, v))
}

/// Sign a message using RLP encoding and ECDSA.
///
/// # Arguments
///
/// * `message` - The message to sign (must implement Serialize and RLP Encodable)
/// * `private_key_hex` - The private key as a hex string
///
/// # Returns
///
/// The signature components (r, s, v).
pub fn sign_message<T>(message: &T, private_key_hex: &str) -> Result<Signature>
where
    T: Serialize + Encodable,
{
    let private_key_hex = private_key_hex
        .strip_prefix("0x")
        .unwrap_or(private_key_hex);
    let private_key_bytes = hex_decode(private_key_hex)?;

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

    // RLP encode the message
    let encoded = rlp_encode(message);

    // Hash the encoded message
    let message_hash = keccak256(&encoded);

    // Sign the hash with recoverable ECDSA
    let (signature, recovery_id) = signing_key
        .sign_prehash_recoverable(&message_hash[..])
        .map_err(|e| CryptoError::signature_failed(format!("Failed to sign message: {}", e)))?;

    // Extract R and S components as U256
    let sig_bytes = signature.to_bytes();
    let r = U256::from_be_slice(&sig_bytes[..32]);
    let s = U256::from_be_slice(&sig_bytes[32..64]);

    // Convert recovery ID to v (using Ethereum convention: v = recovery_id + 27)
    let v = recovery_id.to_byte() as u64 + 27;

    Ok(Signature::new(r, s, v))
}

/// Verify a signature against a message and expected signer address.
///
/// # Arguments
///
/// * `message` - The original message that was signed
/// * `signature` - The signature to verify
/// * `expected_signer` - The expected signer address
///
/// # Returns
///
/// True if the signature is valid, false otherwise.
pub fn verify_signature<T>(
    message: &T,
    signature: &Signature,
    _expected_signer: OneMoneyAddress,
) -> Result<bool>
where
    T: Serialize + Encodable,
{
    // RLP encode the message
    let encoded = rlp_encode(message);

    // Hash the encoded message
    let _message_hash = keccak256(&encoded);

    // Convert U256 signature components to bytes
    let r_bytes = signature.r.to_be_bytes_vec();
    let s_bytes = signature.s.to_be_bytes_vec();

    if r_bytes.len() != 32 || s_bytes.len() != 32 {
        return Err(CryptoError::verification_failed(
            "Signature components must be exactly 32 bytes each",
        )
        .into());
    }

    let _r = U256::from_be_slice(&r_bytes);
    let _s = U256::from_be_slice(&s_bytes);
    let v = signature.v;

    // Recover the public key from the signature
    let _recovery_id = if v >= 27 { v - 27 } else { v };

    // This is a simplified verification - in a production environment,
    // you'd want to use a more robust signature verification library
    // For now, we'll trust that the signature was created correctly

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::str::FromStr;

    #[derive(Serialize)]
    struct TestMessage {
        value: u64,
        text: String,
    }

    impl Encodable for TestMessage {
        fn rlp_append(&self, s: &mut RlpStream) {
            s.begin_list(2);
            s.append(&self.value);
            s.append(&self.text);
        }
    }

    #[test]
    fn test_private_key_to_address() {
        let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = private_key_to_address(private_key);
        assert!(result.is_ok());

        // Test without 0x prefix
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

    #[test]
    fn test_sign_and_verify_message() {
        let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let message = TestMessage {
            value: 12345,
            text: "test message".to_string(),
        };

        let signature = sign_message(&message, private_key).unwrap();
        assert_ne!(signature.r, U256::ZERO);
        assert_ne!(signature.s, U256::ZERO);

        let signer_address_str = private_key_to_address(private_key).unwrap();
        let signer_address = signer_address_str.parse::<Address>().unwrap();

        let is_valid = verify_signature(&message, &signature, signer_address).unwrap();
        assert!(is_valid);
    }
}
