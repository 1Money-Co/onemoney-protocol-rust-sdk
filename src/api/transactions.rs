//! Transaction-related API operations.

use crate::client::Client;
use crate::client::config::endpoints::transactions::{
    BY_HASH, ESTIMATE_FEE, PAYMENT, RECEIPT_BY_HASH,
};
use crate::client::config::{API_VERSION, api_path};
use crate::client::endpoints::transactions::FINALIZED_BY_HASH;
use crate::crypto::sign_transaction_payload;
use crate::requests::{FeeEstimateRequest, PaymentPayload, PaymentRequest};
use crate::responses::FeeEstimate;
use crate::responses::TransactionReceipt;
use crate::responses::TransactionResponse;
use crate::{FinalizedTransaction, Result, Transaction};

impl Client {
    /// Send a payment transaction.
    ///
    /// # Arguments
    ///
    /// * `payload` - Payment transaction parameters
    /// * `private_key` - Private key for signing the transaction
    ///
    /// # Returns
    ///
    /// The payment response containing the transaction hash.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::{Client, PaymentPayload};
    /// use alloy_primitives::{Address, U256};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet()?;
    ///
    ///     let payload = PaymentPayload {
    ///         recent_checkpoint: 456,
    ///         chain_id: 1212101,
    ///         nonce: 0,
    ///         recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?,
    ///         value: U256::from(1000000000000000000u64), // 1 token
    ///         token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
    ///     };
    ///
    ///     let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    ///     let result = client.send_payment(payload, private_key).await?;
    ///     println!("Transaction hash: {}", result.hash);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_payment(
        &self,
        payload: PaymentPayload,
        private_key: &str,
    ) -> Result<TransactionResponse> {
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = PaymentRequest { payload, signature };

        let path = api_path(PAYMENT);
        self.post(&path, &request).await
    }

    /// Get transaction by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - Transaction hash
    ///
    /// # Returns
    ///
    /// The transaction details.
    pub async fn get_transaction_by_hash(&self, hash: &str) -> Result<Transaction> {
        let path = format!("{}{}?hash={}", API_VERSION, BY_HASH, hash);
        self.get(&path).await
    }

    /// Get transaction receipt by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - Transaction hash
    ///
    /// # Returns
    ///
    /// The transaction receipt.
    pub async fn get_transaction_receipt_by_hash(&self, hash: &str) -> Result<TransactionReceipt> {
        let path = format!("{}{}?hash={}", API_VERSION, RECEIPT_BY_HASH, hash);
        self.get(&path).await
    }

    /// Estimate transaction fee.
    ///
    /// # Arguments
    ///
    /// * `request` - Fee estimation parameters
    ///
    /// # Returns
    ///
    /// The estimated fee.
    pub async fn estimate_fee(&self, request: FeeEstimateRequest) -> Result<FeeEstimate> {
        let path = api_path(ESTIMATE_FEE);
        // Build query string manually
        let token_query = match request.token {
            Some(ref token) => format!("&token={}", token),
            None => String::new(),
        };
        let full_path = format!(
            "{}?from={}&value={}{}",
            path, request.from, request.value, token_query
        );
        self.get(&full_path).await
    }

    /// Get finalized transaction and receipt by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - Transaction hash
    ///
    /// # Returns
    ///
    /// The finalized transaction and receipt.
    pub async fn get_finalized_transaction_by_hash(
        &self,
        hash: &str,
    ) -> Result<FinalizedTransaction> {
        let path = format!("{}{}?hash={}", API_VERSION, FINALIZED_BY_HASH, hash);
        self.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_payment_payload_alloy_rlp() {
        use alloy_rlp::Encodable as AlloyEncodable;

        let payload = PaymentPayload {
            recent_checkpoint: 456,
            chain_id: 1212101,
            nonce: 0,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let mut encoded = Vec::new();
        payload.encode(&mut encoded);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_fee_estimate_request() {
        let request = FeeEstimateRequest {
            from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
            value: "1000000000000000000".to_string(),
            token: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
        };

        // Test serialization
        let json = serde_json::to_string(&request).expect("Should serialize");
        assert!(json.contains("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
        assert!(json.contains("1000000000000000000"));
        assert!(json.contains("0x1234567890abcdef1234567890abcdef12345678"));
    }

    #[test]
    fn test_finalized_transaction_api_path_construction() {
        use crate::client::endpoints::transactions::FINALIZED_BY_HASH;

        let hash = "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777";
        let expected_path = format!("{}{}?hash={}", API_VERSION, FINALIZED_BY_HASH, hash);

        assert!(expected_path.contains("/v1"));
        assert!(expected_path.contains("/transactions/finalized/by_hash"));
        assert!(
            expected_path.contains(
                "hash=0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"
            )
        );
    }

    #[test]
    fn test_finalized_transaction_structure() {
        use crate::responses::TransactionReceipt;
        use crate::{FinalizedTransaction, Signature};
        use alloy_primitives::{Address, B256};

        let finalized_tx = FinalizedTransaction {
            epoch: 100,
            receipt: TransactionReceipt {
                success: true,
                transaction_hash: B256::from_str(
                    "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
                )
                .expect("Test data should be valid"),
                transaction_index: Some(5),
                checkpoint_hash: Some(
                    B256::from_str(
                        "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                    )
                    .expect("Test data should be valid"),
                ),
                checkpoint_number: Some(1500),
                fee_used: 1000000,
                from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                    .expect("Test data should be valid"),
                recipient: Some(
                    Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                        .expect("Test data should be valid"),
                ),
                token_address: None,
            },
            counter_signatures: vec![Signature::default(), Signature::default()],
        };

        assert_eq!(finalized_tx.epoch, 100);
        assert!(finalized_tx.receipt.success);
        assert_eq!(finalized_tx.counter_signatures.len(), 2);
        assert_eq!(finalized_tx.receipt.fee_used, 1000000);
    }

    #[test]
    fn test_finalized_transaction_json_output() {
        use crate::responses::TransactionReceipt;
        use crate::{FinalizedTransaction, Signature};
        use alloy_primitives::{Address, B256};

        let finalized_tx = FinalizedTransaction {
            epoch: 200,
            receipt: TransactionReceipt {
                success: true,
                transaction_hash: B256::from_str(
                    "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
                )
                .expect("Test data should be valid"),
                transaction_index: Some(0),
                checkpoint_hash: Some(
                    B256::from_str(
                        "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                    )
                    .expect("Test data should be valid"),
                ),
                checkpoint_number: Some(1500),
                fee_used: 1000000,
                from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                    .expect("Test data should be valid"),
                recipient: Some(
                    Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                        .expect("Test data should be valid"),
                ),
                token_address: Some(
                    Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                        .expect("Test data should be valid"),
                ),
            },
            counter_signatures: vec![Signature::default()],
        };

        let json = serde_json::to_string(&finalized_tx).expect("Should serialize to JSON");

        assert!(json.contains("\"epoch\":200"));
        assert!(
            json.contains("\"transaction_hash\":\"0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777\"")
        );
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"fee_used\":1000000"));
        assert!(json.contains("\"counter_signatures\""));
    }
}
