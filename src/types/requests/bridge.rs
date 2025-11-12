//! Bridge-related API request types and payloads.

use crate::Signature;
use crate::crypto::Signable;
use alloy_primitives::{Address, B256, U256, keccak256};
use alloy_rlp::{BufMut, Encodable as AlloyEncodable};
use serde::{Deserialize, Serialize};

// Serialize U256 as decimal string instead of hex (L1 compatibility)
fn serialize_token_amount_decimal<S>(
    value: &U256,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

// Deserialize U256 from decimal string instead of hex (L1 compatibility)
fn deserialize_token_amount_decimal<'de, D>(deserializer: D) -> std::result::Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as DeError;
    let s = String::deserialize(deserializer)?;
    s.parse::<U256>().map_err(DeError::custom)
}

/// Token bridge and mint payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenBridgeAndMintPayload {
    /// The most recent checkpoint when the transaction is submitted.
    pub recent_checkpoint: u64,
    /// The chain id of the transaction.
    pub chain_id: u64,
    /// The nonce of the transaction.
    pub nonce: u64,
    /// The recipient address to mint tokens to.
    pub recipient: Address,
    /// The amount of tokens to mint from the bridge.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
    /// The token address of the transaction.
    pub token: Address,
    /// The chain ID from which tokens are being bridged.
    pub source_chain_id: u64,
    /// The transaction hash on the source chain proving the lock/burn.
    pub source_tx_hash: String,
    /// Optional bridge metadata for additional verification.
    pub bridge_metadata: Option<String>,
}

impl AlloyEncodable for TokenBridgeAndMintPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        self.recent_checkpoint.encode(out);
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.recipient.encode(out);
        self.value.encode(out);
        self.token.encode(out);
        self.source_chain_id.encode(out);
        self.source_tx_hash.encode(out);
        // Encode presence flag + value when present to preserve Some("") vs None distinction
        // This matches the L1 implementation
        self.bridge_metadata.is_some().encode(out);
        if let Some(meta) = &self.bridge_metadata {
            meta.encode(out);
        }
    }
}

