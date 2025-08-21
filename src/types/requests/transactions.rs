//! Transaction-related API request types.

use crate::crypto::Signable;
use alloy_primitives::{Address, B256, U256};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
fn serialize_token_amount_decimal<S>(
    value: &U256,
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
#[derive(Debug, Clone, Serialize)]
pub struct FeeEstimateRequest {
    /// From address.
    pub from: Address,
    /// Value to transfer.
    pub value: Option<U256>,
    /// Token address (optional).
    pub token: Option<Address>,
}
