//! State-related API operations.

use super::client::Client;
use crate::api::client::api_path;
use crate::api::client::endpoints::states::LATEST_EPOCH_CHECKPOINT;
use crate::{LatestStateResponse, Result};

impl Client {
    /// Get the latest epoch and checkpoint information.
    ///
    /// This is commonly used when creating transactions to get the required
    /// recent_epoch and recent_checkpoint values.
    ///
    /// # Returns
    ///
    /// The latest state information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///
    ///     let state = client.get_latest_epoch_checkpoint().await?;
    ///     println!("Latest state: epoch {} checkpoint {}", state.epoch, state.checkpoint);
    ///
    ///     // Use in transaction payload
    ///     let recent_epoch = state.epoch;
    ///     let recent_checkpoint = state.checkpoint;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_latest_epoch_checkpoint(&self) -> Result<LatestStateResponse> {
        self.get(&api_path(LATEST_EPOCH_CHECKPOINT)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latest_state_response_structure() {
        let state = LatestStateResponse {
            epoch: 123,
            checkpoint: 456,
            checkpoint_hash: "0xabcdef1234567890".to_string(),
            checkpoint_parent_hash: "0x1234567890abcdef".to_string(),
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: LatestStateResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(state.epoch, deserialized.epoch);
        assert_eq!(state.checkpoint, deserialized.checkpoint);
        assert_eq!(state.checkpoint_hash, deserialized.checkpoint_hash);
        assert_eq!(
            state.checkpoint_parent_hash,
            deserialized.checkpoint_parent_hash
        );
    }
}
