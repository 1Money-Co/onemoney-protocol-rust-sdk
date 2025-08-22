//! Token-related API operations.

use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::tokens::{
    BURN, GRANT_AUTHORITY, MANAGE_BLACKLIST, MANAGE_WHITELIST, MINT, PAUSE, TOKEN_METADATA,
    UPDATE_METADATA,
};
use crate::crypto::sign_transaction_payload;
use crate::requests::{
    BlacklistTokenRequest, BurnTokenRequest, MintTokenRequest, PauseTokenRequest,
    TokenAuthorityPayload, TokenAuthorityRequest, TokenBlacklistPayload, TokenBurnPayload,
    TokenMetadataUpdatePayload, TokenMintPayload, TokenPausePayload, TokenWhitelistPayload,
    UpdateMetadataRequest, WhitelistTokenRequest,
};
use crate::responses::MintInfo;
use crate::{Hash, Result};
use alloy_primitives::Address;

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
    pub async fn mint_token(&self, payload: TokenMintPayload, private_key: &str) -> Result<Hash> {
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
    pub async fn burn_token(&self, payload: TokenBurnPayload, private_key: &str) -> Result<Hash> {
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
    ) -> Result<Hash> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenAuthorityRequest { payload, signature };

        self.post(&api_path(GRANT_AUTHORITY), &request).await
    }

    /// Revoke authority for a token from an address.
    ///
    /// Note: This method uses the same `/v1/tokens/grant_authority` endpoint as grant_authority(),
    /// but with `AuthorityAction::Revoke` in the payload to indicate a revoke operation.
    ///
    /// # Arguments
    ///
    /// * `payload` - Authority revoke parameters (with action set to AuthorityAction::Revoke)
    /// * `private_key` - Private key for signing the transaction (must have master authority)
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn revoke_authority(
        &self,
        payload: TokenAuthorityPayload,
        private_key: &str,
    ) -> Result<Hash> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenAuthorityRequest { payload, signature };

        self.post(&api_path(GRANT_AUTHORITY), &request).await
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
    /// use onemoney_protocol::Client;
    /// use alloy_primitives::Address;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///     let mint = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    ///
    ///     let mint_info = client.get_token_metadata(mint).await?;
    ///     println!("Token: {}", mint_info.symbol);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_token_metadata(&self, mint_address: Address) -> Result<MintInfo> {
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
    pub async fn pause_token(&self, payload: TokenPausePayload, private_key: &str) -> Result<Hash> {
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
    ) -> Result<Hash> {
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
    ) -> Result<Hash> {
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
    ) -> Result<Hash> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = UpdateMetadataRequest { payload, signature };

        self.post(&api_path(UPDATE_METADATA), &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Authority, AuthorityAction, BlacklistAction, PauseAction, WhitelistAction};
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

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

    #[test]
    fn test_token_mint_payload_structure() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap();
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenMintPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value: U256::from(1000000000000000000u64),
            token,
        };

        assert_eq!(payload.recent_epoch, 100);
        assert_eq!(payload.recent_checkpoint, 200);
        assert_eq!(payload.chain_id, 1212101);
        assert_eq!(payload.nonce, 5);
        assert_eq!(payload.recipient, address);
        assert_eq!(payload.value, U256::from(1000000000000000000u64));
        assert_eq!(payload.token, token);
    }

    #[test]
    fn test_token_burn_payload_structure() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap();
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenBurnPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            recipient: address,
            value: U256::from(500000000000000000u64),
            token,
        };

        assert_eq!(payload.recent_epoch, 100);
        assert_eq!(payload.recipient, address);
        assert_eq!(payload.value, U256::from(500000000000000000u64));
    }

    #[test]
    fn test_authority_action_serialization() {
        assert_eq!(
            serde_json::to_string(&AuthorityAction::Grant).unwrap(),
            "\"Grant\""
        );
        assert_eq!(
            serde_json::to_string(&AuthorityAction::Revoke).unwrap(),
            "\"Revoke\""
        );
    }

    #[test]
    fn test_pause_action_serialization() {
        assert_eq!(
            serde_json::to_string(&PauseAction::Pause).unwrap(),
            "\"Pause\""
        );
        assert_eq!(
            serde_json::to_string(&PauseAction::Unpause).unwrap(),
            "\"Unpause\""
        );
    }

    #[test]
    fn test_blacklist_action_serialization() {
        assert_eq!(
            serde_json::to_string(&BlacklistAction::Add).unwrap(),
            "\"Add\""
        );
        assert_eq!(
            serde_json::to_string(&BlacklistAction::Remove).unwrap(),
            "\"Remove\""
        );
    }

    #[test]
    fn test_whitelist_action_serialization() {
        assert_eq!(
            serde_json::to_string(&WhitelistAction::Add).unwrap(),
            "\"Add\""
        );
        assert_eq!(
            serde_json::to_string(&WhitelistAction::Remove).unwrap(),
            "\"Remove\""
        );
    }

    #[test]
    fn test_token_authority_payload_structure() {
        let authority_address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap();
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenAuthorityPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            action: AuthorityAction::Grant,
            authority_type: Authority::MintBurnTokens,
            authority_address,
            token,
            value: U256::from(1000000000000000000u64),
        };

        assert_eq!(payload.action, AuthorityAction::Grant);
        assert_eq!(payload.authority_type, Authority::MintBurnTokens);
        assert_eq!(payload.authority_address, authority_address);
    }

    #[test]
    fn test_token_pause_payload_structure() {
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenPausePayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            action: PauseAction::Pause,
            token,
        };

        assert_eq!(payload.action, PauseAction::Pause);
        assert_eq!(payload.token, token);
    }

    #[test]
    fn test_token_blacklist_payload_structure() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap();
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenBlacklistPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            action: BlacklistAction::Add,
            address,
            token,
        };

        assert_eq!(payload.action, BlacklistAction::Add);
        assert_eq!(payload.address, address);
        assert_eq!(payload.token, token);
    }

    #[test]
    fn test_token_whitelist_payload_structure() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap();
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();

        let payload = TokenWhitelistPayload {
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            action: WhitelistAction::Add,
            address,
            token,
        };

        assert_eq!(payload.action, WhitelistAction::Add);
        assert_eq!(payload.address, address);
        assert_eq!(payload.token, token);
    }
}
