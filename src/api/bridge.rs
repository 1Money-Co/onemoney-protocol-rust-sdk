//! Bridge-related API operations.

use crate::Result;
use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::bridge::BRIDGE_AND_MINT;
use crate::crypto::sign_transaction_payload;
use crate::requests::{TokenBridgeAndMintPayload, TokenBridgeAndMintRequest};
use crate::responses::TransactionResponse;

impl Client {
    /// Bridge and mint tokens from another chain.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token bridge and mint parameters
    /// * `private_key` - Private key for signing the transaction
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn bridge_and_mint(
        &self,
        payload: TokenBridgeAndMintPayload,
        private_key: &str,
    ) -> Result<TransactionResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenBridgeAndMintRequest {
            data: payload,
            signature,
        };

        self.post(&api_path(BRIDGE_AND_MINT), &request).await
    }
}
