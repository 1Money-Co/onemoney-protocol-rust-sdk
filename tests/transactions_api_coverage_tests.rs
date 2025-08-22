//! Comprehensive transactions API coverage tests

use alloy_primitives::{Address, U256};
use onemoney_protocol::client::config::api_path;
use onemoney_protocol::client::config::endpoints::transactions::*;
use onemoney_protocol::client::http::Client;
use onemoney_protocol::*;
use std::str::FromStr;

#[test]
fn test_transaction_endpoints_constants() {
    // Test all endpoint constants are correct (without version prefix)
    assert_eq!(PAYMENT, "/transactions/payment");
    assert_eq!(BY_HASH, "/transactions/by_hash");
    assert_eq!(ESTIMATE_FEE, "/transactions/estimate_fee");
    assert_eq!(RECEIPT_BY_HASH, "/transactions/receipt/by_hash");
}

#[test]
fn test_transaction_api_path_construction() {
    // Test path construction for all transaction endpoints (with version prefix)
    let payment_path = api_path(PAYMENT);
    assert_eq!(payment_path, "/v1/transactions/payment");

    let by_hash_path = api_path(BY_HASH);
    assert_eq!(by_hash_path, "/v1/transactions/by_hash");

    let estimate_path = api_path(ESTIMATE_FEE);
    assert_eq!(estimate_path, "/v1/transactions/estimate_fee");

    let receipt_path = api_path(RECEIPT_BY_HASH);
    assert_eq!(receipt_path, "/v1/transactions/receipt/by_hash");
}

#[test]
fn test_transaction_by_hash_path_construction() {
    let hash = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    let path = api_path(&format!("{}?hash={}", BY_HASH, hash));

    assert!(path.contains("/v1/transactions/by_hash"));
    assert!(
        path.contains("hash=0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")
    );
    assert!(path.contains("?"));
}

#[test]
fn test_transaction_receipt_by_hash_path_construction() {
    let hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let path = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, hash));

    assert!(path.contains("/v1/transactions/receipt/by_hash"));
    assert!(
        path.contains("hash=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
    );
    assert!(path.contains("?"));
}

