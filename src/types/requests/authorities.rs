//! Token-related type definitions.

use alloy_rlp::{BufMut, Encodable};
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

impl Display for AuthorityAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
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

impl Encodable for AuthorityAction {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_str().encode(out);
    }
}

impl Encodable for Authority {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_str().encode(out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_rlp::Encodable;

    #[test]
    fn test_authority_action_display() {
        assert_eq!(AuthorityAction::Grant.to_string(), "Grant");
        assert_eq!(AuthorityAction::Revoke.to_string(), "Revoke");
    }

    #[test]
    fn test_authority_display() {
        assert_eq!(Authority::MasterMintBurn.to_string(), "MasterMintBurn");
        assert_eq!(Authority::MintBurnTokens.to_string(), "MintBurnTokens");
        assert_eq!(Authority::Pause.to_string(), "Pause");
        assert_eq!(Authority::ManageList.to_string(), "ManageList");
        assert_eq!(Authority::UpdateMetadata.to_string(), "UpdateMetadata");
    }

    #[test]
    fn test_authority_action_as_str() {
        assert_eq!(AuthorityAction::Grant.as_str(), "Grant");
        assert_eq!(AuthorityAction::Revoke.as_str(), "Revoke");
    }

    #[test]
    fn test_authority_as_str() {
        assert_eq!(Authority::MasterMintBurn.as_str(), "MasterMintBurn");
        assert_eq!(Authority::MintBurnTokens.as_str(), "MintBurnTokens");
        assert_eq!(Authority::Pause.as_str(), "Pause");
        assert_eq!(Authority::ManageList.as_str(), "ManageList");
        assert_eq!(Authority::UpdateMetadata.as_str(), "UpdateMetadata");
    }

    #[test]
    fn test_authority_action_alloy_rlp_encoding() {
        // Test Grant action encoding
        let grant_action = AuthorityAction::Grant;
        let mut encoded = Vec::new();
        grant_action.encode(&mut encoded);
        assert!(
            !encoded.is_empty(),
            "Grant action should encode to non-empty bytes"
        );

        // Test Revoke action encoding
        let revoke_action = AuthorityAction::Revoke;
        let mut encoded = Vec::new();
        revoke_action.encode(&mut encoded);
        assert!(
            !encoded.is_empty(),
            "Revoke action should encode to non-empty bytes"
        );

        // Test that different actions produce different encodings
        let mut grant_encoded = Vec::new();
        let mut revoke_encoded = Vec::new();
        AuthorityAction::Grant.encode(&mut grant_encoded);
        AuthorityAction::Revoke.encode(&mut revoke_encoded);
        assert_ne!(
            grant_encoded, revoke_encoded,
            "Different actions should have different encodings"
        );
    }

    #[test]
    fn test_authority_alloy_rlp_encoding() {
        let authorities = [
            Authority::MasterMintBurn,
            Authority::MintBurnTokens,
            Authority::Pause,
            Authority::ManageList,
            Authority::UpdateMetadata,
        ];

        let mut encodings = Vec::new();

        // Test that all authorities can be encoded
        for authority in &authorities {
            let mut encoded = Vec::new();
            authority.encode(&mut encoded);
            assert!(
                !encoded.is_empty(),
                "Authority {:?} should encode to non-empty bytes",
                authority
            );
            encodings.push(encoded);
        }

        // Test that all authorities have unique encodings
        for (i, encoding1) in encodings.iter().enumerate() {
            for (j, encoding2) in encodings.iter().enumerate() {
                if i != j {
                    assert_ne!(
                        encoding1, encoding2,
                        "Authorities {:?} and {:?} should have different encodings",
                        authorities[i], authorities[j]
                    );
                }
            }
        }
    }

    #[test]
    fn test_authority_action_encoding_deterministic() {
        // Test that encoding is deterministic (same input produces same output)
        let action = AuthorityAction::Grant;

        let mut encoded1 = Vec::new();
        let mut encoded2 = Vec::new();

        action.encode(&mut encoded1);
        action.encode(&mut encoded2);

        assert_eq!(encoded1, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_authority_encoding_deterministic() {
        // Test that encoding is deterministic (same input produces same output)
        let authority = Authority::MasterMintBurn;

        let mut encoded1 = Vec::new();
        let mut encoded2 = Vec::new();

        authority.encode(&mut encoded1);
        authority.encode(&mut encoded2);

        assert_eq!(encoded1, encoded2, "Encoding should be deterministic");
    }

    #[test]
    fn test_authority_action_serialization_compatibility() {
        // Test JSON serialization/deserialization
        let actions = [AuthorityAction::Grant, AuthorityAction::Revoke];

        for action in &actions {
            let json = serde_json::to_string(action).expect("Should serialize to JSON");
            let deserialized: AuthorityAction =
                serde_json::from_str(&json).expect("Should deserialize from JSON");
            assert_eq!(
                *action, deserialized,
                "Serialization round-trip should preserve value"
            );
        }
    }

    #[test]
    fn test_authority_serialization_compatibility() {
        // Test JSON serialization/deserialization
        let authorities = [
            Authority::MasterMintBurn,
            Authority::MintBurnTokens,
            Authority::Pause,
            Authority::ManageList,
            Authority::UpdateMetadata,
        ];

        for authority in &authorities {
            let json = serde_json::to_string(authority).expect("Should serialize to JSON");
            let deserialized: Authority =
                serde_json::from_str(&json).expect("Should deserialize from JSON");
            assert_eq!(
                *authority, deserialized,
                "Serialization round-trip should preserve value"
            );
        }
    }
}
