//! Account-related API operations.

use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::accounts::{NONCE, TOKEN_ACCOUNT};
use crate::{AccountNonce, AssociatedTokenAccount, OneMoneyAddress, Result};
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
        let path = api_path(&format!("{NONCE}?address={address}"));
        self.get(&path).await
    }

    /// Get associated token account information for a specific address and token.
    ///
    /// This method queries the L1 server's `/v1/accounts/token_account` endpoint
    /// to retrieve token account details including balance and nonce.
    ///
    /// # Arguments
    ///
    /// * `address` - The wallet owner address
    /// * `token` - The token mint address
    ///
    /// # Returns
    ///
    /// The associated token account information.
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
    ///     let token = OneMoneyAddress::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    ///
    ///     let account = client.get_associated_token_account(address, token).await?;
    ///     println!("Token balance: {}", account.balance);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_associated_token_account(
        &self,
        address: OneMoneyAddress,
        token: OneMoneyAddress,
    ) -> Result<AssociatedTokenAccount> {
        let path = api_path(&format!("{TOKEN_ACCOUNT}?address={address}&token={token}"));
        self.get(&path).await
    }
}
