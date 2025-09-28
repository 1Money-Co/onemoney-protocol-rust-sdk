//! Comprehensive cryptographic integration tests
//!
//! This file contains end-to-end cryptographic integration tests including:
//! - Complete signature generation and verification chains
//! - Cross-module cryptographic operations
//! - Key derivation and address generation
//! - Transaction signing workflows

use alloy_primitives::{Address, U256};
use onemoney_protocol::Signable;
use onemoney_protocol::TokenMintPayload;
use onemoney_protocol::crypto::{private_key_to_address, sign_transaction_payload};
use std::error::Error;
use std::str::FromStr;

//
// ============================================================================
// ADDRESS DERIVATION INTEGRATION TESTS
// ============================================================================
//

#[test]
fn test_address_derivation_integration() -> Result<(), Box<dyn Error>> {
    // Test with known private key
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    let address_str = private_key_to_address(private_key)?;

    // Address should be valid format
    assert!(address_str.starts_with("0x"));
    assert_eq!(address_str.len(), 42);

    // Same private key should always produce same address
    let address_str2 = private_key_to_address(private_key)?;
    assert_eq!(address_str, address_str2);

    // Test that address can be parsed by alloy_primitives
    let _parsed_address = Address::from_str(&address_str)?;

    println!("Derived address: {}", address_str);
    Ok(())
}

#[test]
fn test_multiple_private_keys_produce_unique_addresses() -> Result<(), Box<dyn Error>> {
    let private_keys = [
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    ];

    let mut addresses = Vec::new();

    // Generate addresses from all private keys
    for private_key in &private_keys {
        let address = private_key_to_address(private_key)?;
        addresses.push(address);
    }

    // Verify all addresses are unique
    for i in 0..addresses.len() {
        for j in (i + 1)..addresses.len() {
            assert_ne!(
                addresses[i], addresses[j],
                "Private keys {} and {} produced the same address: {}",
                private_keys[i], private_keys[j], addresses[i]
            );
        }
    }

    println!(
        "Generated {} unique addresses from {} private keys",
        addresses.len(),
        private_keys.len()
    );
    Ok(())
}

#[test]
fn test_address_derivation_error_cases() {
    let invalid_keys = [
        "",                                                                    // Empty
        "123",                                                                 // Too short
        "xyz",                                                                 // Invalid characters
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef123", // Too long
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0", // Too short (63 chars)
        "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",  // Invalid hex chars
    ];

    for invalid_key in &invalid_keys {
        let result = private_key_to_address(invalid_key);
        assert!(
            result.is_err(),
            "Invalid private key '{}' should produce an error, but got: {:?}",
            invalid_key,
            result
        );
    }
}

//
// ============================================================================
// TRANSACTION SIGNING INTEGRATION TESTS
// ============================================================================
//

#[test]
fn test_token_mint_payload_signing_integration() -> Result<(), Box<dyn Error>> {
    // Create a test token mint payload
    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    let to_address = Address::from_str("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd")?;
    let amount = U256::from(1000000000000000000u64); // 1 token with 18 decimals

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    // Test that payload implements Signable trait
    let hash = payload.signature_hash();
    assert_eq!(hash.len(), 32); // keccak256 produces 32 bytes

    // Test that hash is deterministic
    let hash2 = payload.signature_hash();
    assert_eq!(hash, hash2);

    // Test signing with different payloads produces different hashes
    let payload2 = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 2,
        token: token_address,
        recipient: to_address,
        value: U256::from(2000000000000000000u64), // Different amount
    };

    let hash3 = payload2.signature_hash();
    assert_ne!(hash, hash3);

    println!("Token mint payload hash: {:?}", hash);
    Ok(())
}

#[test]
fn test_end_to_end_transaction_signing() -> Result<(), Box<dyn Error>> {
    // Test complete workflow: create payload -> sign -> verify
    let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Create payload
    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    let to_address = Address::from_str("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd")?;
    let amount = U256::from(1000000000000000000u64);

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    // Sign the payload
    let signature = sign_transaction_payload(&payload, private_key)?;

    // Verify signature has correct structure
    assert_ne!(signature.r, U256::ZERO);
    assert_ne!(signature.s, U256::ZERO);
    assert!(signature.v == 27 || signature.v == 28 || signature.v == 0 || signature.v == 1);

    // Test that same payload with same key produces same signature
    let signature2 = sign_transaction_payload(&payload, private_key)?;
    assert_eq!(signature.r, signature2.r);
    assert_eq!(signature.s, signature2.s);
    assert_eq!(signature.v, signature2.v);

    // Test that different payload produces different signature
    let payload2 = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 2,
        token: token_address,
        recipient: to_address,
        value: U256::from(2000000000000000000u64), // Different amount
    };

    let signature3 = sign_transaction_payload(&payload2, private_key)?;
    assert!(signature.r != signature3.r || signature.s != signature3.s);

    println!(
        "Transaction signature: r={}, s={}, v={}",
        signature.r, signature.s, signature.v
    );
    Ok(())
}

