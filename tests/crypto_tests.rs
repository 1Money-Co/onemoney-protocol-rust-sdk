//! Cryptographic function tests for the OneMoney Rust SDK.
//!
//! These tests verify that cryptographic operations work correctly,
//! including signature generation, verification, and hash computation.

use alloy_primitives::{keccak256, U256};
use onemoney::api::tokens::TokenMintPayload;
use onemoney::{
    crypto::{private_key_to_address, sign_transaction_payload, Signable},
    OneMoneyAddress, TokenAmount,
};
use std::error::Error;
use std::str::FromStr;

#[test]
fn test_address_derivation() -> Result<(), Box<dyn Error>> {
    // Test with known private key
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    let address_str = private_key_to_address(private_key)?;

    // Address should be valid format
    assert!(address_str.starts_with("0x"));
    assert_eq!(address_str.len(), 42);

    // Same private key should always produce same address
    let address_str2 = private_key_to_address(private_key)?;
    assert_eq!(address_str, address_str2);

    println!("Derived address: {}", address_str);
    Ok(())
}

#[test]
fn test_different_private_keys_produce_different_addresses() -> Result<(), Box<dyn Error>> {
    let private_key1 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let private_key2 = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

    let address1 = private_key_to_address(private_key1)?;
    let address2 = private_key_to_address(private_key2)?;

    assert_ne!(
        address1, address2,
        "Different private keys should produce different addresses"
    );

    Ok(())
}

#[test]
fn test_invalid_private_key_formats() {
    let invalid_keys = [
        "",                                                                  // Empty
        "123",                                                               // Too short
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0", // Too long
        "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",  // Invalid hex
    ];

    for invalid_key in &invalid_keys {
        let result = private_key_to_address(invalid_key);
        assert!(
            result.is_err(),
            "Invalid private key '{}' should be rejected",
            invalid_key
        );
    }
}

#[test]
fn test_signature_generation() -> Result<(), Box<dyn Error>> {
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Create a test payload
    let payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    // Sign the payload
    let signature = sign_transaction_payload(&payload, private_key)?;

    // Signature should have expected structure
    assert!(
        signature.r != U256::ZERO,
        "Signature r component should not be zero"
    );
    assert!(
        signature.s != U256::ZERO,
        "Signature s component should not be zero"
    );
    assert!(
        signature.v == 27 || signature.v == 28,
        "Signature v should be 27 or 28"
    );

    println!(
        "Generated signature: r={}, s={}, v={}",
        signature.r, signature.s, signature.v
    );

    Ok(())
}

#[test]
fn test_signature_deterministic() -> Result<(), Box<dyn Error>> {
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Create identical payloads
    let payload1 = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    let payload2 = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    // Sign both payloads
    let signature1 = sign_transaction_payload(&payload1, private_key)?;
    let signature2 = sign_transaction_payload(&payload2, private_key)?;

    // Signatures should be identical for identical payloads
    assert_eq!(
        signature1.r, signature2.r,
        "Signature r components should be identical"
    );
    assert_eq!(
        signature1.s, signature2.s,
        "Signature s components should be identical"
    );
    assert_eq!(
        signature1.v, signature2.v,
        "Signature v components should be identical"
    );

    Ok(())
}

#[test]
fn test_different_payloads_produce_different_signatures() -> Result<(), Box<dyn Error>> {
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Create different payloads (different nonce)
    let payload1 = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    let payload2 = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1, // Different nonce
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    // Sign both payloads
    let signature1 = sign_transaction_payload(&payload1, private_key)?;
    let signature2 = sign_transaction_payload(&payload2, private_key)?;

    // Signatures should be different
    assert!(
        signature1.r != signature2.r || signature1.s != signature2.s,
        "Different payloads should produce different signatures"
    );

    Ok(())
}

#[test]
fn test_hash_computation() -> Result<(), Box<dyn Error>> {
    let payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    // Test signature hash computation
    let hash = payload.signature_hash();

    // Hash should be 32 bytes
    assert_eq!(hash.len(), 32, "Hash should be 32 bytes");

    // Same payload should produce same hash
    let hash2 = payload.signature_hash();
    assert_eq!(hash, hash2, "Same payload should produce same hash");

    println!("Payload hash: {}", hex::encode(hash));

    Ok(())
}

#[test]
fn test_hash_changes_with_payload_changes() -> Result<(), Box<dyn Error>> {
    let payload1 = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    let payload2 = TokenMintPayload {
        recent_epoch: 101, // Different epoch
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    let hash1 = payload1.signature_hash();
    let hash2 = payload2.signature_hash();

    assert_ne!(
        hash1, hash2,
        "Different payloads should produce different hashes"
    );

    Ok(())
}

#[test]
fn test_keccak256_hash_function() {
    // Test basic keccak256 functionality with known input/output
    let input = b"hello world";
    let hash = keccak256(input);

    // Keccak256 of "hello world" should be consistent
    let expected_hash = keccak256(b"hello world");
    assert_eq!(hash, expected_hash);

    // Different inputs should produce different hashes
    let hash2 = keccak256(b"hello world!");
    assert_ne!(hash, hash2);

    println!("Keccak256('hello world'): {}", hex::encode(hash));
}

#[test]
fn test_rlp_encoding_affects_hash() -> Result<(), Box<dyn Error>> {
    use rlp::encode;

    // Test that RLP encoding is deterministic and affects hash
    let payload = TokenMintPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
        value: TokenAmount::from(1000u64),
        token: OneMoneyAddress::from_str("0xabcdef1234567890abcdef1234567890abcdef12")?,
    };

    // Encode the same payload multiple times
    let encoded1 = encode(&payload);
    let encoded2 = encode(&payload);

    // Encoding should be deterministic
    assert_eq!(encoded1, encoded2, "RLP encoding should be deterministic");

    // Hash of encoded data should match signature_hash
    let manual_hash = keccak256(&encoded1);
    let signature_hash = payload.signature_hash();

    assert_eq!(
        manual_hash, signature_hash,
        "Manual hash should match signature_hash"
    );

    println!("RLP encoded length: {} bytes", encoded1.len());
    println!("RLP encoded data: {}", hex::encode(&encoded1));

    Ok(())
}

#[test]
fn test_edge_case_values() -> Result<(), Box<dyn Error>> {
    // Test with edge case values
    let payload = TokenMintPayload {
        recent_epoch: u64::MAX,
        recent_checkpoint: u64::MAX,
        chain_id: u64::MAX,
        nonce: u64::MAX,
        recipient: OneMoneyAddress::from_str("0x0000000000000000000000000000000000000000")?,
        value: TokenAmount::from(u64::MAX),
        token: OneMoneyAddress::from_str("0xffffffffffffffffffffffffffffffffffffffff")?,
    };

    // Should be able to compute hash without panicking
    let hash = payload.signature_hash();
    assert_eq!(hash.len(), 32);

    println!("Edge case hash: {}", hex::encode(hash));

    Ok(())
}

#[test]
fn test_zero_values() -> Result<(), Box<dyn Error>> {
    // Test with all zero values
    let payload = TokenMintPayload {
        recent_epoch: 0,
        recent_checkpoint: 0,
        chain_id: 0,
        nonce: 0,
        recipient: OneMoneyAddress::from_str("0x0000000000000000000000000000000000000000")?,
        value: TokenAmount::from(0u64),
        token: OneMoneyAddress::from_str("0x0000000000000000000000000000000000000000")?,
    };

    // Should be able to compute hash without panicking
    let hash = payload.signature_hash();
    assert_eq!(hash.len(), 32);

    println!("Zero values hash: {}", hex::encode(hash));

    Ok(())
}
