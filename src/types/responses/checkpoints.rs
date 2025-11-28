//! Checkpoint-related API response types.

use crate::Transaction;
use crate::types::responses::transactions::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Checkpoint transactions representation.
/// This can be either transaction hashes or full transaction objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckpointTransactions {
    /// Full transaction objects
    Full(Vec<Transaction>),
    /// Only transaction hashes
    Hashes(Vec<Hash>),
}

impl Display for CheckpointTransactions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CheckpointTransactions::Full(transactions) => {
                write!(
                    f,
                    "Checkpoint with {} full transactions",
                    transactions.len()
                )
            }
            CheckpointTransactions::Hashes(hashes) => {
                write!(f, "Checkpoint with {} transaction hashes", hashes.len())
            }
        }
    }
}

/// A checkpoint includes header data and transactions.
/// Header fields are flattened at the top level to match L1 server format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Hash of the checkpoint.
    pub hash: Hash,
    /// Hash of the parent.
    pub parent_hash: Hash,
    /// State root hash.
    pub state_root: Hash,
    /// Transactions root hash.
    pub transactions_root: Hash,
    /// Transactions receipts root hash.
    pub receipts_root: Hash,
    /// Checkpoint number.
    pub number: u64,
    /// Timestamp.
    pub timestamp: u64,
    /// Extra data.
    pub extra_data: String,
    /// Checkpoint transactions (either hashes or full objects).
    pub transactions: CheckpointTransactions,
    /// Integer the size of this checkpoint in bytes.
    pub size: Option<u64>,
}

impl Display for Checkpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Checkpoint #{}:", self.number)?;
        writeln!(f, "  Hash: {}", self.hash.hash)?;
        writeln!(f, "  Parent Hash: {}", self.parent_hash.hash)?;
        writeln!(f, "  State Root: {}", self.state_root.hash)?;
        writeln!(f, "  Transactions Root: {}", self.transactions_root.hash)?;
        writeln!(f, "  Receipts Root: {}", self.receipts_root.hash)?;
        writeln!(f, "  Timestamp: {}", self.timestamp)?;
        writeln!(f, "  Extra Data: {}", self.extra_data)?;

        if let Some(size) = self.size {
            writeln!(f, "  Size: {} bytes", size)?;
        }

        writeln!(f, "  Transactions:")?;
        match &self.transactions {
            CheckpointTransactions::Full(transactions) => {
                writeln!(
                    f,
                    "    Count: {} (full transaction details)",
                    transactions.len()
                )?;
                for (i, tx) in transactions.iter().enumerate() {
                    writeln!(f, "    Transaction {}:", i + 1)?;
                    writeln!(f, "      Hash: {}", tx.hash)?;
                    writeln!(f, "      From: {}", tx.from)?;
                    writeln!(f, "      Nonce: {}", tx.nonce)?;
                    writeln!(f, "      Chain ID: {}", tx.chain_id)?;

                    if let Some(checkpoint_hash) = &tx.checkpoint_hash {
                        writeln!(f, "      Checkpoint Hash: {}", checkpoint_hash)?;
                    }
                    if let Some(checkpoint_number) = tx.checkpoint_number {
                        writeln!(f, "      Checkpoint Number: {}", checkpoint_number)?;
                    }
                    if let Some(transaction_index) = tx.transaction_index {
                        writeln!(f, "      Transaction Index: {}", transaction_index)?;
                    }

                    writeln!(f, "      Signature:")?;
                    writeln!(f, "        R: {}", tx.signature.r)?;
                    writeln!(f, "        S: {}", tx.signature.s)?;
                    writeln!(f, "        V: {}", tx.signature.v)?;

                    writeln!(f, "      Payload: {:?}", tx.data)?;

                    if i < transactions.len() - 1 {
                        writeln!(f)?;
                    }
                }
            }
            CheckpointTransactions::Hashes(hashes) => {
                writeln!(f, "    Count: {} (hashes only)", hashes.len())?;
                for (i, hash) in hashes.iter().enumerate() {
                    writeln!(f, "    {}: {}", i + 1, hash.hash)?;
                }
            }
        }

        Ok(())
    }
}

/// Checkpoint header representation.
/// This is kept for backward compatibility but consider using flattened Checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointHeader {
    /// Hash of the checkpoint.
    pub hash: Hash,
    /// Hash of the parent.
    pub parent_hash: Hash,
    /// State root hash.
    pub state_root: Hash,
    /// Transactions root hash.
    pub transactions_root: Hash,
    /// Transactions receipts root hash.
    pub receipts_root: Hash,
    /// Checkpoint number.
    pub number: u64,
    /// Timestamp.
    pub timestamp: u64,
    /// Extra data.
    pub extra_data: String,
}

