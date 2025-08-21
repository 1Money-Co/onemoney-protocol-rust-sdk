//! Checkpoint-related API operations.

use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::checkpoints::{BY_HASH, BY_NUMBER, NUMBER};
use crate::{Checkpoint, CheckpointNumber, Result};

impl Client {
    /// Get a specific checkpoint by number.
    ///
    /// # Arguments
    ///
    /// * `number` - The checkpoint number
    /// * `full` - Whether to include full transaction details
    ///
    /// # Returns
    ///
    /// The checkpoint information.
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
    ///     let checkpoint = client.get_checkpoint_by_number(456, false).await?;
    ///     println!("Checkpoint number: {}", checkpoint.number);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_checkpoint_by_number(&self, number: u64, full: bool) -> Result<Checkpoint> {
        let path = api_path(&format!("{}?number={}&full={}", BY_NUMBER, number, full));
        self.get(&path).await
    }

    /// Get a checkpoint by hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The checkpoint hash
    /// * `full` - Whether to include full transaction details
    ///
    /// # Returns
    ///
    /// The checkpoint information.
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
    ///     let hash = "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777";
    ///     let checkpoint = client.get_checkpoint_by_hash(hash, false).await?;
    ///     println!("Checkpoint number: {}", checkpoint.number);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_checkpoint_by_hash(&self, hash: &str, full: bool) -> Result<Checkpoint> {
        let path = api_path(&format!("{}?hash={}&full={}", BY_HASH, hash, full));
        self.get(&path).await
    }

    /// Get the latest checkpoint number.
    ///
    /// # Returns
    ///
    /// The latest checkpoint number.
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
    ///     let checkpoint_number = client.get_checkpoint_number().await?;
    ///     println!("Latest checkpoint number: {}", checkpoint_number.number);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_checkpoint_number(&self) -> Result<CheckpointNumber> {
        self.get(&api_path(NUMBER)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CheckpointTransactions;

    #[test]
    fn test_checkpoint_structure() {
        // Test that Checkpoint can be serialized/deserialized
        let checkpoint = Checkpoint {
            hash: "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777".to_string(),
            parent_hash: "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"
                .to_string(),
            state_root: "0x18b2b9746b15451d1f9bc414f1c12bda8249c63d4a46926e661ae74c69defd9a"
                .to_string(),
            transactions_root: "0xa1e7ed47e548fa45c30232a7e7dfaad6495cff595a0ee1458aa470e574f3f6e4"
                .to_string(),
            receipts_root: "0x59ff04f73d9f934800687c60fb80e2de6e8233817b46d144aec724b569d80c3b"
                .to_string(),
            number: 1500,
            timestamp: 1739760890,
            extra_data: String::new(),
            transactions: CheckpointTransactions::Hashes(vec![]),
            size: Some(1024),
        };

        let json = serde_json::to_string(&checkpoint).unwrap();
        let deserialized: Checkpoint = serde_json::from_str(&json).unwrap();

        assert_eq!(checkpoint.number, deserialized.number);
        assert_eq!(checkpoint.hash, deserialized.hash);
        assert_eq!(checkpoint.timestamp, deserialized.timestamp);
        assert_eq!(checkpoint.size, deserialized.size);
    }

    #[test]
    fn test_checkpoint_number() {
        let checkpoint_number = CheckpointNumber { number: 50 };

        let json = serde_json::to_string(&checkpoint_number).unwrap();
        let deserialized: CheckpointNumber = serde_json::from_str(&json).unwrap();

        assert_eq!(checkpoint_number.number, deserialized.number);
    }
}
