//! Governance and epoch-related API operations.
//!
//! This module provides convenient helpers for retrieving epoch information
//! from the OneMoney REST API using the shared [`Client`] implementation.

use crate::client::Client;
use crate::client::config::{
    api_path,
    endpoints::governance::{CURRENT_EPOCH, EPOCH_BY_ID},
};
use crate::{EpochResponse, Result};

impl Client {
    /// Fetch the current epoch information from the network.
    ///
    /// # Returns
    ///
    /// The latest epoch metadata including the governance certificate data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use onemoney_protocol::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::testnet()?;
    /// let epoch = client.get_current_epoch().await?;
    /// println!("Current epoch id: {}", epoch.epoch_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_current_epoch(&self) -> Result<EpochResponse> {
        let path = api_path(CURRENT_EPOCH);
        self.get(&path).await
    }

    /// Fetch epoch information by its identifier.
    ///
    /// # Arguments
    ///
    /// * `epoch_id` - The epoch identifier to query.
    pub async fn get_epoch_by_id(&self, epoch_id: u64) -> Result<EpochResponse> {
        let path = api_path(&format!("{EPOCH_BY_ID}?id={epoch_id}"));
        self.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::config::api_path;

    #[test]
    fn test_current_epoch_endpoint_path() {
        let path = api_path(CURRENT_EPOCH);
        assert_eq!(path, "/v1/governances/epoch");
        assert!(path.contains("/governances/epoch"));
    }

    #[test]
    fn test_epoch_by_id_endpoint_path() {
        let id = 42;
        let path = api_path(&format!("{EPOCH_BY_ID}?id={id}"));
        assert_eq!(path, "/v1/governances/epoch/by_id?id=42");
        assert!(path.contains("id=42"));
    }
}
