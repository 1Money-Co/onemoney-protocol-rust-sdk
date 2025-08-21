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
    use crate::Authority;

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