impl Signable for TokenBridgeAndMintPayload {
    fn signature_hash(&self) -> B256 {
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Token burn and bridge payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenBurnAndBridgePayload {
    /// The most recent checkpoint when the transaction is submitted.
    pub recent_checkpoint: u64,
    /// The chain id of the transaction.
    pub chain_id: u64,
    /// The nonce of the transaction.
    pub nonce: u64,
    /// The address to burn tokens from.
    pub sender: Address,
    /// The amount of tokens to burn for bridging.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
    /// The token address of the transaction.
    pub token: Address,
    /// The destination chain ID to bridge tokens to.
    pub destination_chain_id: u64,
    /// The destination address on the target chain.
    pub destination_address: String,
    /// The bridging fee necessary to escrow for transferring tokens to the destination chain.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub escrow_fee: U256,
    /// Optional bridge metadata for additional information.
    pub bridge_metadata: Option<String>,
    /// Burn and bridge nonce for tracking bridge operations.
    pub bbnonce: u64,
}

impl AlloyEncodable for TokenBurnAndBridgePayload {
    fn encode(&self, out: &mut dyn BufMut) {
        self.recent_checkpoint.encode(out);
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.sender.encode(out);
        self.value.encode(out);
        self.token.encode(out);
        self.destination_chain_id.encode(out);
        self.destination_address.encode(out);
        self.escrow_fee.encode(out);
        // Encode presence flag + value when present to preserve Some("") vs None distinction
        // This matches the L1 implementation
        self.bridge_metadata.is_some().encode(out);
        if let Some(meta) = &self.bridge_metadata {
            meta.encode(out);
        }
        self.bbnonce.encode(out);
    }
}

impl Signable for TokenBurnAndBridgePayload {
    fn signature_hash(&self) -> B256 {
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

// Request types that wrap payloads with signatures

/// Token bridge and mint request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenBridgeAndMintRequest {
    #[serde(flatten)]
    pub data: TokenBridgeAndMintPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token burn and bridge request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenBurnAndBridgeRequest {
    #[serde(flatten)]
    pub data: TokenBurnAndBridgePayload,
    /// Signature for the payload.
    pub signature: Signature,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_token_bridge_and_mint_payload_structure() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");

        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value: U256::from(1000000000000000000u64),
            token,
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        assert_eq!(payload.recent_checkpoint, 200);
        assert_eq!(payload.chain_id, 1212101);
        assert_eq!(payload.nonce, 5);
        assert_eq!(payload.recipient, address);
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
        assert_eq!(payload.token, token);
        assert_eq!(payload.source_chain_id, 1);
        assert_eq!(
            payload.source_tx_hash,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        );
        assert_eq!(payload.bridge_metadata, None);
    }

    #[test]
    fn test_token_bridge_and_mint_payload_with_metadata() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");

        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value: U256::from(1000000000000000000u64),
            token,
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: Some("bridge_proof_v1".to_string()),
        };

        assert_eq!(payload.bridge_metadata, Some("bridge_proof_v1".to_string()));
    }

    #[test]
    fn test_token_bridge_and_mint_payload_decimal_serialization() {
        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        assert!(json.contains("\"value\":\"1000000000000000000\""));
        assert!(!json.contains("0xde0b6b3a7640000"));
    }

    #[test]
    fn test_token_bridge_and_mint_payload_alloy_rlp_encoding() {
        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenBridgeAndMintPayload should encode to non-empty bytes"
        );

        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_bridge_and_mint_payload_signature_hash_consistency() {
        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let hash1 = payload.signature_hash();
        let hash2 = payload.signature_hash();
        assert_eq!(hash1, hash2, "Signature hash should be deterministic");

        assert_eq!(hash1.len(), 32, "Signature hash should be 32 bytes");
        assert_ne!(hash1, B256::ZERO, "Signature hash should not be zero");
    }

    #[test]
    fn test_token_burn_and_bridge_payload_structure() {
        let sender = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");

        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            sender,
            value: U256::from(500000000u64),
            token,
            destination_chain_id: 1,
            destination_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bbnonce: 42,
        };

        assert_eq!(payload.recent_checkpoint, 200);
        assert_eq!(payload.chain_id, 1212101);
        assert_eq!(payload.nonce, 5);
        assert_eq!(payload.bbnonce, 42);
        assert_eq!(payload.sender, sender);
        assert_eq!(payload.value, U256::from(500000000u64));
        assert_eq!(payload.token, token);
        assert_eq!(payload.destination_chain_id, 1);
        assert_eq!(
            payload.destination_address,
            "0x1234567890abcdef1234567890abcdef12345678"
        );
        assert_eq!(payload.escrow_fee, U256::from(1000000u64));
        assert_eq!(payload.bridge_metadata, None);
    }

    #[test]
    fn test_token_burn_and_bridge_payload_decimal_serialization() {
        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            sender: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(500000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            destination_chain_id: 1,
            destination_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bbnonce: 42,
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        assert!(json.contains("\"value\":\"500000000\""));
        assert!(json.contains("\"escrow_fee\":\"1000000\""));
    }

    #[test]
    fn test_token_burn_and_bridge_payload_alloy_rlp_encoding() {
        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            sender: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(500000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            destination_chain_id: 1,
            destination_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bbnonce: 42,
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenBurnAndBridgePayload should encode to non-empty bytes"
        );

        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_burn_and_bridge_payload_signature_hash_consistency() {
        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            sender: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(500000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            destination_chain_id: 1,
            destination_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bbnonce: 42,
        };

        let hash1 = payload.signature_hash();
        let hash2 = payload.signature_hash();
        assert_eq!(hash1, hash2, "Signature hash should be deterministic");

        assert_eq!(hash1.len(), 32, "Signature hash should be 32 bytes");
        assert_ne!(hash1, B256::ZERO, "Signature hash should not be zero");
    }
}
