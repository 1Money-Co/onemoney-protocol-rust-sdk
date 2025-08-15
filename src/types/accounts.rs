//! Account-related type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
