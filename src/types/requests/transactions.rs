//! Transaction-related API request types.

use crate::crypto::Signable;
use alloy_primitives::{Address, B256, U256};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;

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

/// Payment transaction payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub recipient: Address,
    /// Amount to transfer.
    #[serde(serialize_with = "serialize_token_amount_decimal")]
    pub value: U256,
    /// Token address (use native token address for native transfers).
    pub token: Address,
}

/// Serialize U256 as decimal string instead of hex (L1 compatibility).
fn serialize_token_amount_decimal<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
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
        let compact_bytes = compact_u256_bytes(&self.value);
        s.append(&compact_bytes);
        s.append(&self.token.as_slice());
    }
}

impl PaymentPayload {
    /// Calculate the signature hash for this payload.
    /// This matches the L1 implementation's signature_hash method.
    pub fn signature_hash(&self) -> B256 {
        let encoded = rlp::encode(self);
        alloy_primitives::keccak256(&encoded)
    }
}

impl Signable for PaymentPayload {
    fn signature_hash(&self) -> B256 {
        PaymentPayload::signature_hash(self)
    }
}

/// Payment transaction request.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    #[serde(flatten)]
    pub payload: PaymentPayload,
    /// Signature for the payload.
    pub signature: crate::Signature,
}

/// Fee estimation request.
/// Matches L1 server's EstimateFeeRequest structure with string query parameters.
#[derive(Debug, Clone, Serialize)]
pub struct FeeEstimateRequest {
    /// From address (as string for query parameter).
    pub from: String,
    /// Value to transfer (as string for query parameter).
    pub value: String,
    /// Token address (optional, as string for query parameter).
    pub token: Option<String>,
}
