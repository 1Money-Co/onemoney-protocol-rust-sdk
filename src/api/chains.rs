//! Chain-related API operations.

use crate::Result;
use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::chains::CHAIN_ID;
use crate::responses::ChainIdResponse;

impl Client {
    /// Get the chain ID for this network.
    ///
    /// This method returns the predefined chain ID for the client's network configuration
    /// without making any network requests. This is fast and always available.
    ///
    /// # Returns
    ///
    /// The chain ID for this network.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onemoney_protocol::Client;
    ///
    /// let client = Client::mainnet().unwrap();
    /// let chain_id = client.get_chain_id();
    /// assert_eq!(chain_id, 21210);
    /// ```
    pub fn get_chain_id(&self) -> u64 {
        self.network.chain_id()
    }

    /// Fetch the current chain ID from the network API.
    ///
    /// This method makes an HTTP request to fetch the chain ID from the network.
    /// Use this to verify that the network is responding correctly and matches
    /// the expected chain ID.
    ///
    /// # Returns
    ///
    /// The chain ID from the API response.
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
    ///     let api_chain_id = client.fetch_chain_id_from_network().await?;
    ///     let expected_chain_id = client.get_chain_id();
    ///
    ///     assert_eq!(api_chain_id, expected_chain_id);
    ///     println!("Network chain ID matches expected: {}", api_chain_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_chain_id_from_network(&self) -> Result<u64> {
        let response: ChainIdResponse = self.get(&api_path(CHAIN_ID)).await?;
        Ok(response.chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[test]
    fn test_chain_id_response_structure() {
        // Test that ChainIdResponse can be serialized/deserialized
        let chain_id_response = ChainIdResponse { chain_id: 1212101 };

        let json = serde_json::to_string(&chain_id_response).expect("Test data should be valid");
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(chain_id_response.chain_id, deserialized.chain_id);
    }

    #[test]
    fn test_get_chain_id() {
        // Test get_chain_id method for different client types
        let mainnet_client = Client::mainnet().expect("Should create mainnet client");
        let testnet_client = Client::testnet().expect("Should create testnet client");
        let local_client = Client::local().expect("Should create local client");

        assert_eq!(mainnet_client.get_chain_id(), 21210);
        assert_eq!(testnet_client.get_chain_id(), 1_212_101);
        assert_eq!(local_client.get_chain_id(), 1_212_101);
    }
}
