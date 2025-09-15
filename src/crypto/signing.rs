//! Digital signature operations.

use super::hashing::Signable;
use crate::{CryptoError, Result, Signature};
use alloy_primitives::B256;
use hex::decode as hex_decode;
use k256::ecdsa::SigningKey;

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
    use alloy::signers::{SignerSync, local::LocalSigner};

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

    let local_signer = LocalSigner::from(signing_key);

    // Sign the hash using LocalSigner (matching wallet implementation)
    let alloy_signature = local_signer.sign_hash_sync(message_hash).map_err(|e| {
        CryptoError::signature_failed(format!("Failed to sign hash with LocalSigner: {}", e))
    })?;

    // Extract R, S, and V from alloy signature
    let r = alloy_signature.r();
    let s = alloy_signature.s();
    // L1 expects: v=0 (false/even parity) or v=1 (true/odd parity)
    let v = if alloy_signature.v() { 1 } else { 0 };

    let our_signature = Signature::new(r, s, v);

    Ok(our_signature)
}
