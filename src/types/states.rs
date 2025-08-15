//! State-related type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Latest epoch and checkpoint response.
/// Matches the L1 server's EpochCheckpointResponse structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestStateResponse {
    /// Latest epoch number.
    pub epoch: u64,
    /// Latest checkpoint number.
    pub checkpoint: u64,
    /// Checkpoint hash.
    pub checkpoint_hash: String,
    /// Parent checkpoint hash.
    pub checkpoint_parent_hash: String,
}

impl Display for LatestStateResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "State: epoch {}, checkpoint {}, hash {} (parent: {})",
            self.epoch, self.checkpoint, self.checkpoint_hash, self.checkpoint_parent_hash
        )
    }
}
