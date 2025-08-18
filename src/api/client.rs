//! HTTP client implementation for the OneMoney SDK.

use crate::{Error, Result, error::ErrorResponse};
use reqwest::{Client as HttpClient, header};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::time::Duration;
use url::Url;

/// Default mainnet API URL.
pub const MAINNET_URL: &str = "https://api.mainnet.1money.network";

/// Default testnet API URL.
pub const TESTNET_URL: &str = "https://api.testnet.1money.network";

/// Default local API URL.
pub const LOCAL_URL: &str = "http://127.0.0.1:18555";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// API version prefix.
pub const API_VERSION: &str = "/v1";

/// Build an API path with version prefix.
pub(crate) fn api_path(path: &str) -> String {
    format!("{}{}", API_VERSION, path)
}

/// API endpoint paths.
pub(crate) mod endpoints {
    /// Account-related endpoints.
    pub mod accounts {
        pub const NONCE: &str = "/accounts/nonce";
        pub const TOKEN_ACCOUNT: &str = "/accounts/token_account";
    }

    /// Chain-related endpoints.
    pub mod chains {
        pub const CHAIN_ID: &str = "/chains/chain_id";
    }

    /// Checkpoint-related endpoints.
    pub mod checkpoints {
        pub const NUMBER: &str = "/checkpoints/number";
        pub const BY_NUMBER: &str = "/checkpoints/by_number";
        pub const BY_HASH: &str = "/checkpoints/by_hash";
    }

    /// Transaction-related endpoints.
    pub mod transactions {
        pub const PAYMENT: &str = "/transactions/payment";
        pub const BY_HASH: &str = "/transactions/by_hash";
        pub const RECEIPT_BY_HASH: &str = "/transactions/receipt/by_hash";
        pub const ESTIMATE_FEE: &str = "/transactions/estimate_fee";
    }

    /// Token-related endpoints.
    pub mod tokens {
        pub const MINT: &str = "/tokens/mint";
        pub const BURN: &str = "/tokens/burn";
        pub const GRANT_AUTHORITY: &str = "/tokens/grant_authority";
        pub const REVOKE_AUTHORITY: &str = "/tokens/revoke_authority";
        pub const UPDATE_METADATA: &str = "/tokens/update_metadata";
        pub const MANAGE_BLACKLIST: &str = "/tokens/manage_blacklist";
        pub const MANAGE_WHITELIST: &str = "/tokens/manage_whitelist";
        pub const PAUSE: &str = "/tokens/pause";
        pub const TOKEN_METADATA: &str = "/tokens/token_metadata";
    }

    /// State-related endpoints.
    pub mod states {
        pub const LATEST_EPOCH_CHECKPOINT: &str = "/states/latest_epoch_checkpoint";
    }

    // /// Governance-related endpoints - TODO: implement when needed
    // pub mod governances {
    //     pub const CERTIFICATE: &str = "/governances/certificate";
    //     pub const EPOCH: &str = "/governances/epoch";
    //     pub const EPOCH_BY_ID: &str = "/governances/epoch/by_id";
    // }

    // /// Utility endpoints - TODO: implement when needed
    // pub mod utils {
    //     pub const HEALTH: &str = "/api/health";
    //     pub const METRICS: &str = "/metrics";
    // }
}

/// Network environment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Network {
    /// Mainnet environment.
    #[default]
    Mainnet,
    /// Testnet environment.
    Testnet,
    /// Local development environment.
    Local,
}

impl Network {
    /// Get the base URL for this network.
    pub fn url(&self) -> &'static str {
        match self {
            Network::Mainnet => MAINNET_URL,
            Network::Testnet => TESTNET_URL,
            Network::Local => LOCAL_URL,
        }
    }

    /// Check if this is a production network.
    pub fn is_production(&self) -> bool {
        matches!(self, Network::Mainnet)
    }

    /// Check if this is a test network.
    pub fn is_test(&self) -> bool {
        matches!(self, Network::Testnet | Network::Local)
    }
}

