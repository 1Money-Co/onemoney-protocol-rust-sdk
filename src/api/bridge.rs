//! Bridge-related API operations.

use crate::Result;
use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::bridge::{BRIDGE_AND_MINT, BURN_AND_BRIDGE};
use crate::crypto::sign_transaction_payload;
use crate::requests::{
    TokenBridgeAndMintPayload, TokenBridgeAndMintRequest, TokenBurnAndBridgePayload,
    TokenBurnAndBridgeRequest,
};
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

    /// Burn and bridge tokens to another chain.
    ///
    /// # Arguments
    ///
    /// * `payload` - Token burn and bridge parameters
    /// * `private_key` - Private key for signing the transaction
    ///
    /// # Returns
    ///
    /// The transaction result.
    pub async fn burn_and_bridge(
        &self,
        payload: TokenBurnAndBridgePayload,
        private_key: &str,
    ) -> Result<TransactionResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = TokenBurnAndBridgeRequest {
            data: payload,
            signature,
        };

        self.post(&api_path(BURN_AND_BRIDGE), &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::config::endpoints::bridge::{BRIDGE_AND_MINT, BURN_AND_BRIDGE};
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_bridge_and_mint_api_path_construction() {
        let path = api_path(BRIDGE_AND_MINT);
        assert_eq!(path, "/v1/tokens/bridge_and_mint");
    }

    #[test]
    fn test_burn_and_bridge_api_path_construction() {
        let path = api_path(BURN_AND_BRIDGE);
        assert_eq!(path, "/v1/tokens/burn_and_bridge");
    }

    #[test]
    fn test_token_bridge_and_mint_payload_structure() {
        let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");

        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 100,
            chain_id: 1212101,
            nonce: 1,
            recipient,
            value: U256::from(5000000000000000000u64),
            token,
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        assert_eq!(payload.recent_checkpoint, 100);
        assert_eq!(payload.chain_id, 1212101);
        assert_eq!(payload.nonce, 1);
        assert_eq!(payload.recipient, recipient);
        assert_eq!(payload.value, U256::from(5000000000000000000u64));
        assert_eq!(payload.token, token);
        assert_eq!(payload.source_chain_id, 1);
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
            value: U256::from(3000000000000000000u64),
            token,
            destination_chain_id: 1,
            destination_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bridge_param: None,
        };

        assert_eq!(payload.recent_checkpoint, 200);
        assert_eq!(payload.chain_id, 1212101);
        assert_eq!(payload.nonce, 5);
        assert_eq!(payload.sender, sender);
        assert_eq!(payload.value, U256::from(3000000000000000000u64));
        assert_eq!(payload.token, token);
        assert_eq!(payload.destination_chain_id, 1);
    }

    #[test]
    fn test_bridge_and_mint_payload_alloy_rlp_encoding() {
        use alloy_rlp::Encodable as AlloyEncodable;

        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 100,
            chain_id: 1212101,
            nonce: 1,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(5000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);
        assert!(!encoded.is_empty());
        assert!(encoded.len() > 100);
    }

    #[test]
    fn test_burn_and_bridge_payload_alloy_rlp_encoding() {
        use alloy_rlp::Encodable as AlloyEncodable;

        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 5,
            sender: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(3000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            destination_chain_id: 1,
            destination_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bridge_param: None,
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);
        assert!(!encoded.is_empty());
        assert!(encoded.len() > 100);
    }

    #[test]
    fn test_bridge_payloads_different_encodings() {
        use alloy_rlp::Encodable as AlloyEncodable;

        let payload1 = TokenBridgeAndMintPayload {
            recent_checkpoint: 100,
            chain_id: 1212101,
            nonce: 1,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(5000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let payload2 = TokenBridgeAndMintPayload {
            recent_checkpoint: 200,
            chain_id: 1212101,
            nonce: 2,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(5000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let mut encoded1 = Vec::new();
        payload1.encode(&mut encoded1);

        let mut encoded2 = Vec::new();
        payload2.encode(&mut encoded2);

        assert_ne!(encoded1, encoded2);
    }

    #[test]
    fn test_bridge_payload_encoding_deterministic() {
        use alloy_rlp::Encodable as AlloyEncodable;

        let payload = TokenBurnAndBridgePayload {
            recent_checkpoint: 100,
            chain_id: 1212101,
            nonce: 1,
            sender: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            destination_chain_id: 1,
            destination_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            escrow_fee: U256::from(1000000u64),
            bridge_metadata: None,
            bridge_param: None,
        };

        let mut encoded1 = Vec::new();
        payload.encode(&mut encoded1);

        let mut encoded2 = Vec::new();
        payload.encode(&mut encoded2);

        assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_bridge_request_structure() {
        use crate::types::common::Signature;

        let payload = TokenBridgeAndMintPayload {
            recent_checkpoint: 100,
            chain_id: 1212101,
            nonce: 1,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(5000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            source_chain_id: 1,
            source_tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            bridge_metadata: None,
        };

        let signature = Signature::new(
            U256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .expect("Test data should be valid"),
            U256::from_str("0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321")
                .expect("Test data should be valid"),
            27,
        );

        let request = TokenBridgeAndMintRequest {
            data: payload.clone(),
            signature: signature.clone(),
        };

        assert_eq!(request.data.chain_id, 1212101);
        assert_eq!(request.signature.v, 27);
    }
}
