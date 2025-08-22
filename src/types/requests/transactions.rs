//! Transaction-related API request types.

use crate::Signature;
use crate::crypto::Signable;
use alloy_primitives::{Address, B256, U256};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;

/// Creates a compact byte representation of U256 by removing leading zeros.
/// Returns vec![0] if all bytes are zero.
fn compact_u256_bytes(value: &U256) -> Vec<u8> {
    let value_bytes = value.to_be_bytes_vec();

    // Find the first non-zero byte index
    let first_non_zero = value_bytes.iter().position(|&b| b != 0);

    match first_non_zero {
        Some(index) => value_bytes[index..].to_vec(),
        None => vec![0], // All bytes are zero
    }
}

/// Payment transaction payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentPayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
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
    // Accept string; fail fast on non-decimal
    let s = String::deserialize(deserializer)?;
    s.parse::<U256>().map_err(serde::de::Error::custom)
}

impl Display for PaymentPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Payment to {}: value {}, token {}, nonce {}, epoch {}, checkpoint {}, chain {}",
            self.recipient,
            self.value,
            self.token,
            self.nonce,
            self.recent_epoch,
            self.recent_checkpoint,
            self.chain_id
        )
    }
}

impl Encodable for PaymentPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.recipient.as_slice());
        // Encode U256 as compact bytes (no leading zeros) to match L1
        let compact_bytes = compact_u256_bytes(&self.value);
        s.append(&compact_bytes);
        s.append(&self.token.as_slice());
    }
}

impl PaymentPayload {
    /// Calculate the signature hash for this payload.
    /// This matches the L1 implementation's signature_hash method.
    pub fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        alloy_primitives::keccak256(&encoded)
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
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "1000000000000000000",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: PaymentPayload =
            serde_json::from_str(json).expect("Should deserialize decimal value");
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
        assert_eq!(payload.recent_epoch, 100);
        assert_eq!(payload.nonce, 5);
    }

    #[test]
    fn test_payment_payload_round_trip_serialization() {
        let original_payload = PaymentPayload {
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
}