#[test]
fn test_fee_estimate_path_construction_comprehensive() {
    // Test fee estimate request with token
    let request_with_token = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "1000000000000000000".to_string(),
        token: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };

    // Simulate path construction logic from estimate_fee method
    let mut path = ESTIMATE_FEE.to_string();
    let mut query_params = Vec::new();

    query_params.push(format!("from={}", request_with_token.from));
    query_params.push(format!("value={}", request_with_token.value));
    if let Some(token) = &request_with_token.token {
        query_params.push(format!("token={}", token));
    }

    if !query_params.is_empty() {
        path.push('?');
        path.push_str(&query_params.join("&"));
    }

    assert!(path.contains("from=0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
    assert!(path.contains("value=1000000000000000000"));
    assert!(path.contains("token=0x1234567890abcdef1234567890abcdef12345678"));
    assert!(path.contains("&"));
    assert!(path.contains("?"));

    let full_path = api_path(&path);
    assert!(full_path.contains("/v1/transactions/estimate_fee"));
}

#[test]
fn test_fee_estimate_path_construction_without_token() {
    // Test fee estimate request without token
    let request_without_token = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "500000000000000000".to_string(),
        token: None,
    };

    // Simulate path construction logic from estimate_fee method
    let mut path = ESTIMATE_FEE.to_string();
    let mut query_params = Vec::new();

    query_params.push(format!("from={}", request_without_token.from));
    query_params.push(format!("value={}", request_without_token.value));
    if let Some(token) = request_without_token.token {
        query_params.push(format!("token={}", token));
    }

    if !query_params.is_empty() {
        path.push('?');
        path.push_str(&query_params.join("&"));
    }

    assert!(path.contains("from=0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
    assert!(path.contains("value=500000000000000000"));
    assert!(!path.contains("token="));
    assert!(path.contains("?"));

    let full_path = api_path(&path);
    assert!(full_path.contains("/v1/transactions/estimate_fee"));
}

#[test]
fn test_payment_payload_comprehensive() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = PaymentPayload {
        recent_epoch: 123,
        recent_checkpoint: 456,
        chain_id: 1212101,
        nonce: 0,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    // Test all fields
    assert_eq!(payload.recent_epoch, 123);
    assert_eq!(payload.recent_checkpoint, 456);
    assert_eq!(payload.chain_id, 1212101);
    assert_eq!(payload.nonce, 0);
    assert_eq!(payload.recipient, recipient);
    assert_eq!(payload.value, U256::from(1000000000000000000u64));
    assert_eq!(payload.token, token);

    // Test serialization/deserialization
    let json = serde_json::to_string(&payload).expect("Test data should be valid");
    let deserialized: PaymentPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
    assert_eq!(payload.recent_checkpoint, deserialized.recent_checkpoint);
    assert_eq!(payload.chain_id, deserialized.chain_id);
    assert_eq!(payload.nonce, deserialized.nonce);
    assert_eq!(payload.recipient, deserialized.recipient);
    assert_eq!(payload.value, deserialized.value);
    assert_eq!(payload.token, deserialized.token);

    // Test signature hash calculation
    let hash = payload.signature_hash();
    assert_ne!(hash, alloy_primitives::B256::default());

    // Test deterministic hash
    let hash2 = payload.signature_hash();
    assert_eq!(hash, hash2);

    // Test RLP encoding
    let encoded = rlp::encode(&payload);
    assert!(!encoded.is_empty());

    // Test display implementation
    let display_str = format!("{}", payload);
    assert!(display_str.contains("Payment to"));
    assert!(display_str.contains("0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0"));
    assert!(display_str.contains("1000000000000000000"));
    assert!(display_str.contains("token 0x1234567890AbcdEF1234567890aBcdef12345678"));
}

#[test]
fn test_payment_payload_different_values() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test with different values to ensure hash changes
    let payload1 = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let payload2 = PaymentPayload {
        recent_epoch: 101, // Different epoch
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let payload3 = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 201, // Different checkpoint
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let payload4 = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 6, // Different nonce
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let payload5 = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(2000000000000000000u64), // Different value
        token,
    };

    // All hashes should be different
    let hash1 = payload1.signature_hash();
    let hash2 = payload2.signature_hash();
    let hash3 = payload3.signature_hash();
    let hash4 = payload4.signature_hash();
    let hash5 = payload5.signature_hash();

    assert_ne!(hash1, hash2);
    assert_ne!(hash1, hash3);
    assert_ne!(hash1, hash4);
    assert_ne!(hash1, hash5);
    assert_ne!(hash2, hash3);
    assert_ne!(hash2, hash4);
    assert_ne!(hash2, hash5);
    assert_ne!(hash3, hash4);
    assert_ne!(hash3, hash5);
    assert_ne!(hash4, hash5);

    // RLP encodings should also be different
    assert_ne!(rlp::encode(&payload1), rlp::encode(&payload2));
    assert_ne!(rlp::encode(&payload1), rlp::encode(&payload3));
    assert_ne!(rlp::encode(&payload1), rlp::encode(&payload4));
    assert_ne!(rlp::encode(&payload1), rlp::encode(&payload5));
}

#[test]
fn test_payment_request_structure() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let signature = Signature {
        r: U256::from(12345u64),
        s: U256::from(67890u64),
        v: 27,
    };

    let request = PaymentRequest { payload, signature };

    // Test serialization (PaymentRequest only derives Serialize, not Deserialize)
    let json = serde_json::to_string(&request).expect("Test data should be valid");
    assert!(json.contains("recent_epoch"));
    assert!(json.contains("100"));
    assert!(json.contains("signature"));
    assert!(json.contains("12345"));
    assert!(json.contains("67890"));
    assert!(json.contains("27"));
    assert!(json.contains("recipient"));
    // Address format in JSON is lowercase
    assert!(json.contains("0x742d35cc6634c0532925a3b8d91d6f4a81b8cbc0"));
}