/// Hook trait for request/response middleware.
pub trait Hook: Send + Sync {
    /// Called before sending a request.
    fn before_request(&self, method: &str, url: &str, body: Option<&str>);

    /// Called after receiving a response.
    fn after_response(&self, method: &str, url: &str, status: u16, body: Option<&str>);
}

/// Logger trait for pluggable logging.
pub trait Logger: Send + Sync {
    /// Log a message.
    fn log(&self, level: LogLevel, message: &str);
}

/// Log levels.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// OneMoney API client.
pub struct Client {
    pub(crate) base_url: Url,
    http_client: HttpClient,
    hooks: Vec<Box<dyn Hook>>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("hooks_count", &self.hooks.len())
            .finish()
    }
}

impl Client {
    /// Create a new client for mainnet.
    ///
    /// # Panics
    ///
    /// This method should never panic as it uses default, valid configuration.
    pub fn mainnet() -> Self {
        ClientBuilder::new()
            .network(Network::Mainnet)
            .build()
            .expect("Failed to create mainnet client with default configuration")
    }

    /// Create a new client for testnet.
    ///
    /// # Panics
    ///
    /// This method should never panic as it uses default, valid configuration.
    pub fn testnet() -> Self {
        ClientBuilder::new()
            .network(Network::Testnet)
            .build()
            .expect("Failed to create testnet client with default configuration")
    }

    /// Create a new client for local development.
    ///
    /// # Panics
    ///
    /// This method should never panic as it uses default, valid configuration.
    pub fn local() -> Self {
        ClientBuilder::new()
            .network(Network::Local)
            .build()
            .expect("Failed to create local client with default configuration")
    }

    /// Perform a GET request.
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let url_str = url.as_str().to_string();

        // Execute hooks
        for hook in &self.hooks {
            hook.before_request("GET", &url_str, None);
        }

        let response = self.http_client.get(url).send().await?;
        let status = response.status();

        let response_text = response.text().await?;

        // Execute hooks
        for hook in &self.hooks {
            hook.after_response("GET", &url_str, status.as_u16(), Some(&response_text));
        }

        if !status.is_success() {
            return Err(self.handle_error_response(status.as_u16(), &response_text));
        }

        let result: T = serde_json::from_str(&response_text)?;
        Ok(result)
    }

    /// Perform a POST request.
    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let url_str = url.as_str().to_string();

        let body_json = serde_json::to_string(body)?;

        // Execute hooks
        for hook in &self.hooks {
            hook.before_request("POST", &url_str, Some(&body_json));
        }

        let response = self
            .http_client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body_json)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        // Execute hooks
        for hook in &self.hooks {
            hook.after_response("POST", &url_str, status.as_u16(), Some(&response_text));
        }

        if !status.is_success() {
            return Err(self.handle_error_response(status.as_u16(), &response_text));
        }

        let result: T = serde_json::from_str(&response_text)?;
        Ok(result)
    }

    /// Handle error responses from the API.
    fn handle_error_response(&self, status_code: u16, body: &str) -> Error {
        // Try to parse as structured error response
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(body) {
            Error::api(
                status_code,
                error_response.error_code,
                error_response.message,
            )
        } else {
            // Fallback to generic error
            Error::api(status_code, "unknown".to_string(), body.to_string())
        }
    }
}

/// Builder for configuring and creating clients.
pub struct ClientBuilder {
    network: Option<Network>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    http_client: Option<HttpClient>,
    hooks: Vec<Box<dyn Hook>>,
}

