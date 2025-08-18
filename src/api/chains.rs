//! Chain-related API operations.

use super::client::Client;
use crate::Result;
use crate::api::client::api_path;
use crate::api::client::endpoints::chains::CHAIN_ID;
use serde::{Deserialize, Serialize};

/// Chain ID response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainIdResponse {
    pub chain_id: u64,
}

impl Client {
    /// Get the current chain ID.
    ///
    /// # Returns
    ///
    /// The chain ID.
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
    ///     let chain_id = client.get_chain_id().await?;
    ///     println!("Current chain ID: {}", chain_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_chain_id(&self) -> Result<u64> {
        let response: ChainIdResponse = self.get(&api_path(CHAIN_ID)).await?;
        Ok(response.chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_response_structure() {
        // Test that ChainIdResponse can be serialized/deserialized
        let chain_id_response = ChainIdResponse { chain_id: 1212101 };

        let json = serde_json::to_string(&chain_id_response).unwrap();
        let deserialized: ChainIdResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(chain_id_response.chain_id, deserialized.chain_id);
    }
}
