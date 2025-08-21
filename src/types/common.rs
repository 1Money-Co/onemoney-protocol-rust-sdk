//! Common types used throughout the OneMoney SDK.

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// ECDSA signature components.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signature {
    /// R component of the signature.
    pub r: U256,
    /// S component of the signature.
    pub s: U256,
    /// V component of the signature (recovery ID).
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
