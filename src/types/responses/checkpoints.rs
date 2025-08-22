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
                    writeln!(f, "      Epoch: {}", tx.epoch)?;
                    writeln!(f, "      Checkpoint: {}", tx.checkpoint)?;
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
