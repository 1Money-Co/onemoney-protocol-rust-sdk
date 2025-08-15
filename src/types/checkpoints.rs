//! Checkpoint-related type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Checkpoint header representation.
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
            "Checkpoint #{} ({}) at timestamp {}{}",
            self.number,
            self.hash,
            self.timestamp,
            if !self.extra_data.is_empty() {
                format!(" [extra: {}]", self.extra_data)
            } else {
                String::new()
            }
        )?;

        // Add roots info on a new line for better readability
        write!(
            f,
            "\n    Roots: State={}, Tx={}, Receipts={}, Parent={}",
            self.state_root, self.transactions_root, self.receipts_root, self.parent_hash
        )
    }
}

/// Checkpoint transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckpointTransactions {
    /// Full transactions.
    Full(Vec<serde_json::Value>),
    /// Only hashes.
    Hashes(Vec<String>),
}

impl Display for CheckpointTransactions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CheckpointTransactions::Full(txs) => write!(f, "{} full transactions", txs.len()),
            CheckpointTransactions::Hashes(hashes) => {
                write!(f, "{} transaction hashes", hashes.len())
            }
        }
    }
}

/// Checkpoint information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Header of the checkpoint.
    #[serde(flatten)]
    pub header: CheckpointHeader,
    /// Checkpoint transactions.
    pub transactions: CheckpointTransactions,
    /// Size of this checkpoint in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

impl Display for Checkpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let size_info = match self.size {
            Some(size) => format!(" ({} bytes)", size),
            None => String::new(),
        };
        write!(f, "{} with {}{}", self.header, self.transactions, size_info)
    }
}

/// Checkpoint number response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointNumber {
    /// Checkpoint number.
    pub number: u64,
}

impl Display for CheckpointNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Checkpoint #{}", self.number)
    }
}
