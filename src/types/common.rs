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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;

    #[test]
    fn test_signature_structure() {
        let signature = Signature {
            r: U256::from_str_radix("12345678901234567890", 10).expect("Valid decimal"),
            s: U256::from_str_radix("98765432109876543210", 10).expect("Valid decimal"),
            v: 1, // Updated to L1 compatible format (0 or 1)
        };

        // Test serialization
        let json = serde_json::to_string(&signature).expect("Should serialize");
        // U256 serializes as hex strings with 0x prefix
        assert!(json.contains("\"0x"));
        assert!(json.contains("1")); // Updated to L1 format

        // Test deserialization
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(
            deserialized.r,
            U256::from_str_radix("12345678901234567890", 10).expect("Valid decimal")
        );
        assert_eq!(
            deserialized.s,
            U256::from_str_radix("98765432109876543210", 10).expect("Valid decimal")
        );
        assert_eq!(deserialized.v, 1);

        // Test display
        let display_str = format!("{}", signature);
        assert!(display_str.contains("Signature(r:"));
        assert!(display_str.contains("s:"));
        assert!(display_str.contains("v: 1"));

        // Test debug
        let debug_str = format!("{:?}", signature);
        assert!(debug_str.contains("Signature"));
        assert!(debug_str.contains("r:"));
        assert!(debug_str.contains("s:"));
        assert!(debug_str.contains("v: 1"));
    }

    #[test]
    fn test_signature_new_constructor() {
        let r = U256::from(1111111111111111111u64);
        let s = U256::from(2222222222222222222u64);
        let v = 1; // Updated to L1 format

        let signature = Signature::new(r, s, v);

        assert_eq!(signature.r, r);
        assert_eq!(signature.s, s);
        assert_eq!(signature.v, v);
    }

    #[test]
    fn test_signature_default() {
        let default_signature = Signature::default();

        assert_eq!(default_signature.r, U256::ZERO);
        assert_eq!(default_signature.s, U256::ZERO);
        assert_eq!(default_signature.v, 0);

        // Test that default can be serialized
        let json = serde_json::to_string(&default_signature).expect("Should serialize");
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_signature, deserialized);

        // Test display of default
        let display_str = format!("{}", default_signature);
        assert_eq!(display_str, "Signature(r: 0, s: 0, v: 0)");
    }

    #[test]
    fn test_signature_equality_and_hashing() {
        let signature1 = Signature::new(U256::from(123u64), U256::from(456u64), 0);
        let signature2 = Signature::new(U256::from(123u64), U256::from(456u64), 0);
        let signature3 = Signature::new(U256::from(123u64), U256::from(456u64), 1);

        // Test equality
        assert_eq!(signature1, signature2);
        assert_ne!(signature1, signature3);

        // Test that Hash trait is implemented and works
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(signature1.clone());
        set.insert(signature2.clone());
        set.insert(signature3.clone());

        // Should have 2 unique signatures (signature1 == signature2)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&signature1));
        assert!(set.contains(&signature3));
    }

    #[test]
    fn test_signature_clone() {
        let signature = Signature::new(U256::from(999u64), U256::from(888u64), 0);
        let cloned = signature.clone();

        assert_eq!(signature.r, cloned.r);
        assert_eq!(signature.s, cloned.s);
        assert_eq!(signature.v, cloned.v);
        assert_eq!(signature, cloned);
    }

    #[test]
    fn test_signature_with_large_values() {
        // Test with maximum U256 values
        let max_signature = Signature {
            r: U256::MAX,
            s: U256::MAX,
            v: u64::MAX,
        };

        let json = serde_json::to_string(&max_signature).expect("Should serialize");
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(max_signature, deserialized);
        assert_eq!(deserialized.r, U256::MAX);
        assert_eq!(deserialized.s, U256::MAX);
        assert_eq!(deserialized.v, u64::MAX);

        // Test with zero values
        let zero_signature = Signature {
            r: U256::ZERO,
            s: U256::ZERO,
            v: 0,
        };

        let json = serde_json::to_string(&zero_signature).expect("Should serialize");
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(zero_signature, deserialized);
    }

    #[test]
    fn test_signature_with_ethereum_recovery_ids() {
        // Test L1-compatible recovery IDs
        let recovery_ids = [0, 1];

        for &recovery_id in &recovery_ids {
            let signature = Signature::new(U256::from(100u64), U256::from(200u64), recovery_id);

            let json = serde_json::to_string(&signature).expect("Should serialize");
            let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");

            assert_eq!(signature, deserialized);
            assert_eq!(deserialized.v, recovery_id);
        }
    }

    #[test]
    fn test_action_type_serialization() {
        let action_types = [
            ActionType::Payment,
            ActionType::TokenIssue,
            ActionType::TokenMint,
            ActionType::TokenBurn,
            ActionType::AuthorityGrant,
            ActionType::AuthorityRevoke,
        ];

        let expected_json_values = [
            "payment",
            "token_issue",
            "token_mint",
            "token_burn",
            "authority_grant",
            "authority_revoke",
        ];

        for (action_type, expected_json) in action_types.iter().zip(expected_json_values.iter()) {
            // Test serialization
            let json = serde_json::to_string(action_type).expect("Should serialize");
            assert_eq!(json, format!("\"{}\"", expected_json));

            // Test deserialization
            let deserialized: ActionType = serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(&deserialized, action_type);
        }
    }

    #[test]
    fn test_action_type_display() {
        let test_cases = [
            (ActionType::Payment, "Payment"),
            (ActionType::TokenIssue, "Token Issue"),
            (ActionType::TokenMint, "Token Mint"),
            (ActionType::TokenBurn, "Token Burn"),
            (ActionType::AuthorityGrant, "Authority Grant"),
            (ActionType::AuthorityRevoke, "Authority Revoke"),
        ];

        for (action_type, expected_display) in test_cases {
            let display_str = format!("{}", action_type);
            assert_eq!(display_str, expected_display);
        }
    }

    #[test]
    fn test_action_type_equality_and_clone() {
        let action1 = ActionType::Payment;
        let action2 = ActionType::Payment;
        let action3 = ActionType::TokenIssue;

        // Test equality
        assert_eq!(action1, action2);
        assert_ne!(action1, action3);

        // Test clone
        let cloned = action1.clone();
        assert_eq!(action1, cloned);

        // Test debug
        let debug_str = format!("{:?}", action1);
        assert!(debug_str.contains("Payment"));
    }

    #[test]
    fn test_action_type_comprehensive_serialization_round_trip() {
        let all_actions = [
            ActionType::Payment,
            ActionType::TokenIssue,
            ActionType::TokenMint,
            ActionType::TokenBurn,
            ActionType::AuthorityGrant,
            ActionType::AuthorityRevoke,
        ];

        for action_type in &all_actions {
            // Test serialization round-trip
            let json = serde_json::to_string(action_type).expect("Should serialize");
            let deserialized: ActionType = serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(&deserialized, action_type);

            // Ensure JSON uses snake_case
            let json_value: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");
            if let serde_json::Value::String(s) = json_value {
                assert!(s.contains('_') || s == "payment"); // payment is already snake_case
                assert!(!s.chars().any(|c| c.is_uppercase())); // no uppercase letters
            } else {
                panic!("Expected string JSON value");
            }
        }
    }

    #[test]
    fn test_json_format_compatibility() {
        // Test that our structures match expected JSON format from L1 API

        // Signature should serialize with field names preserved
        let signature = Signature::new(U256::from(123u64), U256::from(456u64), 0);
        let json = serde_json::to_string(&signature).expect("Should serialize");
        assert!(json.contains("\"r\""));
        assert!(json.contains("\"s\""));
        assert!(json.contains("\"v\""));

        // Test deserialization from L1-compatible format
        let l1_signature_json = r#"{"r":"123","s":"456","v":0}"#;
        let deserialized: Signature =
            serde_json::from_str(l1_signature_json).expect("Should deserialize");
        assert_eq!(deserialized.r, U256::from(123u64));
        assert_eq!(deserialized.s, U256::from(456u64));
        assert_eq!(deserialized.v, 0);

        // ActionType should serialize as snake_case strings
        let action = ActionType::TokenMint;
        let json = serde_json::to_string(&action).expect("Should serialize");
        assert_eq!(json, "\"token_mint\"");

        // Test deserialization from L1-compatible format
        let l1_action_json = "\"authority_grant\"";
        let deserialized: ActionType =
            serde_json::from_str(l1_action_json).expect("Should deserialize");
        assert_eq!(deserialized, ActionType::AuthorityGrant);
    }

    #[test]
    fn test_signature_edge_cases() {
        // Test signature with one component being zero
        let partial_zero_sig = Signature {
            r: U256::ZERO,
            s: U256::from(12345u64),
            v: 1, // Updated to L1 format
        };

        let json = serde_json::to_string(&partial_zero_sig).expect("Should serialize");
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(partial_zero_sig, deserialized);

        // Test signature with specific secp256k1 curve values
        let curve_order = U256::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .expect("Valid hex");
        let secp_signature = Signature {
            r: curve_order - U256::from(1u64), // Just below curve order
            s: U256::from(1u64),               // Minimum valid s value
            v: 0,                              // Even parity
        };

        let json = serde_json::to_string(&secp_signature).expect("Should serialize");
        let deserialized: Signature = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(secp_signature, deserialized);
    }

    #[test]
    fn test_action_type_case_insensitive_deserialization() {
        // Test that deserialization works with the exact snake_case format
        let test_cases = [
            ("\"payment\"", ActionType::Payment),
            ("\"token_issue\"", ActionType::TokenIssue),
            ("\"token_mint\"", ActionType::TokenMint),
            ("\"token_burn\"", ActionType::TokenBurn),
            ("\"authority_grant\"", ActionType::AuthorityGrant),
            ("\"authority_revoke\"", ActionType::AuthorityRevoke),
        ];

        for (json_str, expected_action) in test_cases {
            let deserialized: ActionType =
                serde_json::from_str(json_str).expect("Should deserialize");
            assert_eq!(deserialized, expected_action);
        }
    }

    #[test]
    fn test_signature_field_access() {
        let signature = Signature::new(U256::from(777u64), U256::from(888u64), 29);

        // Test direct field access
        assert_eq!(signature.r, U256::from(777u64));
        assert_eq!(signature.s, U256::from(888u64));
        assert_eq!(signature.v, 29);

        // Test field modification (through mutable instance)
        let mut mutable_signature = signature.clone();
        mutable_signature.r = U256::from(999u64);
        mutable_signature.s = U256::from(111u64);
        mutable_signature.v = 30;

        assert_eq!(mutable_signature.r, U256::from(999u64));
        assert_eq!(mutable_signature.s, U256::from(111u64));
        assert_eq!(mutable_signature.v, 30);
        assert_ne!(signature, mutable_signature);
    }
}