#[test]
fn test_fee_estimate_request_comprehensive() {
    // Test with token
    let request_with_token = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "1000000000000000000".to_string(),
        token: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };

    // Test serialization (FeeEstimateRequest only derives Serialize, not Deserialize)
    let json = serde_json::to_string(&request_with_token).expect("Test data should be valid");
    assert!(json.contains("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
    assert!(json.contains("1000000000000000000"));
    assert!(json.contains("0x1234567890abcdef1234567890abcdef12345678"));
    assert!(json.contains("from"));
    assert!(json.contains("value"));
    assert!(json.contains("token"));

    // Test without token
    let request_without_token = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "500000000000000000".to_string(),
        token: None,
    };

    let json2 = serde_json::to_string(&request_without_token).expect("Test data should be valid");
    assert!(json2.contains("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
    assert!(json2.contains("500000000000000000"));
    assert!(!json2.contains("0x1234567890abcdef1234567890abcdef12345678"));
    assert!(json2.contains("from"));
    assert!(json2.contains("value"));

    // Test edge cases
    let edge_cases = [
        // Very small value
        FeeEstimateRequest {
            from: "0x0000000000000000000000000000000000000001".to_string(),
            value: "1".to_string(),
            token: None,
        },
        // Very large value
        FeeEstimateRequest {
            from: "0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string(),
            value: "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                .to_string(),
            token: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        },
        // Zero value
        FeeEstimateRequest {
            from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
            value: "0".to_string(),
            token: None,
        },
    ];

    for request in edge_cases {
        let json = serde_json::to_string(&request).expect("Test data should be valid");
        assert!(json.contains(&request.from));
        assert!(json.contains(&request.value));
        if let Some(token) = &request.token {
            assert!(json.contains(token));
        }
    }
}

#[test]
fn test_payment_payload_edge_cases() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test with zero values
    let zero_payload = PaymentPayload {
        recent_epoch: 0,
        recent_checkpoint: 0,
        chain_id: 1212101,
        nonce: 0,
        recipient,
        value: U256::ZERO,
        token,
    };

    let hash_zero = zero_payload.signature_hash();
    assert_ne!(hash_zero, alloy_primitives::B256::default());

    // Test with maximum values
    let max_payload = PaymentPayload {
        recent_epoch: u64::MAX,
        recent_checkpoint: u64::MAX,
        chain_id: 1212101,
        nonce: u64::MAX,
        recipient,
        value: U256::MAX,
        token,
    };

    let hash_max = max_payload.signature_hash();
    assert_ne!(hash_max, alloy_primitives::B256::default());
    assert_ne!(hash_zero, hash_max);

    // Test serialization of edge cases
    let json_zero = serde_json::to_string(&zero_payload).expect("Test data should be valid");
    let json_max = serde_json::to_string(&max_payload).expect("Test data should be valid");

    assert!(json_zero.contains("\"0\""));
    assert!(json_max.contains(&u64::MAX.to_string()));

    // Test RLP encoding of edge cases
    let encoded_zero = rlp::encode(&zero_payload);
    let encoded_max = rlp::encode(&max_payload);

    assert!(!encoded_zero.is_empty());
    assert!(!encoded_max.is_empty());
    assert_ne!(encoded_zero, encoded_max);
}

#[test]
fn test_signature_structure_comprehensive() {
    // Test various signature values
    let signatures = [
        Signature {
            r: U256::from(1u64),
            s: U256::from(2u64),
            v: 27,
        },
        Signature {
            r: U256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .expect("Test data should be valid"),
            s: U256::from_str("0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321")
                .expect("Test data should be valid"),
            v: 28,
        },
        Signature {
            r: U256::MAX,
            s: U256::MAX,
            v: 28,
        },
        Signature::default(),
    ];

    for signature in signatures {
        // Test serialization/deserialization
        let json = serde_json::to_string(&signature).expect("Test data should be valid");
        let deserialized: Signature =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(signature.r, deserialized.r);
        assert_eq!(signature.s, deserialized.s);
        assert_eq!(signature.v, deserialized.v);

        // Test clone and debug
        let cloned = signature.clone();
        assert_eq!(signature.r, cloned.r);
        assert_eq!(signature.s, cloned.s);
        assert_eq!(signature.v, cloned.v);

        let debug_str = format!("{:?}", signature);
        assert!(debug_str.contains("Signature"));
    }
}

#[test]
fn test_client_creation_for_transaction_operations()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    // Test that clients can be created for transaction operations
    let mainnet_client = Client::mainnet()?;
    let testnet_client = Client::testnet()?;
    let local_client = Client::local()?;

    // Verify they're client instances (via debug output)
    let mainnet_debug = format!("{:?}", mainnet_client);
    let testnet_debug = format!("{:?}", testnet_client);
    let local_debug = format!("{:?}", local_client);

    assert!(mainnet_debug.contains("Client"));
    assert!(testnet_debug.contains("Client"));
    assert!(local_debug.contains("Client"));

    Ok(())
}