#[test]
fn test_signature_determinism_across_restarts() -> Result<(), Box<dyn Error>> {
    // Test that signatures are deterministic across multiple runs
    let private_key = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

    let token_address = Address::from_str("0x9999999999999999999999999999999999999999")?;
    let to_address = Address::from_str("0x8888888888888888888888888888888888888888")?;
    let amount = U256::from(5000000000000000000u64);

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    // Generate multiple signatures to ensure determinism
    let signatures: Vec<_> = (0..5)
        .map(|_| sign_transaction_payload(&payload, private_key))
        .collect::<Result<Vec<_>, _>>()?;

    // All signatures should be identical
    for i in 1..signatures.len() {
        assert_eq!(signatures[0].r, signatures[i].r);
        assert_eq!(signatures[0].s, signatures[i].s);
        assert_eq!(signatures[0].v, signatures[i].v);
    }

    Ok(())
}

//
// ============================================================================
// CRYPTOGRAPHIC WORKFLOW INTEGRATION TESTS
// ============================================================================
//

#[test]
fn test_complete_key_to_signature_workflow() -> Result<(), Box<dyn Error>> {
    // Test complete workflow from private key to signature
    let private_key = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";

    // Step 1: Derive address from private key
    let address_str = private_key_to_address(private_key)?;
    let derived_address = Address::from_str(&address_str)?;

    // Step 2: Create transaction payload involving the derived address
    let token_address = Address::from_str("0x1111111111111111111111111111111111111111")?;
    let amount = U256::from(1500000000000000000u64);

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 3,
        token: token_address,
        recipient: derived_address, // Using the derived address as recipient
        value: amount,
    };

    // Step 3: Sign the payload with the private key
    let signature = sign_transaction_payload(&payload, private_key)?;

    // Step 4: Verify all components are valid
    assert!(address_str.starts_with("0x"));
    assert_eq!(address_str.len(), 42);
    assert_ne!(signature.r, U256::ZERO);
    assert_ne!(signature.s, U256::ZERO);

    // Step 5: Verify hash consistency
    let payload_hash = payload.signature_hash();
    assert_eq!(payload_hash.len(), 32);

    println!(
        "Complete workflow: key={} -> address={} -> signature=(r={}, s={}, v={})",
        &private_key[..8], // Show first 8 chars for privacy
        address_str,
        signature.r,
        signature.s,
        signature.v
    );

    Ok(())
}

#[test]
fn test_multiple_transaction_types_signing() -> Result<(), Box<dyn Error>> {
    // Test signing different types of payloads with the same key
    let private_key = "5555555555555555555555555555555555555555555555555555555555555555";

    // Create multiple different payloads
    let token1 = Address::from_str("0x1111111111111111111111111111111111111111")?;
    let token2 = Address::from_str("0x2222222222222222222222222222222222222222")?;
    let recipient1 = Address::from_str("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")?;
    let recipient2 = Address::from_str("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")?;

    let payloads = [
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token1,
            recipient: recipient1,
            value: U256::from(1000u64),
        },
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token1,
            recipient: recipient2,
            value: U256::from(2000u64),
        },
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token2,
            recipient: recipient1,
            value: U256::from(3000u64),
        },
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token2,
            recipient: recipient2,
            value: U256::from(4000u64),
        },
    ];

    let mut signatures = Vec::new();
    let mut hashes = Vec::new();

    // Sign all payloads
    for payload in &payloads {
        let signature = sign_transaction_payload(payload, private_key)?;
        let hash = payload.signature_hash();

        signatures.push(signature);
        hashes.push(hash);
    }

    // Verify all signatures are different (different payloads)
    for i in 0..signatures.len() {
        for j in (i + 1)..signatures.len() {
            assert!(
                signatures[i].r != signatures[j].r || signatures[i].s != signatures[j].s,
                "Signatures {} and {} should be different but are the same",
                i,
                j
            );
        }
    }

    // Verify all hashes are different
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Hashes {} and {} should be different but are the same",
                i, j
            );
        }
    }

    println!("Successfully signed {} different payloads", payloads.len());
    Ok(())
}

//
// ============================================================================
// CRYPTOGRAPHIC EDGE CASES AND ERROR HANDLING
// ============================================================================
//

