//! Token-related API request types and payloads.

use crate::crypto::Signable;
use crate::responses::MetadataKVPair;
use crate::{Authority, AuthorityAction, Signature};
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
fn deserialize_token_amount_decimal<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as DeError;
    // Accept string; fail fast on non-decimal
    let s = String::deserialize(deserializer)?;
    s.parse::<U256>().map_err(DeError::custom)
}

/// Token mint payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenMintPayload {
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
    /// Amount to mint.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
    /// Token address.
    pub token: Address,
}

impl AlloyEncodable for TokenMintPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
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

impl Signable for TokenMintPayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Token burn payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenBurnPayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Token account to burn from.
    pub recipient: Address,
    /// Amount to burn.
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
    /// Token address.
    pub token: Address,
}

impl AlloyEncodable for TokenBurnPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
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

impl Signable for TokenBurnPayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Token authority payload (unified for grant/revoke operations).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenAuthorityPayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Authority action (Grant or Revoke).
    pub action: AuthorityAction,
    /// Authority type.
    pub authority_type: Authority,
    /// Address to grant/revoke authority to/from.
    pub authority_address: Address,
    /// Token address.
    pub token: Address,
    /// Allowance value (for MintBurnTokens authority type).
    #[serde(
        serialize_with = "serialize_token_amount_decimal",
        deserialize_with = "deserialize_token_amount_decimal"
    )]
    pub value: U256,
}

impl AlloyEncodable for TokenAuthorityPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Use alloy_rlp encoding to match L1 implementation exactly
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.action.encode(&mut temp_buf);
        self.authority_type.encode(&mut temp_buf);
        self.authority_address.encode(&mut temp_buf);
        self.token.encode(&mut temp_buf);
        self.value.encode(&mut temp_buf);

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

impl Signable for TokenAuthorityPayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Pause action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum PauseAction {
    /// Pause token operations.
    Pause,
    /// Unpause token operations.
    Unpause,
}

impl PauseAction {
    /// Returns a stable string representation for RLP encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            PauseAction::Pause => "Pause",
            PauseAction::Unpause => "Unpause",
        }
    }
}

impl AlloyEncodable for PauseAction {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_str().encode(out);
    }
}

/// Token pause payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenPausePayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Pause action.
    pub action: PauseAction,
    /// Token address.
    pub token: Address,
}

impl AlloyEncodable for TokenPausePayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.action.encode(&mut temp_buf);
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

impl Signable for TokenPausePayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Blacklist action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum BlacklistAction {
    /// Add address to blacklist.
    Add,
    /// Remove address from blacklist.
    Remove,
}

impl BlacklistAction {
    /// Returns a stable string representation for RLP encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            BlacklistAction::Add => "Add",
            BlacklistAction::Remove => "Remove",
        }
    }
}

impl AlloyEncodable for BlacklistAction {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_str().encode(out);
    }
}

/// Token blacklist management payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenBlacklistPayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Blacklist action.
    pub action: BlacklistAction,
    /// Address to blacklist/unblacklist.
    pub address: Address,
    /// Token address.
    pub token: Address,
}

impl AlloyEncodable for TokenBlacklistPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.action.encode(&mut temp_buf);
        self.address.encode(&mut temp_buf);
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

impl Signable for TokenBlacklistPayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Whitelist action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum WhitelistAction {
    /// Add address to whitelist.
    Add,
    /// Remove address from whitelist.
    Remove,
}

impl WhitelistAction {
    /// Returns a stable string representation for RLP encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            WhitelistAction::Add => "Add",
            WhitelistAction::Remove => "Remove",
        }
    }
}

impl AlloyEncodable for WhitelistAction {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_str().encode(out);
    }
}

/// Token whitelist management payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenWhitelistPayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Whitelist action.
    pub action: WhitelistAction,
    /// Address to whitelist/unwhitelist.
    pub address: Address,
    /// Token address.
    pub token: Address,
}

impl AlloyEncodable for TokenWhitelistPayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.action.encode(&mut temp_buf);
        self.address.encode(&mut temp_buf);
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

impl Signable for TokenWhitelistPayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

/// Token metadata update payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenMetadataUpdatePayload {
    /// Recent epoch number.
    pub recent_epoch: u64,
    /// Recent checkpoint number.
    pub recent_checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// Account nonce.
    pub nonce: u64,
    /// Token name.
    pub name: String,
    /// Metadata URI.
    pub uri: String,
    /// Token address.
    pub token: Address,
    /// Additional metadata as key-value pairs.
    pub additional_metadata: Vec<MetadataKVPair>,
}

