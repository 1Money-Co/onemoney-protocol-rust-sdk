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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Address;
    use std::str::FromStr;

    #[test]
    fn test_account_nonce_structure() {
        let nonce = AccountNonce { nonce: 42 };

        // Test serialization
        let json = serde_json::to_string(&nonce).expect("Should serialize");
        assert!(json.contains("42"));

        // Test deserialization
        let deserialized: AccountNonce = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.nonce, 42);

        // Test display
        let display_str = format!("{}", nonce);
        assert_eq!(display_str, "Account Nonce: 42");

        // Test clone and debug
        let cloned = nonce.clone();
        assert_eq!(nonce.nonce, cloned.nonce);

        let debug_str = format!("{:?}", nonce);
        assert!(debug_str.contains("AccountNonce"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_account_nonce_different_values() {
        let test_cases = [0u64, 1, 100, 999999, u64::MAX];

        for nonce_value in test_cases {
            let nonce = AccountNonce { nonce: nonce_value };

            // Test serialization round-trip
            let json = serde_json::to_string(&nonce).expect("Should serialize");
            let deserialized: AccountNonce =
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(nonce.nonce, deserialized.nonce);

            // Test display
            let display_str = format!("{}", nonce);
            assert_eq!(display_str, format!("Account Nonce: {}", nonce_value));
        }
    }

    #[test]
    fn test_associated_token_account_structure() {
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let account = AssociatedTokenAccount {
            token_account_address: address,
            balance: "1000000000000000000".to_string(),
            nonce: 5,
        };

        // Test serialization
        let json = serde_json::to_string(&account).expect("Should serialize");
        assert!(json.contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0")); // lowercase hex
        assert!(json.contains("1000000000000000000"));
        assert!(json.contains("5"));

        // Test deserialization
        let deserialized: AssociatedTokenAccount =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.token_account_address, address);
        assert_eq!(deserialized.balance, "1000000000000000000");
        assert_eq!(deserialized.nonce, 5);

        // Test display
        let display_str = format!("{}", account);
        assert!(display_str.contains("Associated Token Account:"));
        assert!(display_str.contains("Address:"));
        assert!(display_str.contains("Balance:"));
        assert!(display_str.contains("Nonce:"));
        assert!(display_str.contains("0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0")); // Address format with 0x prefix and checksum case
        assert!(display_str.contains("1000000000000000000"));
        assert!(display_str.contains("5"));
    }

    #[test]
    fn test_associated_token_account_default() {
        let default_account = AssociatedTokenAccount::default();

        assert_eq!(default_account.token_account_address, Address::ZERO);
        assert_eq!(default_account.balance, String::default());
        assert_eq!(default_account.nonce, 0);

        // Test that default can be serialized
        let json = serde_json::to_string(&default_account).expect("Should serialize");
        let deserialized: AssociatedTokenAccount =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_account, deserialized);
    }

    #[test]
    fn test_associated_token_account_equality_and_hashing() {
        let address1 =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let address2 =
            Address::from_str("0x1234567890abcdef1234567890abcdef12345678").expect("Valid address");

        let account1 = AssociatedTokenAccount {
            token_account_address: address1,
            balance: "1000".to_string(),
            nonce: 1,
        };

        let account2 = AssociatedTokenAccount {
            token_account_address: address1,
            balance: "1000".to_string(),
            nonce: 1,
        };

        let account3 = AssociatedTokenAccount {
            token_account_address: address2,
            balance: "1000".to_string(),
            nonce: 1,
        };

        // Test equality
        assert_eq!(account1, account2);
        assert_ne!(account1, account3);

        // Test that Hash trait is implemented and works
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(account1.clone());
        set.insert(account2.clone());
        set.insert(account3.clone());

        // Should have 2 unique accounts (account1 == account2)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&account1));
        assert!(set.contains(&account3));
    }

    #[test]
    fn test_associated_token_account_clone() {
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let account = AssociatedTokenAccount {
            token_account_address: address,
            balance: "2000000000000000000".to_string(),
            nonce: 10,
        };

        let cloned = account.clone();

        assert_eq!(account.token_account_address, cloned.token_account_address);
        assert_eq!(account.balance, cloned.balance);
        assert_eq!(account.nonce, cloned.nonce);
        assert_eq!(account, cloned);
    }

    #[test]
    fn test_associated_token_account_debug() {
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let account = AssociatedTokenAccount {
            token_account_address: address,
            balance: "500000".to_string(),
            nonce: 3,
        };

        let debug_str = format!("{:?}", account);
        assert!(debug_str.contains("AssociatedTokenAccount"));
        assert!(debug_str.contains("token_account_address"));
        assert!(debug_str.contains("balance"));
        assert!(debug_str.contains("nonce"));
        assert!(debug_str.contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0")); // lowercase hex in debug
        assert!(debug_str.contains("500000"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_associated_token_account_edge_cases() {
        // Test with zero address
        let zero_account = AssociatedTokenAccount {
            token_account_address: Address::ZERO,
            balance: "0".to_string(),
            nonce: 0,
        };

        let json = serde_json::to_string(&zero_account).expect("Should serialize");
        let deserialized: AssociatedTokenAccount =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(zero_account, deserialized);

        // Test with very large balance
        let large_balance_account = AssociatedTokenAccount {
            token_account_address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Valid address"),
            balance: "340282366920938463463374607431768211455".to_string(), // Max u128
            nonce: u64::MAX,
        };

        let json = serde_json::to_string(&large_balance_account).expect("Should serialize");
        let deserialized: AssociatedTokenAccount =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(large_balance_account, deserialized);
    }

    #[test]
    fn test_nonce_type_alias() {
        // Test that Nonce type alias works correctly
        let nonce: Nonce = 42;
        assert_eq!(nonce, 42u64);

        // Test in context of AssociatedTokenAccount
        let account = AssociatedTokenAccount {
            token_account_address: Address::ZERO,
            balance: "0".to_string(),
            nonce,
        };
        assert_eq!(account.nonce, 42);
    }

    #[test]
    fn test_json_format_compatibility() {
        // Test that our structures match expected JSON format from L1 API

        // Account nonce should serialize as simple object with nonce field
        let nonce = AccountNonce { nonce: 123 };
        let json = serde_json::to_string(&nonce).expect("Should serialize");
        assert_eq!(json, r#"{"nonce":123}"#);

        // Test deserialization from expected L1 format
        let l1_json = r#"{"nonce":456}"#;
        let deserialized: AccountNonce = serde_json::from_str(l1_json).expect("Should deserialize");
        assert_eq!(deserialized.nonce, 456);

        // AssociatedTokenAccount should maintain field names
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let account = AssociatedTokenAccount {
            token_account_address: address,
            balance: "1000".to_string(),
            nonce: 5,
        };

        let json = serde_json::to_string(&account).expect("Should serialize");
        assert!(json.contains("token_account_address"));
        assert!(json.contains("balance"));
        assert!(json.contains("nonce"));
    }
}
