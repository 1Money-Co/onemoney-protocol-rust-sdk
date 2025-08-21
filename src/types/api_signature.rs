//! REST API signature types matching L1 implementation.

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Signature type for REST requests.
///
/// We use this type to avoid the ambiguity of the signature type in the core
/// primitives.
///
/// This type is referred to `https://github.com/alloy-rs/alloy/blob/b2278c40b2693908e4e5108d65ade26e8d716765/crates/rpc-types-eth/src/transaction/signature.rs#L9`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RestSignature {
    /// The R field of the signature; the point on the curve.
    pub r: U256,
    /// The S field of the signature; the point on the curve.
    pub s: U256,
    // See <https://github.com/ethereum/go-ethereum/issues/27727> for more information
    /// For EIP-155, EIP-2930 and Blob transactions this is set to the parity (0
    /// for even, 1 for odd) of the y-value of the secp256k1 signature.
    ///
    /// For legacy transactions, this is the recovery id
    ///
    /// See also <https://ethereum.github.io/execution-apis/api-documentation/> and <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionbyhash>
    pub v: u64,
}

impl Display for RestSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "RestSignature {{ r: {}, s: {}, v: {} }}",
            self.r, self.s, self.v
        )
    }
}

impl From<crate::Signature> for RestSignature {
    fn from(signature: crate::Signature) -> Self {
        Self {
            v: signature.v,
            r: signature.r,
            s: signature.s,
        }
    }
}

impl From<RestSignature> for crate::Signature {
    fn from(signature: RestSignature) -> Self {
        crate::Signature::new(signature.r, signature.s, signature.v)
    }
}
