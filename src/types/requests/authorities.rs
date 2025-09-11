//! Token-related type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Authority action type for granting or revoking permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorityAction {
    /// Grant authority to a user.
    Grant,
    /// Revoke authority from a user.
    Revoke,
}

/// Authority levels that can be granted or revoked for a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Authority {
    /// Can issue tokens and assign all authorities except MasterMintBurn.
    MasterMintBurn,
    /// Can mint/burn tokens.
    MintBurnTokens,
    /// Can pause/unpause the token (blocks transactions).
    Pause,
    /// Can manage the blacklist/whitelist.
    ManageList,
    /// Can update token metadata.
    UpdateMetadata,
}

impl Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Authority {
    /// Returns a stable string representation for RLP encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            Authority::MasterMintBurn => "MasterMintBurn",
            Authority::MintBurnTokens => "MintBurnTokens",
            Authority::Pause => "Pause",
            Authority::ManageList => "ManageList",
            Authority::UpdateMetadata => "UpdateMetadata",
        }
    }
}

impl AuthorityAction {
    /// Returns a stable string representation for RLP encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthorityAction::Grant => "Grant",
            AuthorityAction::Revoke => "Revoke",
        }
    }
}
