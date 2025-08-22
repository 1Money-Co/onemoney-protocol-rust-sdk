//! Comprehensive transaction API coverage tests

use alloy_primitives::{Address, B256, U256};
use onemoney_protocol::*;
use std::str::FromStr;

#[test]
fn test_payment_payload_structure_comprehensive() {
    let payload = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::from(1000000000000000000u64),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    // Test all fields
    assert_eq!(payload.recent_epoch, 100);
    assert_eq!(payload.recent_checkpoint, 200);
    assert_eq!(payload.chain_id, 1212101);
    assert_eq!(payload.nonce, 5);
    assert_eq!(payload.value, U256::from(1000000000000000000u64));

    // Test serialization/deserialization
    let json = serde_json::to_string(&payload).expect("Test data should be valid");
    let deserialized: PaymentPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payload.recent_epoch, deserialized.recent_epoch);
    assert_eq!(payload.recipient, deserialized.recipient);
    assert_eq!(payload.token, deserialized.token);

    // Test signature hash calculation
    let hash = payload.signature_hash();
    assert_ne!(hash, B256::default());

    // Test deterministic hash
    let hash2 = payload.signature_hash();
    assert_eq!(hash, hash2);

    // Test RLP encoding
    let encoded = rlp::encode(&payload);
    assert!(!encoded.is_empty());

    // Test display
    let display_str = format!("{}", payload);
    assert!(display_str.contains("Payment to"));
}

#[test]
fn test_payment_payload_different_values() {
    // Test with different values to ensure hash changes
    let payload1 = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::from(1000000000000000000u64),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    let payload2 = PaymentPayload {
        recent_epoch: 101, // Different epoch
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::from(1000000000000000000u64),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    // Hashes should be different
    assert_ne!(payload1.signature_hash(), payload2.signature_hash());

    // RLP encodings should be different
    assert_ne!(rlp::encode(&payload1), rlp::encode(&payload2));
}

#[test]
fn test_payment_request_structure() {
    let payload = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::from(1000000000000000000u64),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
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
}

#[test]
fn test_api_path_construction() {
    use onemoney_protocol::client::config::api_path;
    use onemoney_protocol::client::config::endpoints::transactions::*;

    // Test payment path
    let payment_path = api_path(PAYMENT);
    assert!(payment_path.contains("/transactions/payment"));

    // Test by_hash path with parameter
    let hash = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    let by_hash_path = api_path(&format!("{}?hash={}", BY_HASH, hash));
    assert!(by_hash_path.contains("/transactions/by_hash"));
    assert!(by_hash_path.contains(hash));

    // Test estimate fee path
    let estimate_path = api_path(ESTIMATE_FEE);
    assert!(estimate_path.contains("/transactions/estimate_fee"));

    // Test receipt by hash path
    let receipt_path = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, hash));
    assert!(receipt_path.contains("/transactions/receipt/by_hash"));
    assert!(receipt_path.contains(hash));
}

#[test]
fn test_fee_estimate_query_params() {
    // Simulate the query parameter construction logic from estimate_fee method
    let request = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "1000000000000000000".to_string(),
        token: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };

    let mut path = "/transactions/estimate_fee".to_string();
    let mut query_params = Vec::new();

    query_params.push(format!("from={}", request.from));
    query_params.push(format!("value={}", request.value));
    if let Some(token) = request.token {
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
}

#[test]
fn test_fee_estimate_query_params_without_token() {
    // Test query parameter construction without token
    let request = FeeEstimateRequest {
        from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
        value: "500000000000000000".to_string(),
        token: None,
    };

    let mut path = "/transactions/estimate_fee".to_string();
    let mut query_params = Vec::new();

    query_params.push(format!("from={}", request.from));
    query_params.push(format!("value={}", request.value));
    if let Some(token) = request.token {
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
}

#[test]
fn test_payment_payload_edge_cases() {
    // Test with zero values
    let payload_zero = PaymentPayload {
        recent_epoch: 0,
        recent_checkpoint: 0,
        chain_id: 1212101,
        nonce: 0,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::ZERO,
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    let hash_zero = payload_zero.signature_hash();
    assert_ne!(hash_zero, B256::default());

    // Test with maximum values
    let payload_max = PaymentPayload {
        recent_epoch: u64::MAX,
        recent_checkpoint: u64::MAX,
        chain_id: 1212101,
        nonce: u64::MAX,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::MAX,
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    let hash_max = payload_max.signature_hash();
    assert_ne!(hash_max, B256::default());
    assert_ne!(hash_zero, hash_max);
}

#[test]
fn test_endpoints_constants() {
    use onemoney_protocol::client::config::endpoints::transactions::*;

    assert_eq!(PAYMENT, "/transactions/payment");
    assert_eq!(BY_HASH, "/transactions/by_hash");
    assert_eq!(ESTIMATE_FEE, "/transactions/estimate_fee");
    assert_eq!(RECEIPT_BY_HASH, "/transactions/receipt/by_hash");
}

#[test]
fn test_signature_serialization() {
    let signature = Signature {
        r: U256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
            .expect("Test data should be valid"),
        s: U256::from_str("0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321")
            .expect("Test data should be valid"),
        v: 28,
    };

    let json = serde_json::to_string(&signature).expect("Test data should be valid");
    let deserialized: Signature = serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(signature.r, deserialized.r);
    assert_eq!(signature.s, deserialized.s);
    assert_eq!(signature.v, deserialized.v);
}

#[test]
fn test_payment_payload_display() {
    let payload = PaymentPayload {
        recent_epoch: 123,
        recent_checkpoint: 456,
        chain_id: 1212101,
        nonce: 10,
        recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        value: U256::from(1500000000000000000u64),
        token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
    };

    let display_str = format!("{}", payload);
    assert!(display_str.contains("Payment to"));
    assert!(display_str.contains("0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0"));
    assert!(display_str.contains("1500000000000000000"));
    assert!(display_str.contains("token 0x1234567890AbcdEF1234567890aBcdef12345678"));
}