impl Debug for ClientBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ClientBuilder")
            .field("network", &self.network)
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("hooks_count", &self.hooks.len())
            .finish()
    }
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self {
            network: None,
            base_url: None,
            timeout: None,
            http_client: None,
            hooks: Vec::new(),
        }
    }

    /// Set the network environment.
    pub fn network(mut self, network: Network) -> Self {
        self.network = Some(network);
        self
    }

    /// Set the base URL.
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set a custom HTTP client.
    pub fn http_client(mut self, client: HttpClient) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Add a hook for request/response middleware.
    pub fn hook<H: Hook + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<Client> {
        // Priority: base_url > network > default (mainnet)
        let base_url = if let Some(url) = self.base_url {
            url
        } else if let Some(network) = self.network {
            network.url().to_string()
        } else {
            Network::default().url().to_string()
        };
        let base_url = Url::parse(&base_url)?;

        let http_client = if let Some(client) = self.http_client {
            client
        } else {
            let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);
            reqwest::Client::builder()
                .timeout(timeout)
                .user_agent("onemoney-rust-sdk/0.1.0")
                .build()?
        };

        Ok(Client {
            base_url,
            http_client,
            hooks: self.hooks,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple console logger implementation.
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::Trace => {} // Skip trace messages
            LogLevel::Debug => {} // Skip debug messages
            LogLevel::Info => println!("[INFO] {}", message),
            LogLevel::Warn => println!("[WARN] {}", message),
            LogLevel::Error => println!("[ERROR] {}", message),
        }
    }
}

/// Simple request/response logging hook.
pub struct LoggingHook {
    logger: Box<dyn Logger>,
}

impl LoggingHook {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl Hook for LoggingHook {
    fn before_request(&self, method: &str, url: &str, body: Option<&str>) {
        if let Some(body) = body {
            self.logger.log(
                LogLevel::Debug,
                &format!("-> {} {} with body: {}", method, url, body),
            );
        } else {
            self.logger
                .log(LogLevel::Debug, &format!("-> {} {}", method, url));
        }
    }

    fn after_response(&self, method: &str, url: &str, status: u16, body: Option<&str>) {
        if let Some(body) = body {
            self.logger.log(
                LogLevel::Debug,
                &format!("<- {} {} [{}] body: {}", method, url, status, body),
            );
        } else {
            self.logger.log(
                LogLevel::Debug,
                &format!("<- {} {} [{}]", method, url, status),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::mainnet();
        assert!(client.base_url.as_str().starts_with(MAINNET_URL));

        let testnet_client = Client::testnet();
        assert!(testnet_client.base_url.as_str().starts_with(TESTNET_URL));

        let local_client = Client::local();
        assert!(local_client.base_url.as_str().starts_with(LOCAL_URL));
    }

    #[test]
    fn test_client_builder() {
        let client = ClientBuilder::new()
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert!(
            client
                .base_url
                .as_str()
                .starts_with("https://custom.api.com")
        );
    }

    #[test]
    fn test_client_builder_with_network() {
        let testnet_client = ClientBuilder::new()
            .network(Network::Testnet)
            .build()
            .unwrap();
        assert!(testnet_client.base_url.as_str().starts_with(TESTNET_URL));

        let local_client = ClientBuilder::new()
            .network(Network::Local)
            .build()
            .unwrap();
        assert!(local_client.base_url.as_str().starts_with(LOCAL_URL));

        // base_url should take precedence over network
        let custom_client = ClientBuilder::new()
            .network(Network::Testnet)
            .base_url("https://custom.api.com")
            .build()
            .unwrap();
        assert!(
            custom_client
                .base_url
                .as_str()
                .starts_with("https://custom.api.com")
        );
    }

    #[test]
    fn test_network_methods() {
        assert_eq!(Network::Mainnet.url(), MAINNET_URL);
        assert_eq!(Network::Testnet.url(), TESTNET_URL);
        assert_eq!(Network::Local.url(), LOCAL_URL);

        assert!(Network::Mainnet.is_production());
        assert!(!Network::Testnet.is_production());
        assert!(!Network::Local.is_production());

        assert!(!Network::Mainnet.is_test());
        assert!(Network::Testnet.is_test());
        assert!(Network::Local.is_test());

        assert_eq!(Network::default(), Network::Mainnet);
    }

    #[test]
    fn test_invalid_url() {
        let result = ClientBuilder::new().base_url("invalid-url").build();

        assert!(result.is_err());
    }
}
