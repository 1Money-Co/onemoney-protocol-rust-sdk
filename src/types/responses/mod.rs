//! API response type definitions.

use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod accounts;
pub mod chains;
pub mod checkpoints;
pub mod states;
pub mod tokens;
pub mod transactions;

// Common response types used across multiple modules

/// Generic transaction response from API operations.
/// All transaction operations return the same format: {"hash": "string"}
/// Used by payment transactions, token operations, etc.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// The transaction hash.
    pub hash: B256,
}

impl Display for TransactionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Transaction: {}", self.hash)
    }
}

// Re-export commonly used response types
pub use accounts::*;
pub use chains::*;
pub use checkpoints::*;
pub use states::*;
pub use tokens::*;
pub use transactions::*;

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::B256;
    use std::str::FromStr;

    #[test]
    fn test_transaction_response_serialization() {
        let transaction_response = TransactionResponse {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&transaction_response).expect("Test data should be valid");
        let deserialized: TransactionResponse =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(transaction_response.hash, deserialized.hash);
    }

    #[test]
    fn test_transaction_response_display() {
        let transaction_response = TransactionResponse {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let display_str = format!("{}", transaction_response);
        assert_eq!(
            display_str,
            "Transaction: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"
        );
    }

    #[test]
    fn test_payment_response_serialization() {
        let payment_response = TransactionResponse {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&payment_response).expect("Test data should be valid");
        let deserialized: TransactionResponse =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(payment_response.hash, deserialized.hash);
    }

    #[test]
    fn test_payment_response_display() {
        let payment_response = TransactionResponse {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let display_str = format!("{}", payment_response);
        assert_eq!(
            display_str,
            "Transaction: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"
        );
    }

    #[test]
    fn test_response_types_compatibility() {
        let hash =
            B256::from_str("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
                .expect("Test data should be valid");

        let transaction_response = TransactionResponse { hash };
        let payment_response = TransactionResponse { hash };

        // Both should serialize to the same JSON format
        let transaction_json =
            serde_json::to_string(&transaction_response).expect("Should serialize");
        let payment_json = serde_json::to_string(&payment_response).expect("Should serialize");

        // JSON should contain the hash field
        assert!(transaction_json.contains("\"hash\""));
        assert!(payment_json.contains("\"hash\""));

        // Both should have the same hash value
        assert_eq!(transaction_response.hash, payment_response.hash);
    }
}
