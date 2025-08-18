//! Token-related API operations.

use crate::client::config::api_path;
use crate::client::config::endpoints::tokens::{
    BURN, GRANT_AUTHORITY, MANAGE_BLACKLIST, MANAGE_WHITELIST, MINT, PAUSE, REVOKE_AUTHORITY,
    TOKEN_METADATA, UPDATE_METADATA,
};
use crate::client::Client;
use crate::crypto::{sign_transaction_payload, Signable};
use crate::{
    Authority, AuthorityAction, MetadataKVPair, MintInfo, OneMoneyAddress, Result, Signature,
    TokenAmount,
};
use alloy_primitives::{keccak256, B256};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};

/// Token mint payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub recipient: OneMoneyAddress,
    /// Amount to mint.
    pub value: TokenAmount,
    /// Token address.
    pub token: OneMoneyAddress,
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
        let value_bytes = self.value.to_be_bytes_vec();
        let mut compact_bytes = value_bytes;
        while !compact_bytes.is_empty() && compact_bytes[0] == 0 {
            compact_bytes.remove(0);
        }
        if compact_bytes.is_empty() {
            compact_bytes = vec![0];
        }
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

/// Token mint request.
#[derive(Debug, Clone, Serialize)]
pub struct MintTokenRequest {
    #[serde(flatten)]
    pub payload: TokenMintPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token burn payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub recipient: OneMoneyAddress,
    /// Amount to burn.
    pub value: TokenAmount,
    /// Token address.
    pub token: OneMoneyAddress,
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
        let value_bytes = self.value.to_be_bytes_vec();
        let mut compact_bytes = value_bytes;
        while !compact_bytes.is_empty() && compact_bytes[0] == 0 {
            compact_bytes.remove(0);
        }
        if compact_bytes.is_empty() {
            compact_bytes = vec![0];
        }
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

/// Token burn request.
#[derive(Debug, Clone, Serialize)]
pub struct BurnTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBurnPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token authority payload (unified for grant/revoke operations).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub authority_address: OneMoneyAddress,
    /// Token address.
    pub token: OneMoneyAddress,
    /// Allowance value (for MintBurnTokens authority type).
    pub value: TokenAmount,
}

impl Encodable for TokenAuthorityPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&format!("{:?}", self.action));
        s.append(&format!("{:?}", self.authority_type));
        s.append(&self.authority_address.as_slice());
        s.append(&self.token.as_slice());
        // Encode U256 as compact bytes (no leading zeros) to match L1
        let value_bytes = self.value.to_be_bytes_vec();
        let mut compact_bytes = value_bytes;
        while !compact_bytes.is_empty() && compact_bytes[0] == 0 {
            compact_bytes.remove(0);
        }
        if compact_bytes.is_empty() {
            compact_bytes = vec![0];
        }
        s.append(&compact_bytes);
    }
}

impl Signable for TokenAuthorityPayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        keccak256(&encoded)
    }
}

/// Token authority request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenAuthorityRequest {
    #[serde(flatten)]
    pub payload: TokenAuthorityPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token operation response.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenOperationResponse {
    /// Transaction hash.
    pub hash: String,
}

/// Pause action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PauseAction {
    /// Pause token operations.
    Pause,
    /// Unpause token operations.
    Unpause,
}

/// Token pause payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub token: OneMoneyAddress,
}

impl Encodable for TokenPausePayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&format!("{:?}", self.action));
        s.append(&self.token.as_slice());
    }
}

impl Signable for TokenPausePayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        keccak256(&encoded)
    }
}

/// Token pause request.
#[derive(Debug, Clone, Serialize)]
pub struct PauseTokenRequest {
    #[serde(flatten)]
    pub payload: TokenPausePayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Blacklist action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum BlacklistAction {
    /// Add address to blacklist.
    Add,
    /// Remove address from blacklist.
    Remove,
}

/// Token blacklist management payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub address: OneMoneyAddress,
    /// Token address.
    pub token: OneMoneyAddress,
}

impl Encodable for TokenBlacklistPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&format!("{:?}", self.action));
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

/// Token blacklist request.
#[derive(Debug, Clone, Serialize)]
pub struct BlacklistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenBlacklistPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Whitelist action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum WhitelistAction {
    /// Add address to whitelist.
    Add,
    /// Remove address from whitelist.
    Remove,
}

/// Token whitelist management payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub address: OneMoneyAddress,
    /// Token address.
    pub token: OneMoneyAddress,
}

impl Encodable for TokenWhitelistPayload {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.recent_epoch);
        s.append(&self.recent_checkpoint);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&format!("{:?}", self.action));
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

