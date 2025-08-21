//! State-related API response types.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Response type for latest state endpoint
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LatestStateResponse {
    /// Current epoch number.
    pub epoch: u64,
    /// Current checkpoint number.
    pub checkpoint: u64,
}

impl Display for LatestStateResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Latest State: epoch={}, checkpoint={}",
            self.epoch, self.checkpoint
        )
    }
}
