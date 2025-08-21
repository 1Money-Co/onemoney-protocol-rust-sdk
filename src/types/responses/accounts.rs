//! Account-related API response types.

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Nonce type from L1 primitives
pub type Nonce = u64;

/// Account nonce information from API response.
/// Matches the L1 server's AccountInfo structure: { "nonce": u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountNonce {
    /// Current nonce.
    pub nonce: u64,
}

impl Display for AccountNonce {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Account Nonce: {}", self.nonce)
    }
}

/// Represents the token holdings and associated data for a specific address.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssociatedTokenAccount {
    /// The address that derived from the owner address and token address, we
    /// call it as the associated token account address.
    pub token_account_address: Address,
    /// The balance of the token.
    pub balance: String,
    /// The nonce of the owner account.
    pub nonce: Nonce,
}

impl Display for AssociatedTokenAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Associated Token Account:\n  Address: {}\n  Balance: {}\n  Nonce: {}",
            self.token_account_address, self.balance, self.nonce
        )
    }
}