#[test]
fn test_query_parameter_handling() {
    // Test empty query parameters
    let empty_params: Vec<String> = Vec::new();
    let mut path = "/test".to_string();

    if !empty_params.is_empty() {
        path.push('?');
        path.push_str(&empty_params.join("&"));
    }

    assert_eq!(path, "/test");

    // Test single query parameter
    let single_param = ["param1=value1".to_string()];
    let mut path2 = "/test".to_string();

    if !single_param.is_empty() {
        path2.push('?');
        path2.push_str(&single_param.join("&"));
    }

    assert_eq!(path2, "/test?param1=value1");

    // Test multiple query parameters
    let multiple_params = [
        "from=0x123".to_string(),
        "value=1000".to_string(),
        "token=0x456".to_string(),
    ];
    let mut path3 = "/test".to_string();

    if !multiple_params.is_empty() {
        path3.push('?');
        path3.push_str(&multiple_params.join("&"));
    }

    assert_eq!(path3, "/test?from=0x123&value=1000&token=0x456");
}

#[test]
fn test_hash_string_formats() {
    // Test various hash string formats
    let hash_formats = [
        "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "0x0000000000000000000000000000000000000000000000000000000000000001",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ];

    for hash in hash_formats {
        // Test transaction by hash path (without version prefix in raw format)
        let by_hash_path = format!("{}?hash={}", BY_HASH, hash);
        assert!(by_hash_path.contains("/transactions/by_hash"));
        assert!(by_hash_path.contains(&format!("hash={}", hash)));

        // Test receipt by hash path (without version prefix in raw format)
        let receipt_path = format!("{}?hash={}", RECEIPT_BY_HASH, hash);
        assert!(receipt_path.contains("/transactions/receipt/by_hash"));
        assert!(receipt_path.contains(&format!("hash={}", hash)));

        // Test with api_path function (with version prefix)
        let by_hash_full_path = api_path(&by_hash_path);
        assert!(by_hash_full_path.contains("/v1/transactions/by_hash"));

        let receipt_full_path = api_path(&receipt_path);
        assert!(receipt_full_path.contains("/v1/transactions/receipt/by_hash"));
    }
}

#[test]
fn test_rlp_encoding_comprehensive() {
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    let payload = PaymentPayload {
        recent_epoch: 123,
        recent_checkpoint: 456,
        chain_id: 1212101,
        nonce: 0,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    // Test RLP encoding produces consistent results
    let encoded1 = rlp::encode(&payload);
    let encoded2 = rlp::encode(&payload);
    assert_eq!(encoded1, encoded2);

    // Test RLP encoding is not empty
    assert!(!encoded1.is_empty());

    // Test RLP encoding with different values produces different results
    let payload2 = PaymentPayload {
        recent_epoch: 124, // Different epoch
        recent_checkpoint: 456,
        chain_id: 1212101,
        nonce: 0,
        recipient,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let encoded_different = rlp::encode(&payload2);
    assert_ne!(encoded1, encoded_different);
}
