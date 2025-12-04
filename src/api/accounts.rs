//! Account-related API operations.

use crate::client::Client;
use crate::client::config::api_path;
use crate::client::config::endpoints::accounts::{BBNONCE, NONCE, TOKEN_ACCOUNT};
use crate::{AccountBBNonce, AccountNonce, AssociatedTokenAccount, Result};
use alloy_primitives::Address;

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
    /// use onemoney_protocol::Client;
    /// use alloy_primitives::Address;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet()?;
    ///     let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    ///
    ///     let nonce = client.get_account_nonce(address).await?;
    ///     println!("Account nonce: {}", nonce.nonce);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_nonce(&self, address: Address) -> Result<AccountNonce> {
        let path = api_path(&format!("{NONCE}?address={address}"));
        self.get(&path).await
    }

    /// Get the BB nonce for an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The account address to query
    ///
    /// # Returns
    ///
    /// The account BB nonce information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use onemoney_protocol::Client;
    /// use alloy_primitives::Address;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet()?;
    ///     let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    ///
    ///     let bbnonce = client.get_account_bbonce(address).await?;
    ///     println!("Account BB nonce: {}", bbnonce.bbnonce);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_bbonce(&self, address: Address) -> Result<AccountBBNonce> {
        let path = api_path(&format!("{BBNONCE}?address={address}"));
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
    /// use onemoney_protocol::Client;
    /// use alloy_primitives::Address;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::mainnet()?;
    ///     let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    ///     let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")?;
    ///
    ///     let account = client.get_associated_token_account(address, token).await?;
    ///     println!("Token balance: {}", account.balance);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_associated_token_account(
        &self,
        address: Address,
        token: Address,
    ) -> Result<AssociatedTokenAccount> {
        let path = api_path(&format!("{TOKEN_ACCOUNT}?address={address}&token={token}"));
        self.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::config::api_path;
    use alloy_primitives::Address;
    use std::str::FromStr;

    #[test]
    fn test_nonce_api_path_construction() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let expected_path = api_path(&format!("{NONCE}?address={address}"));

        assert!(expected_path.contains("/accounts/nonce"));
        assert!(expected_path.contains("address=0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0"));
    }

    #[test]
    fn test_bbnonce_api_path_construction() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let expected_path = api_path(&format!("{BBNONCE}?address={address}"));

        assert!(expected_path.contains("/accounts/bbnonce"));
        assert!(expected_path.contains("address=0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0"));
    }

    #[test]
    fn test_token_account_api_path_construction() {
        let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid");
        let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid");
        let expected_path = api_path(&format!("{TOKEN_ACCOUNT}?address={address}&token={token}"));

        assert!(expected_path.contains("/accounts/token_account"));
        assert!(expected_path.contains("address=0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0"));
        assert!(expected_path.contains("token=0x1234567890AbcdEF1234567890aBcdef12345678"));
    }

    #[test]
    fn test_account_nonce_display() {
        let nonce = AccountNonce { nonce: 42 };
        let display_str = format!("{}", nonce);
        assert_eq!(display_str, "Account Nonce: 42");
    }

    #[test]
    fn test_account_bbnonce_display() {
        let bbnonce = AccountBBNonce { bbnonce: 42 };
        let display_str = format!("{}", bbnonce);
        assert_eq!(display_str, "Account BB Nonce: 42");
    }

    #[test]
    fn test_associated_token_account_display() {
        let account = AssociatedTokenAccount {
            balance: "1000000000000000000".to_string(),
            nonce: 5,
        };

        let display_str = format!("{}", account);
        assert!(display_str.contains("Associated Token Account"));
        assert!(display_str.contains("Balance: 1000000000000000000"));
        assert!(display_str.contains("Nonce: 5"));
    }

    #[test]
    fn test_associated_token_account_equality() {
        let account1 = AssociatedTokenAccount {
            balance: "1000000000000000000".to_string(),
            nonce: 5,
        };

        let account2 = AssociatedTokenAccount {
            balance: "1000000000000000000".to_string(),
            nonce: 5,
        };

        assert_eq!(account1, account2);
    }

    #[test]
    fn test_associated_token_account_clone() {
        let account = AssociatedTokenAccount {
            balance: "1000000000000000000".to_string(),
            nonce: 5,
        };

        let cloned = account.clone();
        assert_eq!(account, cloned);
    }

    #[test]
    fn test_associated_token_account_default() {
        let account = AssociatedTokenAccount::default();
        assert_eq!(account.balance, String::default());
        assert_eq!(account.nonce, 0);
    }
}