/// Token whitelist request.
#[derive(Debug, Clone, Serialize)]
pub struct WhitelistTokenRequest {
    #[serde(flatten)]
    pub payload: TokenWhitelistPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Token metadata update payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub token: OneMoneyAddress,
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
                                          // Manually encode Vec<MetadataKVPair> as nested RLP list
        let mut metadata_stream = rlp::RlpStream::new_list(self.additional_metadata.len());
        for metadata_item in &self.additional_metadata {
            metadata_stream.append(metadata_item);
        }
        s.append_raw(&metadata_stream.out(), 1);
    }
}

impl Signable for TokenMetadataUpdatePayload {
    fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        keccak256(&encoded)
    }
}

/// Token metadata update request.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMetadataRequest {
    #[serde(flatten)]
    pub payload: TokenMetadataUpdatePayload,
    /// Signature for the payload.
    pub signature: Signature,
}

impl Client {
    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token mint parameters
    /// * `private_key` - Private key for signing the transaction (must have mint authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn mint_token(
        &self,
        payload: TokenMintPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = MintTokenRequest { payload, signature };

        self.post(&api_path(MINT), &request).await
    }

    /// Burn tokens from an account.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token burn parameters
    /// * `private_key` - Private key for signing the transaction (must have burn authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn burn_token(
        &self,
        payload: TokenBurnPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = BurnTokenRequest { payload, signature };

        self.post(&api_path(BURN), &request).await
    }

    /// Grant authority for a token to an address.
    ///
    /// # Arguments
    ///
    /// * `payload` - Authority grant parameters
    /// * `private_key` - Private key for signing the transaction (must have master authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn grant_authority(
        &self,
        payload: TokenAuthorityPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenAuthorityRequest { payload, signature };

        self.post(&api_path(GRANT_AUTHORITY), &request).await
    }

    /// Revoke authority for a token from an address.
    ///
    /// # Arguments
    ///
    /// * `payload` - Authority revoke parameters
    /// * `private_key` - Private key for signing the transaction (must have master authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn revoke_authority(
        &self,
        payload: TokenAuthorityPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenAuthorityRequest { payload, signature };

        self.post(&api_path(REVOKE_AUTHORITY), &request).await
    }

    /// Get token metadata by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint_address` - The token mint address
    ///
    /// # Returns
    ///
    /// The token metadata.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::{Client, OneMoneyAddress};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///     let mint = OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    ///
    ///     let mint_info = client.get_token_metadata(mint).await?;
    ///     println!("Token: {}", mint_info.symbol);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_token_metadata(&self, mint_address: OneMoneyAddress) -> Result<MintInfo> {
        let path = api_path(&format!("{}?token={}", TOKEN_METADATA, mint_address));
        let response: MintInfo = self.get(&path).await?;
        Ok(response)
    }

    /// Pause or unpause a token.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token pause parameters
    /// * `private_key` - Private key for signing the transaction (must have pause authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn pause_token(
        &self,
        payload: TokenPausePayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = PauseTokenRequest { payload, signature };

        self.post(&api_path(PAUSE), &request).await
    }

    /// Manage token blacklist (add or remove addresses).
    ///
    /// # Arguments
    ///
    /// * `payload` - Token blacklist management parameters
    /// * `private_key` - Private key for signing the transaction (must have manage list authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn manage_blacklist(
        &self,
        payload: TokenBlacklistPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = BlacklistTokenRequest { payload, signature };

        self.post(&api_path(MANAGE_BLACKLIST), &request).await
    }

    /// Manage token whitelist (add or remove addresses).
    ///
    /// # Arguments
    ///
    /// * `payload` - Token whitelist management parameters
    /// * `private_key` - Private key for signing the transaction (must have manage list authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn manage_whitelist(
        &self,
        payload: TokenWhitelistPayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = WhitelistTokenRequest { payload, signature };

        self.post(&api_path(MANAGE_WHITELIST), &request).await
    }

    /// Update token metadata.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token metadata update parameters
    /// * `private_key` - Private key for signing the transaction (must have update metadata authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn update_token_metadata(
        &self,
        payload: TokenMetadataUpdatePayload,
        private_key: &str,
    ) -> Result<TokenOperationResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = UpdateMetadataRequest { payload, signature };

        self.post(&api_path(UPDATE_METADATA), &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authority_values() {
        assert_eq!(
            serde_json::to_string(&Authority::MasterMintBurn).unwrap(),
            "\"MasterMintBurn\""
        );
        assert_eq!(
            serde_json::to_string(&Authority::MintBurnTokens).unwrap(),
            "\"MintBurnTokens\""
        );
        assert_eq!(
            serde_json::to_string(&Authority::Pause).unwrap(),
            "\"Pause\""
        );
        assert_eq!(
            serde_json::to_string(&Authority::ManageList).unwrap(),
            "\"ManageList\""
        );
        assert_eq!(
            serde_json::to_string(&Authority::UpdateMetadata).unwrap(),
            "\"UpdateMetadata\""
        );
    }
}
