//! Transaction-related API operations.

use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::transactions::{
    BY_HASH, ESTIMATE_FEE, PAYMENT, RECEIPT_BY_HASH,
};
use crate::crypto::sign_transaction_payload;
use crate::requests::{FeeEstimateRequest, PaymentPayload, PaymentRequest};
use crate::responses::FeeEstimate;
use crate::responses::TransactionReceipt;
use crate::{Hash, Result, Transaction};
#[cfg(test)]
use rlp::encode as rlp_encode;

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
    ///     let client = Client::mainnet();
    ///
    ///     let payload = PaymentPayload {
    ///         recent_epoch: 123,
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
    pub async fn send_payment(&self, payload: PaymentPayload, private_key: &str) -> Result<Hash> {
        // Use the L1-compatible signing method
        let signature = sign_transaction_payload(&payload, private_key)?;
        let request = PaymentRequest { payload, signature };

        self.post(&api_path(PAYMENT), &request).await
    }

    /// Get transaction by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The transaction hash to query
    ///
    /// # Returns
    ///
    /// The transaction information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///
    ///     let tx_hash = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    ///     let transaction = client.get_transaction_by_hash(tx_hash).await?;
    ///
    ///     println!("Transaction hash: {}", transaction.hash);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transaction_by_hash(&self, hash: &str) -> Result<Transaction> {
        let path = api_path(&format!("{}?hash={}", BY_HASH, hash));
        self.get(&path).await
    }

    /// Estimate fees for a transaction.
    ///
    /// # Arguments
    ///
    /// * `request` - Fee estimation request parameters
    ///
    /// # Returns
    ///
    /// The fee estimate information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::{Client, FeeEstimateRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///
    ///     let request = FeeEstimateRequest {
    ///         from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
    ///         value: "1000000000000000000".to_string(),
    ///         token: None,
    ///     };
    ///
    ///     let estimate = client.estimate_fee(request).await?;
    ///     println!("Estimated fee: {}", estimate.fee);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn estimate_fee(&self, request: FeeEstimateRequest) -> Result<FeeEstimate> {
        let mut path = ESTIMATE_FEE.to_string();
        let mut query_params = Vec::new();

        query_params.push(format!("from={}", request.from));
        query_params.push(format!("value={}", request.value));
        if let Some(token) = request.token {
            query_params.push(format!("token={}", token));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get(&api_path(&path)).await
    }

    /// Get transaction receipt by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The transaction hash to query
    ///
    /// # Returns
    ///
    /// The transaction receipt information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///
    ///     let tx_hash = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    ///     let receipt = client.get_transaction_receipt_by_hash(tx_hash).await?;
    ///
    ///     println!("Transaction success: {}", receipt.success);
    ///     println!("Fee used: {}", receipt.fee_used);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transaction_receipt_by_hash(&self, hash: &str) -> Result<TransactionReceipt> {
        let path = api_path(&format!("{}?hash={}", RECEIPT_BY_HASH, hash));
        self.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_payment_payload_rlp() {
        let payload = PaymentPayload {
            recent_epoch: 123,
            recent_checkpoint: 456,
            chain_id: 1212101,
            nonce: 0,
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: U256::from(1000000000000000000u64),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let encoded = rlp_encode(&payload);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_fee_estimate_request() {
        let request = FeeEstimateRequest {
            from: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0".to_string(),
            value: "1000000000000000000".to_string(),
            token: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"));
    }
}