#[test]
fn test_signing_with_invalid_private_keys() {
    let invalid_keys = [
        "",                                                                    // Empty
        "123",                                                                 // Too short
        "xyz",                                                                 // Invalid characters
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef123", // Too long
    ];

    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();
    let to_address = Address::from_str("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd").unwrap();
    let amount = U256::from(1000u64);

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    for invalid_key in &invalid_keys {
        let result = sign_transaction_payload(&payload, invalid_key);
        assert!(
            result.is_err(),
            "Invalid private key '{}' should produce an error, but signing succeeded",
            invalid_key
        );
    }
}

#[test]
fn test_extreme_value_handling() -> Result<(), Box<dyn Error>> {
    // Test signing with extreme values
    let private_key = "7777777777777777777777777777777777777777777777777777777777777777";

    let zero_address = Address::ZERO;
    let max_address = Address::from([0xFF; 20]);

    let payloads = [
        // Zero amount
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: zero_address,
            recipient: zero_address,
            value: U256::ZERO,
        },
        // Maximum amount
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: max_address,
            recipient: max_address,
            value: U256::MAX,
        },
        // Mixed extremes
        TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: zero_address,
            recipient: max_address,
            value: U256::from(1u64),
        },
    ];

    for (i, payload) in payloads.iter().enumerate() {
        let signature = sign_transaction_payload(payload, private_key)?;

        // Verify signature is valid structure
        assert_ne!(
            signature.r,
            U256::ZERO,
            "Signature {} has zero r component",
            i
        );
        assert_ne!(
            signature.s,
            U256::ZERO,
            "Signature {} has zero s component",
            i
        );

        // Verify hash is generated
        let hash = payload.signature_hash();
        assert_eq!(hash.len(), 32, "Payload {} hash should be 32 bytes", i);
    }

    Ok(())
}

#[test]
fn test_concurrent_signing_consistency() -> Result<(), Box<dyn Error>> {
    // Test that concurrent signing operations produce consistent results
    use std::sync::{Arc, Mutex};
    use std::thread;

    let private_key = "8888888888888888888888888888888888888888888888888888888888888888";
    let token_address = Address::from_str("0x3333333333333333333333333333333333333333")?;
    let to_address = Address::from_str("0x4444444444444444444444444444444444444444")?;
    let amount = U256::from(7777777777777777777u64);

    let payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    let signatures = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Spawn multiple threads to sign the same payload
    for _ in 0..5 {
        let payload_clone = payload.clone();
        let signatures_clone = Arc::clone(&signatures);
        let private_key_str = private_key.to_string();

        let handle = thread::spawn(move || {
            let signature = sign_transaction_payload(&payload_clone, &private_key_str).unwrap();
            signatures_clone.lock().unwrap().push(signature);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let signatures = signatures.lock().unwrap();
    assert_eq!(signatures.len(), 5);

    // All signatures should be identical
    for i in 1..signatures.len() {
        assert_eq!(signatures[0].r, signatures[i].r);
        assert_eq!(signatures[0].s, signatures[i].s);
        assert_eq!(signatures[0].v, signatures[i].v);
    }

    Ok(())
}

//
// ============================================================================
// PERFORMANCE AND SCALABILITY TESTS
// ============================================================================
//

#[test]
fn test_signing_performance_baseline() -> Result<(), Box<dyn Error>> {
    // Test that signing operations complete within reasonable time
    use std::time::Instant;

    let private_key = "9999999999999999999999999999999999999999999999999999999999999999";
    let token_address = Address::from_str("0x5555555555555555555555555555555555555555")?;
    let to_address = Address::from_str("0x6666666666666666666666666666666666666666")?;
    let amount = U256::from(12345678901234567890u64);

    let _payload = TokenMintPayload {
        recent_checkpoint: 200,
        chain_id: 1,
        nonce: 1,
        token: token_address,
        recipient: to_address,
        value: amount,
    };

    let iterations = 100;
    let start = Instant::now();

    // Perform multiple signing operations
    for i in 0..iterations {
        let test_payload = TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token_address,
            recipient: to_address,
            value: amount + U256::from(i), // Slightly different each time
        };

        let _signature = sign_transaction_payload(&test_payload, private_key)?;
    }

    let duration = start.elapsed();
    let avg_time_per_signature = duration / iterations;

    println!(
        "Signed {} transactions in {:?} (avg: {:?} per signature)",
        iterations, duration, avg_time_per_signature
    );

    // Sanity check: each signature should complete in reasonable time
    assert!(
        avg_time_per_signature.as_millis() < 100,
        "Average signing time {} ms is too slow",
        avg_time_per_signature.as_millis()
    );

    Ok(())
}
