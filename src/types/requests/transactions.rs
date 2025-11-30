//! Transaction-related API request types.

use crate::Signature;
use crate::crypto::Signable;
use alloy_primitives::{Address, B256, U256, keccak256};
use alloy_rlp::{BufMut, Encodable as AlloyEncodable};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;

/// Payment transaction payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentPayload {
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Recipient address.
    pub recipient: Address,
    /// Amount to transfer.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
    /// Token address (use native token address for native transfers).
    pub token: Address,
}

/// Serialize U256 as decimal string instead of hex (L1 compatibility).
fn serialize_token_amount_decimal<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Deserialize U256 from decimal string instead of hex (L1 compatibility).
fn deserialize_token_amount_decimal<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as DeError;
    // Accept string; fail fast on non-decimal
    let s = String::deserialize(deserializer)?;
    s.parse::<U256>().map_err(DeError::custom)
}

impl Display for PaymentPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Payment to {}: value {}, token {}, nonce {}, chain {}",
            self.recipient, self.value, self.token, self.nonce, self.chain_id
        )
    }
}

impl AlloyEncodable for PaymentPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.recipient.encode(&mut temp_buf);
        self.value.encode(&mut temp_buf);
        self.token.encode(&mut temp_buf);

        // Now encode the proper header with correct payload length
        alloy_rlp::Header {
            list: true,
            payload_length: temp_buf.len(),
        }
        .encode(out);

        // Write the actual payload
        out.put_slice(&temp_buf);
    }
}

impl PaymentPayload {
    /// Calculate the signature hash for this payload.
    /// This matches the L1 implementation's signature_hash method.
    pub fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

impl Signable for PaymentPayload {
    fn signature_hash(&self) -> B256 {
        PaymentPayload::signature_hash(self)
    }
}

/// Payment transaction request.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    #[serde(flatten)]
    pub payload: PaymentPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Fee estimation request.
/// Matches L1 server's EstimateFeeRequest structure with string query parameters.
#[derive(Debug, Clone, Serialize)]
pub struct FeeEstimateRequest {
    /// From address (as string for query parameter).
    pub from: String,
    /// Value to transfer (as string for query parameter).
    pub value: String,
    /// Token address (optional, as string for query parameter).
    pub token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_payment_payload_decimal_deserialization() {
        let json = r#"{
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "1000000000000000000",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: PaymentPayload =
            serde_json::from_str(json).expect("Should deserialize decimal value");
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
        assert_eq!(payload.nonce, 5);
    }

    #[test]
    fn test_payment_payload_round_trip_serialization() {
        let original_payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original_payload).expect("Should serialize");

        // Verify it serializes as decimal
        assert!(json.contains("\"value\":\"1000000000000000000\""));
        assert!(!json.contains("0xde0b6b3a7640000"));

        // Deserialize back from JSON
        let deserialized_payload: PaymentPayload =
            serde_json::from_str(&json).expect("Should deserialize");

        // Should be identical
        assert_eq!(original_payload, deserialized_payload);

        // Verify value is handled correctly
        assert_eq!(original_payload.value, deserialized_payload.value);
    }

    // ========================================================================
    // ALLOY RLP ENCODING TESTS
    // ========================================================================

    #[test]
    fn test_payment_payload_alloy_rlp_encoding() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "PaymentPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_payment_payload_signature_hash_consistency() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 10,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(500000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        // Test that signature_hash is deterministic
        let hash1 = payload.signature_hash();
        let hash2 = payload.signature_hash();
        assert_eq!(hash1, hash2, "Signature hash should be deterministic");

