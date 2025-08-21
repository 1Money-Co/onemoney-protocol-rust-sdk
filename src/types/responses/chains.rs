//! Chain-related API response types.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Response type for chain ID endpoint
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainIdResponse {
    pub chain_id: u64,
}

impl Display for ChainIdResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Chain ID: {}", self.chain_id)
    }
}