impl AlloyEncodable for TokenMetadataUpdatePayload {
    fn encode(&self, out: &mut dyn BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.recent_epoch.encode(&mut temp_buf);
        self.recent_checkpoint.encode(&mut temp_buf);
        self.chain_id.encode(&mut temp_buf);
        self.nonce.encode(&mut temp_buf);
        self.name.encode(&mut temp_buf);
        self.uri.encode(&mut temp_buf);
        self.token.encode(&mut temp_buf);
        self.additional_metadata.encode(&mut temp_buf);

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

impl Signable for TokenMetadataUpdatePayload {
    fn signature_hash(&self) -> B256 {
        // Use alloy_rlp encoding to match L1 exactly
        let mut encoded = Vec::new();
        self.encode(&mut encoded);
        keccak256(&encoded)
    }
}

// Request types that wrap payloads with signatures

/// Token mint request.
#[derive(Debug, Clone, Serialize)]
pub struct MintTokenRequest {
    #[serde(flatten)]
    pub payload: TokenMintPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token burn request.
#[derive(Debug, Clone, Serialize)]
pub struct BurnTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBurnPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token authority management request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenAuthorityRequest {
    #[serde(flatten)]
    pub payload: TokenAuthorityPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token blacklist request.
#[derive(Debug, Clone, Serialize)]
pub struct BlacklistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBlacklistPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token whitelist request.
#[derive(Debug, Clone, Serialize)]
pub struct WhitelistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenWhitelistPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token pause request.
#[derive(Debug, Clone, Serialize)]
pub struct PauseTokenRequest {
    #[serde(flatten)]
    pub payload: TokenPausePayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token metadata update request.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMetadataRequest {
    #[serde(flatten)]
    pub payload: TokenMetadataUpdatePayload,
    /// Signature for the payload.
    pub signature: Signature,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_token_mint_payload_decimal_serialization() {
        let payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(1000000000000000000u64), // 1 ETH in wei
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        // Should serialize value as decimal string, not hex
        assert!(json.contains("\"value\":\"1000000000000000000\""));
        assert!(!json.contains("0xde0b6b3a7640000")); // hex representation
    }

    #[test]
    fn test_token_burn_payload_decimal_serialization() {
        let payload = TokenBurnPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(500000000000000000u64), // 0.5 ETH in wei
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        // Should serialize value as decimal string, not hex
        assert!(json.contains("\"value\":\"500000000000000000\""));
        assert!(!json.contains("0x6f05b59d3b20000")); // hex representation
    }

    #[test]
    fn test_token_authority_payload_decimal_serialization() {
        let payload = TokenAuthorityPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            action: AuthorityAction::Grant,
            authority_type: Authority::MintBurnTokens,
            authority_address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            value: U256::from(2000000000000000000u64), // 2 ETH in wei allowance
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        // Should serialize value as decimal string, not hex
        assert!(json.contains("\"value\":\"2000000000000000000\""));
        assert!(!json.contains("0x1bc16d674ec80000")); // hex representation
    }

    #[test]
    fn test_decimal_serialization_consistency_with_payment_payload() {
        use crate::types::requests::transactions::PaymentPayload;

        let value = U256::from(1000000000000000000u64); // 1 ETH in wei
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");

        // Test PaymentPayload serialization
        let payment_payload = PaymentPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value,
            token,
        };

        // Test TokenMintPayload serialization
        let mint_payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value,
            token,
        };

        let payment_json =
            serde_json::to_string(&payment_payload).expect("Test data should be valid");
        let mint_json = serde_json::to_string(&mint_payload).expect("Test data should be valid");

        // Both should serialize value as decimal string consistently
        assert!(payment_json.contains("\"value\":\"1000000000000000000\""));
        assert!(mint_json.contains("\"value\":\"1000000000000000000\""));

        // Neither should contain hex representation
        assert!(!payment_json.contains("0xde0b6b3a7640000"));
        assert!(!mint_json.contains("0xde0b6b3a7640000"));
    }

    #[test]
    fn test_large_u256_decimal_serialization() {
        // Test with a very large U256 value
        let large_value = U256::from_str("123456789012345678901234567890123456789").unwrap();

        let payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: large_value,
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        // Should serialize large value as decimal string
        assert!(json.contains("\"value\":\"123456789012345678901234567890123456789\""));
    }

    #[test]
    fn test_zero_value_decimal_serialization() {
        let payload = TokenBurnPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::ZERO,
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");

        // Should serialize zero value as "0"
        assert!(json.contains("\"value\":\"0\""));
        assert!(!json.contains("\"value\":\"0x0\""));
    }

    #[test]
    fn test_token_mint_payload_decimal_deserialization() {
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "1000000000000000000",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: TokenMintPayload =
            serde_json::from_str(json).expect("Should deserialize decimal value");
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
        assert_eq!(payload.recent_epoch, 100);
        assert_eq!(payload.nonce, 5);
    }

    #[test]
    fn test_token_burn_payload_decimal_deserialization() {
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "500000000000000000",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: TokenBurnPayload =
            serde_json::from_str(json).expect("Should deserialize decimal value");
        assert_eq!(payload.value, U256::from(500000000000000000u64));
    }

    #[test]
    fn test_token_authority_payload_decimal_deserialization() {
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "action": "Grant",
            "authority_type": "MintBurnTokens",
            "authority_address": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "token": "0x1234567890abcdef1234567890abcdef12345678",
            "value": "2000000000000000000"
        }"#;

        let payload: TokenAuthorityPayload =
            serde_json::from_str(json).expect("Should deserialize decimal value");
        assert_eq!(payload.value, U256::from(2000000000000000000u64));
        assert_eq!(payload.action, AuthorityAction::Grant);
        assert_eq!(payload.authority_type, Authority::MintBurnTokens);
    }

    #[test]
    fn test_round_trip_serialization_deserialization() {
        let original_payload = TokenMintPayload {
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

        // Deserialize back from JSON
        let deserialized_payload: TokenMintPayload =
            serde_json::from_str(&json).expect("Should deserialize");

        // Should be identical
        assert_eq!(original_payload, deserialized_payload);

        // Verify value is handled correctly
        assert_eq!(original_payload.value, deserialized_payload.value);
    }

    #[test]
    fn test_large_u256_decimal_deserialization() {
        let large_value_str = "123456789012345678901234567890123456789";
        let json = format!(
            r#"{{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "{}",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }}"#,
            large_value_str
        );

        let payload: TokenMintPayload =
            serde_json::from_str(&json).expect("Should deserialize large decimal value");
        let expected_value = U256::from_str(large_value_str).unwrap();
        assert_eq!(payload.value, expected_value);
    }

    #[test]
    fn test_zero_value_decimal_deserialization() {
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "0",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: TokenBurnPayload =
            serde_json::from_str(json).expect("Should deserialize zero value");
        assert_eq!(payload.value, U256::ZERO);
    }

    #[test]
    fn test_invalid_decimal_value_deserialization_fails() {
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "not_a_number",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let result: Result<TokenMintPayload, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Should fail to deserialize invalid decimal value"
        );
    }

    #[test]
    fn test_hex_value_deserialization_works() {
        // U256::parse can handle both hex and decimal formats
        let json = r#"{
            "recent_epoch": 100,
            "recent_checkpoint": 200,
            "chain_id": 1212101,
            "nonce": 5,
            "recipient": "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
            "value": "0xde0b6b3a7640000",
            "token": "0x1234567890abcdef1234567890abcdef12345678"
        }"#;

        let payload: TokenMintPayload =
            serde_json::from_str(json).expect("Should deserialize hex value");
        // This hex value should equal 1000000000000000000 in decimal
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
    }

    // ========================================================================
    // ALLOY RLP ENCODING TESTS
    // ========================================================================

    #[test]
    fn test_token_mint_payload_alloy_rlp_encoding() {
        let payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
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
            "TokenMintPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_token_burn_payload_alloy_rlp_encoding() {
        let payload = TokenBurnPayload {
            recent_epoch: 150,
            recent_checkpoint: 250,
            chain_id: 1212101,
            nonce: 10,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(500000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenBurnPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_token_authority_payload_alloy_rlp_encoding() {
        let payload = TokenAuthorityPayload {
            recent_epoch: 300,
            recent_checkpoint: 400,
            chain_id: 1212101,
            nonce: 15,
            action: AuthorityAction::Grant,
            authority_type: Authority::MintBurnTokens,
            authority_address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .unwrap(),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            value: U256::from(2000000000000000000u64),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenAuthorityPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_pause_action_alloy_rlp_encoding() {
        let pause_action = PauseAction::Pause;
        let unpause_action = PauseAction::Unpause;

        let mut pause_encoded = Vec::new();
        let mut unpause_encoded = Vec::new();

        pause_action.encode(&mut pause_encoded);
        unpause_action.encode(&mut unpause_encoded);

        assert!(
            !pause_encoded.is_empty(),
            "Pause action should encode to non-empty bytes"
        );
        assert!(
            !unpause_encoded.is_empty(),
            "Unpause action should encode to non-empty bytes"
        );
        assert_ne!(
            pause_encoded, unpause_encoded,
            "Different actions should have different encodings"
        );
    }

    #[test]
    fn test_token_pause_payload_alloy_rlp_encoding() {
        let payload = TokenPausePayload {
            recent_epoch: 500,
            recent_checkpoint: 600,
            chain_id: 1212101,
            nonce: 20,
            action: PauseAction::Pause,
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenPausePayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_blacklist_action_alloy_rlp_encoding() {
        let add_action = BlacklistAction::Add;
        let remove_action = BlacklistAction::Remove;

        let mut add_encoded = Vec::new();
        let mut remove_encoded = Vec::new();

        add_action.encode(&mut add_encoded);
        remove_action.encode(&mut remove_encoded);

        assert!(
            !add_encoded.is_empty(),
            "Add action should encode to non-empty bytes"
        );
        assert!(
            !remove_encoded.is_empty(),
            "Remove action should encode to non-empty bytes"
        );
        assert_ne!(
            add_encoded, remove_encoded,
            "Different actions should have different encodings"
        );
    }

    #[test]
    fn test_token_blacklist_payload_alloy_rlp_encoding() {
        let payload = TokenBlacklistPayload {
            recent_epoch: 700,
            recent_checkpoint: 800,
            chain_id: 1212101,
            nonce: 25,
            action: BlacklistAction::Add,
            address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenBlacklistPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_whitelist_action_alloy_rlp_encoding() {
        let add_action = WhitelistAction::Add;
        let remove_action = WhitelistAction::Remove;

        let mut add_encoded = Vec::new();
        let mut remove_encoded = Vec::new();

        add_action.encode(&mut add_encoded);
        remove_action.encode(&mut remove_encoded);

        assert!(
            !add_encoded.is_empty(),
            "Add action should encode to non-empty bytes"
        );
        assert!(
            !remove_encoded.is_empty(),
            "Remove action should encode to non-empty bytes"
        );
        assert_ne!(
            add_encoded, remove_encoded,
            "Different actions should have different encodings"
        );
    }

    #[test]
    fn test_token_whitelist_payload_alloy_rlp_encoding() {
        let payload = TokenWhitelistPayload {
            recent_epoch: 900,
            recent_checkpoint: 1000,
            chain_id: 1212101,
            nonce: 30,
            action: WhitelistAction::Add,
            address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenWhitelistPayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_token_metadata_update_payload_alloy_rlp_encoding() {
        let payload = TokenMetadataUpdatePayload {
            recent_epoch: 1100,
            recent_checkpoint: 1200,
            chain_id: 1212101,
            nonce: 35,
            name: "Test Token".to_string(),
            uri: "https://example.com/token.json".to_string(),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            additional_metadata: vec![
                MetadataKVPair {
                    key: "version".to_string(),
                    value: "1.0".to_string(),
                },
                MetadataKVPair {
                    key: "author".to_string(),
                    value: "OneMoney Team".to_string(),
                },
            ],
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);

        assert!(
            !encoded.is_empty(),
            "TokenMetadataUpdatePayload should encode to non-empty bytes"
        );

        // Test deterministic encoding
        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);
        assert_eq!(encoded, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_payload_signature_hash_consistency() {
        let payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
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
    fn test_different_payloads_different_encodings() {
        let payload1 = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let payload2 = TokenMintPayload {
            recent_epoch: 101, // Different epoch
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
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
    fn test_encoding_with_large_values() {
        let large_value = U256::from_str("123456789012345678901234567890123456789").unwrap();

        let payload = TokenMintPayload {
            recent_epoch: u64::MAX,
            recent_checkpoint: u64::MAX,
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
    fn test_encoding_with_zero_values() {
        let payload = TokenBurnPayload {
            recent_epoch: 0,
            recent_checkpoint: 0,
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
}