        // Test that signature_hash produces valid B256
        assert_eq!(hash1.len(), 32, "Signature hash should be 32 bytes");
        assert_ne!(hash1, B256::ZERO, "Signature hash should not be zero");
    }

    #[test]
    fn test_payment_payload_signable_trait() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 15,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(2000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        // Test Signable trait implementation
        let signable_hash = payload.signature_hash();
        let direct_hash = payload.signature_hash();
        assert_eq!(
            signable_hash, direct_hash,
            "Signable trait should match direct implementation"
        );
    }

    #[test]
    fn test_different_payment_payloads_different_encodings() {
        let payload1 = PaymentPayload {
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let payload2 = PaymentPayload {
            chain_id: 1212102, // Different chain id
            nonce: 6,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded1 = Vec::new();
        let mut encoded2 = Vec::new();

        payload1.encode(&mut encoded1);
        payload2.encode(&mut encoded2);

        assert_ne!(
            encoded1, encoded2,
            "Different payloads should have different encodings"
        );
        assert_ne!(
            payload1.signature_hash(),
            payload2.signature_hash(),
            "Different payloads should have different signature hashes"
        );
    }

    #[test]
    fn test_payment_payload_encoding_with_large_values() {
        let large_value = U256::from_str("999999999999999999999999999999999999999").unwrap();

        let payload = PaymentPayload {
            chain_id: u64::MAX,
            nonce: u64::MAX,
            recipient: Address::from_str("0xffffffffffffffffffffffffffffffffffffffff").unwrap(),
            value: large_value,
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "Payload with large values should encode successfully"
        );

        // Test signature hash with large values
        let hash = payload.signature_hash();
        assert_ne!(
            hash,
            B256::ZERO,
            "Signature hash should be valid even with large values"
        );
    }

    #[test]
    fn test_payment_payload_encoding_with_zero_values() {
        let payload = PaymentPayload {
            chain_id: 0,
            nonce: 0,
            recipient: Address::ZERO,
            value: U256::ZERO,
            token: Address::ZERO,
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "Payload with zero values should encode successfully"
        );

        // Test signature hash with zero values
        let hash = payload.signature_hash();
        assert_ne!(
            hash,
            B256::ZERO,
            "Signature hash should be valid even with zero values"
        );
    }

    #[test]
    fn test_payment_payload_default_implementation() {
        let default_payload = PaymentPayload::default();

        assert_eq!(default_payload.chain_id, 0);
        assert_eq!(default_payload.nonce, 0);
        assert_eq!(default_payload.recipient, Address::ZERO);
        assert_eq!(default_payload.value, U256::ZERO);
        assert_eq!(default_payload.token, Address::ZERO);

        // Test that default can be encoded
        let mut encoded = Vec::new();
        default_payload.encode(&mut encoded);
        assert!(
            !encoded.is_empty(),
            "Default payload should encode successfully"
        );
    }

    #[test]
    fn test_payment_payload_display_formatting() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let display_str = format!("{}", payload);

        assert!(display_str.contains("Payment to"));
        // Check for the address (case insensitive)
        assert!(
            display_str
                .to_lowercase()
                .contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0")
        );
        assert!(display_str.contains("value 1000000000000000000"));
        assert!(display_str.contains("nonce 5"));
        assert!(display_str.contains("chain 1212101"));
    }

    #[test]
    fn test_payment_payload_traits() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let payload_clone = payload.clone();

        // Test Clone, PartialEq, Eq
        assert_eq!(payload, payload_clone);

        // Test Hash (via collecting into HashSet)
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(payload.clone());
        set.insert(payload_clone);
        assert_eq!(set.len(), 1, "Hash should work correctly");

        // Test Debug
        let debug_str = format!("{:?}", payload);
        assert!(debug_str.contains("PaymentPayload"));
    }

    #[test]
    fn test_payment_payload_serialization_consistency() {
        let payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 25,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(5000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        // Test JSON serialization/deserialization
        let json = serde_json::to_string(&payload).expect("Should serialize to JSON");
        let deserialized: PaymentPayload =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(
            payload, deserialized,
            "Serialization round-trip should preserve value"
        );

        // Verify value is serialized as decimal
        assert!(json.contains("\"value\":\"5000000000000000000\""));
        assert!(!json.contains("0x4563918244f40000")); // hex representation
    }

    #[test]
    fn test_payment_payload_encoding_edge_cases() {
        // Test with minimum values
        let min_payload = PaymentPayload {
            chain_id: 1,
            nonce: 1,
            recipient: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            value: U256::from(1u64),
            token: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
        };

        let mut encoded = Vec::new();
        min_payload.encode(&mut encoded);
        assert!(
            !encoded.is_empty(),
            "Minimum payload should encode successfully"
        );

        // Test with very specific value patterns
        let pattern_payload = PaymentPayload {
            chain_id: 1212101,
            nonce: 111,
            recipient: Address::from_str("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
            value: U256::from_str("123456789012345678901234567890").unwrap(),
            token: Address::from_str("0x5555555555555555555555555555555555555555").unwrap(),
        };

        let mut encoded = Vec::new();
        pattern_payload.encode(&mut encoded);
        assert!(
            !encoded.is_empty(),
            "Pattern payload should encode successfully"
        );

        // Test signature hash consistency
        let hash1 = pattern_payload.signature_hash();
        let hash2 = pattern_payload.signature_hash();
        assert_eq!(
            hash1, hash2,
            "Signature hash should be consistent for edge case values"
        );
    }
}