impl Display for CheckpointHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Checkpoint #{}: {} (parent: {}, timestamp: {})",
            self.number, self.hash, self.parent_hash, self.timestamp
        )
    }
}

/// Checkpoint number response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointNumber {
    /// Current checkpoint number.
    pub number: u64,
}

impl Display for CheckpointNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Checkpoint Number: {}", self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::responses::transactions::*;
    use alloy_primitives::B256;
    use std::str::FromStr;

    /// Helper function to create Hash from hex string
    fn create_hash(hex_str: &str) -> Hash {
        // Pad short hex strings to 32 bytes (64 hex chars + 0x prefix)
        let padded_hex = if hex_str.len() < 66 {
            let without_prefix = hex_str.strip_prefix("0x").unwrap_or(hex_str);
            format!("0x{:0<64}", without_prefix)
        } else {
            hex_str.to_string()
        };

        Hash {
            hash: B256::from_str(&padded_hex).expect("Test data should be valid"),
        }
    }

    #[test]
    fn test_checkpoint_number_structure() {
        let checkpoint_num = CheckpointNumber { number: 12345 };

        // Test serialization
        let json = serde_json::to_string(&checkpoint_num).expect("Test data should be valid");
        let deserialized: CheckpointNumber =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(checkpoint_num.number, deserialized.number);

        // Test display
        let display_str = format!("{}", checkpoint_num);
        assert_eq!(display_str, "Checkpoint Number: 12345");

        // Test clone
        let cloned_num = checkpoint_num.clone();
        assert_eq!(checkpoint_num.number, cloned_num.number);

        // Test debug output
        let debug_num_str = format!("{:?}", checkpoint_num);
        assert!(debug_num_str.contains("CheckpointNumber"));
        assert!(debug_num_str.contains("12345"));
    }

    #[test]
    fn test_checkpoint_transactions_hashes() {
        let hashes = vec![
            create_hash("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"),
            create_hash("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"),
            create_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"),
        ];

        // Test individual Hash serialization is transparent (not an object)
        let single_hash_json =
            serde_json::to_string(&hashes[0]).expect("Hash serialization should work");
        assert!(single_hash_json.starts_with("\"0x"));
        assert!(single_hash_json.ends_with("\""));
        assert!(!single_hash_json.contains("{"));
        assert!(!single_hash_json.contains("hash"));

        let checkpoint_transactions = CheckpointTransactions::Hashes(hashes.clone());

        // Test serialization
        let json =
            serde_json::to_string(&checkpoint_transactions).expect("Test data should be valid");
        let deserialized: CheckpointTransactions =
            serde_json::from_str(&json).expect("Test data should be valid");

        match deserialized {
            CheckpointTransactions::Hashes(deserialized_hashes) => {
                assert_eq!(deserialized_hashes.len(), 3);
                assert_eq!(deserialized_hashes, hashes);
            }
            _ => panic!("Should be Hashes variant"),
        }

        // Test display
        let display_str = format!("{}", checkpoint_transactions);
        assert!(display_str.contains("Checkpoint with 3 transaction hashes"));
    }

    #[test]
    fn test_checkpoint_transactions_empty() {
        // Test empty transactions
        let empty_full = CheckpointTransactions::Full(vec![]);
        let display_str = format!("{}", empty_full);
        assert!(display_str.contains("Checkpoint with 0 full transactions"));

        let empty_hashes = CheckpointTransactions::Hashes(vec![]);
        let display_str = format!("{}", empty_hashes);
        assert!(display_str.contains("Checkpoint with 0 transaction hashes"));
    }

    #[test]
    fn test_checkpoint_header_structure() {
        let header = CheckpointHeader {
            hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
            parent_hash: create_hash(
                "0x000abc456def789012345678901234567890123456789012345678901234567e",
            ),
            state_root: create_hash(
                "0xabc123456def789012345678901234567890123456789012345678901234567e",
            ),
            transactions_root: create_hash(
                "0xdef456789012345678901234567890123456789012345678901234567890abc1",
            ),
            receipts_root: create_hash(
                "0x789012345678901234567890123456789012345678901234567890abc123def4",
            ),
            number: 999,
            timestamp: 1703097600,
            extra_data: "0x1234".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&header).expect("Test data should be valid");
        let deserialized: CheckpointHeader =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(header.hash, deserialized.hash);
        assert_eq!(header.parent_hash, deserialized.parent_hash);
        assert_eq!(header.state_root, deserialized.state_root);
        assert_eq!(header.transactions_root, deserialized.transactions_root);
        assert_eq!(header.receipts_root, deserialized.receipts_root);
        assert_eq!(header.number, deserialized.number);
        assert_eq!(header.timestamp, deserialized.timestamp);
        assert_eq!(header.extra_data, deserialized.extra_data);

        // Test display
        let display_str = format!("{}", header);
        assert!(display_str.contains("Checkpoint #999:"));
        assert!(display_str.contains(&header.hash.hash.to_string()));
        assert!(display_str.contains(&header.parent_hash.hash.to_string()));
        assert!(display_str.contains("1703097600"));
    }

    #[test]
    fn test_checkpoint_with_various_sizes() {
        let checkpoint_with_size = Checkpoint {
            hash: create_hash("0x123abc"),
            parent_hash: create_hash("0x000abc"),
            state_root: create_hash("0xabc123"),
            transactions_root: create_hash("0xdef456"),
            receipts_root: create_hash("0x789012"),
            number: 100,
            timestamp: 1703097600,
            extra_data: "0x".to_string(),
            transactions: CheckpointTransactions::Hashes(vec![]),
            size: Some(2048),
        };

        let checkpoint_without_size = Checkpoint {
            hash: create_hash("0x123abc"),
            parent_hash: create_hash("0x000abc"),
            state_root: create_hash("0xabc123"),
            transactions_root: create_hash("0xdef456"),
            receipts_root: create_hash("0x789012"),
            number: 100,
            timestamp: 1703097600,
            extra_data: "0x".to_string(),
            transactions: CheckpointTransactions::Hashes(vec![]),
            size: None,
        };

        // Test serialization with size
        let json_with_size =
            serde_json::to_string(&checkpoint_with_size).expect("Test data should be valid");
        let deserialized_with_size: Checkpoint =
            serde_json::from_str(&json_with_size).expect("Test data should be valid");
        assert_eq!(checkpoint_with_size.size, deserialized_with_size.size);

        // Test serialization without size
        let json_without_size =
            serde_json::to_string(&checkpoint_without_size).expect("Test data should be valid");
        let deserialized_without_size: Checkpoint =
            serde_json::from_str(&json_without_size).expect("Test data should be valid");
        assert_eq!(checkpoint_without_size.size, deserialized_without_size.size);
        assert!(deserialized_without_size.size.is_none());
    }

    #[test]
    fn test_checkpoint_with_hashes_only() {
        let checkpoint = Checkpoint {
            hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
            parent_hash: create_hash(
                "0x000abc456def789012345678901234567890123456789012345678901234567e",
            ),
            state_root: create_hash(
                "0xabc123456def789012345678901234567890123456789012345678901234567e",
            ),
            transactions_root: create_hash(
                "0xdef456789012345678901234567890123456789012345678901234567890abc1",
            ),
            receipts_root: create_hash(
                "0x789012345678901234567890123456789012345678901234567890abc123def4",
            ),
            number: 1500,
            timestamp: 1703097600,
            extra_data: "checkpoint_data".to_string(),
            transactions: CheckpointTransactions::Hashes(vec![
                create_hash("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"),
                create_hash("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"),
            ]),
            size: None, // Test without size
        };

        // Test display with hashes
        let display_str = format!("{}", checkpoint);
        assert!(display_str.contains("Checkpoint #1500:"));
        assert!(display_str.contains("checkpoint_data"));
        assert!(!display_str.contains("Size:")); // Should not contain size when None
        assert!(display_str.contains("hashes only"));
        assert!(
            display_str
                .contains("1: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
        );
        assert!(
            display_str
                .contains("2: 0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f")
        );
    }

    #[test]
    fn test_serde_untagged_enum() {
        // Test that the untagged enum properly deserializes different JSON structures

        // JSON with array of hashes (should deserialize as Hashes)
        let hashes_json = r#"["0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777", "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"]"#;
        let parsed: CheckpointTransactions =
            serde_json::from_str(hashes_json).expect("Test data should be valid");

        match parsed {
            CheckpointTransactions::Hashes(hashes) => {
                assert_eq!(hashes.len(), 2);
            }
            _ => panic!("Should parse as Hashes variant"),
        }

        // Note: Testing full transaction parsing would require a complete transaction JSON structure
        // with all required fields. For now, we've tested the Hashes variant which works correctly.
        // The untagged enum will automatically choose the correct variant based on the JSON structure.
    }
}
