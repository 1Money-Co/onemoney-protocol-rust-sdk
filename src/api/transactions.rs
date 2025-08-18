//! Transaction-related API operations.

use super::client::Client;
use crate::api::client::api_path;
use crate::api::client::endpoints::transactions::{
    BY_HASH, ESTIMATE_FEE, PAYMENT, RECEIPT_BY_HASH,
};
use crate::crypto::Signable;
use crate::crypto::sign_transaction_payload;
use crate::{OneMoneyAddress, Result, Signature, TokenAmount, Transaction};
use alloy_primitives::{B256, keccak256};
#[cfg(test)]
use rlp::encode as rlp_encode;
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Payment transaction payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPayload {
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
    /// Amount to transfer.
    #[serde(serialize_with = "serialize_token_amount_decimal")]
    pub value: TokenAmount,
    /// Token address (use native token address for native transfers).
    pub token: OneMoneyAddress,
}

/// Serialize TokenAmount as decimal string instead of hex (L1 compatibility).
fn serialize_token_amount_decimal<S>(
    value: &TokenAmount,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

impl Display for PaymentPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Payment to {}: value {}, token {}, nonce {}, epoch {}, checkpoint {}, chain {}",
            self.recipient,
            self.value,
            self.token,
            self.nonce,
            self.recent_epoch,
            self.recent_checkpoint,
            self.chain_id
        )
    }
}

impl Encodable for PaymentPayload {
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

impl PaymentPayload {
    /// Calculate the signature hash for this payload.
    /// This matches the L1 implementation's signature_hash method.
    pub fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        keccak256(&encoded)
    }
}

impl Signable for PaymentPayload {
    fn signature_hash(&self) -> B256 {
        self.signature_hash()
    }
}

/// Payment transaction request.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    #[serde(flatten)]
    pub payload: PaymentPayload,
    /// Signature for the payload.
    pub signature: Signature,
}

/// Fee estimation request.
#[derive(Debug, Clone, Serialize)]
pub struct FeeEstimateRequest {
    /// From address.
    pub from: OneMoneyAddress,
    /// Value to transfer.
    pub value: Option<TokenAmount>,
    /// Token address (optional).
    pub token: Option<OneMoneyAddress>,
}

/// Payment response (matches server's Hash type).
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    /// Transaction hash.
    pub hash: String,
}

impl Display for PaymentResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Transaction hash: {}", self.hash)
    }
}

/// Transaction receipt response.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionReceipt {
    /// If transaction is executed successfully.
    pub success: bool,
    /// Transaction Hash.
    pub transaction_hash: String,
    /// Index within the block.
    pub transaction_index: Option<u64>,
    /// Hash of the checkpoint this transaction was included within.
    pub checkpoint_hash: Option<String>,
    /// Number of the checkpoint this transaction was included within.
    pub checkpoint_number: Option<u64>,
    /// Fee used.
    pub fee_used: u128,
    /// Address of the sender.
    pub from: String,
    /// Address of the receiver. None when its a contract creation transaction.
    pub to: Option<String>,
    /// Token address created, or None if not a deployment.
    pub token_address: Option<String>,
}

impl Display for TransactionReceipt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Transaction Receipt:")?;
        writeln!(f, "  Success: {}", self.success)?;
        writeln!(f, "  Transaction Hash: {}", self.transaction_hash)?;
        writeln!(f, "  Fee Used: {}", self.fee_used)?;
        if let Some(idx) = self.transaction_index {
            writeln!(f, "  Transaction Index: {}", idx)?;
        }
        if let Some(hash) = &self.checkpoint_hash {
            writeln!(f, "  Checkpoint Hash: {}", hash)?;
        }
        if let Some(num) = self.checkpoint_number {
            writeln!(f, "  Checkpoint Number: {}", num)?;
        }
        writeln!(f, "  From: {}", self.from)?;
        if let Some(to) = &self.to {
            writeln!(f, "  To: {}", to)?;
        }
        if let Some(token) = &self.token_address {
            write!(f, "  Token Address: {}", token)?;
        }
        Ok(())
    }
}

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
    /// use onemoney_protocol::{Client, PaymentPayload, OneMoneyAddress, TokenAmount};
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
    ///         recipient: OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?,
    ///         value: TokenAmount::from(1000000000000000000u64), // 1 token
    ///         token: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?,
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
    ) -> Result<PaymentResponse> {
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
    /// use onemoney_protocol::{Client, FeeEstimateRequest, OneMoneyAddress, TokenAmount};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///
    ///     let request = FeeEstimateRequest {
    ///         from: OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?,
    ///         value: Some(TokenAmount::from(1000000000000000000u64)),
    ///         token: None,
    ///     };
    ///
    ///     let estimate = client.estimate_fee(request).await?;
    ///     println!("Estimated fee: {}", estimate.total_fee);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn estimate_fee(&self, request: FeeEstimateRequest) -> Result<crate::FeeEstimate> {
        let mut path = ESTIMATE_FEE.to_string();
        let mut query_params = Vec::new();

        query_params.push(format!("from={}", request.from));
        if let Some(value) = request.value {
            query_params.push(format!("value={}", value));
        }
        if let Some(token) = request.token {
            query_params.push(format!("token={}", token));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        #[derive(Deserialize)]
        struct EstimateFeeResponse {
            fee: String,
        }

        let response: EstimateFeeResponse = self.get(&api_path(&path)).await?;
        // Parse the fee string as TokenAmount
        let fee_amount = response
            .fee
            .parse::<u128>()
            .map_err(|_| crate::Error::custom("Invalid fee format from API".to_string()))?;

        Ok(crate::FeeEstimate {
            gas_limit: 21000, // Default gas limit for simple transactions
            gas_price: TokenAmount::from(fee_amount / 21000), // Calculated gas price
            total_fee: TokenAmount::from(fee_amount),
        })
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
    use std::str::FromStr;

    #[test]
    fn test_payment_payload_rlp() {
        let payload = PaymentPayload {
            recent_epoch: 123,
            recent_checkpoint: 456,
            chain_id: 1212101,
            nonce: 0,
            recipient: OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .unwrap(),
            value: TokenAmount::from(1000000000000000000u64),
            token: OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
        };

        let encoded = rlp_encode(&payload);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_fee_estimate_request() {
        let request = FeeEstimateRequest {
            from: OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").unwrap(),
            value: Some(TokenAmount::from(1000000000000000000u64)),
            token: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0"));
    }
}
