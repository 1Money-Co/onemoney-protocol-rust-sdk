//! Account-related API operations.

use super::client::Client;
use crate::api::client::api_path;
use crate::api::client::endpoints::accounts::{NONCE, TOKEN_ACCOUNT};
use crate::{AccountNonce, OneMoneyAddress, Result, TokenAccount};
use serde::Serialize;

/// Account query parameters.
#[derive(Debug, Clone, Serialize)]
pub struct AccountQuery {
    /// Account address to query.
    pub address: OneMoneyAddress,
}

/// Token account query parameters.
#[derive(Debug, Clone, Serialize)]
pub struct TokenAccountQuery {
    /// Owner wallet address.
    pub owner: OneMoneyAddress,
    /// Token mint address.
    pub mint: OneMoneyAddress,
}

impl Client {
    /// Get the nonce for an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The account address to query
    ///
    /// # Returns
    ///
    /// The account nonce information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::{Client, OneMoneyAddress};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///     let address = OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    ///
    ///     let nonce = client.get_account_nonce(address).await?;
    ///     println!("Account nonce: {}", nonce.nonce);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_nonce(&self, address: OneMoneyAddress) -> Result<AccountNonce> {
        let path = api_path(&format!("{}?address={}", NONCE, address));
        self.get(&path).await
    }

    /// Get token account information for a specific owner and mint.
    ///
    /// # Arguments
    ///
    /// * `owner` - The wallet owner address
    /// * `mint` - The token mint address
    ///
    /// # Returns
    ///
    /// The token account information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::{Client, OneMoneyAddress};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet();
    ///     let owner = OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    ///     let mint = OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    ///
    ///     let account = client.get_token_account(owner, mint).await?;
    ///     println!("Token balance: {}", account.balance);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_token_account(
        &self,
        owner: OneMoneyAddress,
        mint: OneMoneyAddress,
    ) -> Result<TokenAccount> {
        let path = api_path(&format!(
            "{}?address={}&token={}",
            TOKEN_ACCOUNT, owner, mint
        ));
        self.get(&path).await
    }
}
