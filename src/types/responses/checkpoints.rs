//! Checkpoint-related API response types.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Checkpoint transactions representation.
/// This can be either transaction hashes or full transaction objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckpointTransactions {
    /// Full transaction objects
    Full(Vec<crate::Transaction>),
    /// Only transaction hashes
    Hashes(Vec<String>),
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
    pub hash: String,
    /// Hash of the parent.
    pub parent_hash: String,
    /// State root hash.
    pub state_root: String,
    /// Transactions root hash.
    pub transactions_root: String,
    /// Transactions receipts root hash.
    pub receipts_root: String,
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
        write!(
            f,
            "Checkpoint #{}: {} (parent: {}, timestamp: {})",
            self.number, self.hash, self.parent_hash, self.timestamp
        )?;
        if let Some(size) = self.size {
            write!(f, " size: {} bytes", size)?;
        }
        Ok(())
    }
}

/// Checkpoint header representation.
/// This is kept for backward compatibility but consider using flattened Checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointHeader {
    /// Hash of the checkpoint.
    pub hash: String,
    /// Hash of the parent.
    pub parent_hash: String,
    /// State root hash.
    pub state_root: String,
    /// Transactions root hash.
    pub transactions_root: String,
    /// Transactions receipts root hash.
    pub receipts_root: String,
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
