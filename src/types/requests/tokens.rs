//! Token-related API request types and payloads.

use crate::crypto::Signable;
use crate::responses::MetadataKVPair;
use crate::{Authority, AuthorityAction};
use alloy_primitives::{Address, B256, U256, keccak256};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};

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
    pub value: U256,
    /// Token address.
    pub token: Address,
}

impl Encodable for TokenMintPayload {
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

impl Signable for TokenMintPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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
    pub value: U256,
    /// Token address.
    pub token: Address,
}

impl Encodable for TokenBurnPayload {
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

impl Signable for TokenBurnPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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
    pub value: U256,
}

impl Encodable for TokenAuthorityPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.action.as_str());
        s.append(&self.authority_type.as_str());
        s.append(&self.authority_address.as_slice());
        s.append(&self.token.as_slice());
        // Encode U256 as compact bytes (no leading zeros) to match L1
        let compact_bytes = compact_u256_bytes(&self.value);
        s.append(&compact_bytes);
    }
}

impl Signable for TokenAuthorityPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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

impl Encodable for TokenPausePayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.action.as_str());
        s.append(&self.token.as_slice());
    }
}

impl Signable for TokenPausePayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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

impl Encodable for TokenBlacklistPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.action.as_str());
        s.append(&self.address.as_slice());
        s.append(&self.token.as_slice());
    }
}

impl Signable for TokenBlacklistPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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

impl Encodable for TokenWhitelistPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.action.as_str());
        s.append(&self.address.as_slice());
        s.append(&self.token.as_slice());
    }
}

impl Signable for TokenWhitelistPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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

impl Encodable for TokenMetadataUpdatePayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(8);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.name);
        s.append(&self.uri);
        s.append(&self.token.as_slice()); // token at position 7
        // Encode Vec<MetadataKVPair> as nested RLP list
        s.append_list(&self.additional_metadata);
    }
}

impl Signable for TokenMetadataUpdatePayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
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
    pub signature: crate::Signature,
}

/// Token burn request.
#[derive(Debug, Clone, Serialize)]
pub struct BurnTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBurnPayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Token authority management request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenAuthorityRequest {
    #[serde(flatten)]
    pub payload: TokenAuthorityPayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Token blacklist request.
#[derive(Debug, Clone, Serialize)]
pub struct BlacklistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBlacklistPayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Token whitelist request.
#[derive(Debug, Clone, Serialize)]
pub struct WhitelistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenWhitelistPayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Token pause request.
#[derive(Debug, Clone, Serialize)]
pub struct PauseTokenRequest {
    #[serde(flatten)]
    pub payload: TokenPausePayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Token metadata update request.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMetadataRequest {
    #[serde(flatten)]
    pub payload: TokenMetadataUpdatePayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}
