//! Chain-related API operations.

use crate::Result;
use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::chains::CHAIN_ID;
use crate::responses::ChainIdResponse;

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
    ///     let client = Client::mainnet()?;
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

        let json = serde_json::to_string(&chain_id_response).expect("Test data should be valid");
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(chain_id_response.chain_id, deserialized.chain_id);
    }
}
