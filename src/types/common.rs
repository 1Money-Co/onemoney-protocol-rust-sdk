//! Common types used throughout the OneMoney SDK.

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// ECDSA signature components.
///
/// Compatible with REST API and L1 implementation signature format.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature {
    /// The R field of the signature; a scalar (U256) representing the x-coordinate-derived component of the signature.
    pub r: U256,
    /// The S field of the signature; a scalar (U256) representing the multiplicative component of the signature.
    pub s: U256,
    /// For EIP-155, EIP-2930 and Blob transactions this is set to the parity (0
    /// for even, 1 for odd) of the y-value of the secp256k1 signature.
    ///
    /// For legacy transactions, this is the recovery id.
    pub v: u64,
}

impl Signature {
    /// Create a new signature from components.
    pub fn new(r: U256, s: U256, v: u64) -> Self {
        Self { r, s, v }
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Signature(r: {}, s: {}, v: {})", self.r, self.s, self.v)
    }
}

/// Transaction action types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// Payment transaction.
    Payment,
    /// Token issuance.
    TokenIssue,
    /// Token minting.
    TokenMint,
    /// Token burning.
    TokenBurn,
    /// Authority grant.
    AuthorityGrant,
    /// Authority revoke.
    AuthorityRevoke,
}

impl Display for ActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let action_name = match self {
            ActionType::Payment => "Payment",
            ActionType::TokenIssue => "Token Issue",
            ActionType::TokenMint => "Token Mint",
            ActionType::TokenBurn => "Token Burn",
            ActionType::AuthorityGrant => "Authority Grant",
            ActionType::AuthorityRevoke => "Authority Revoke",
        };
        write!(f, "{}", action_name)
    }
}
