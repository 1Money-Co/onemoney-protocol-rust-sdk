//! Client builder for configuration and creation.

use super::{
    config::{Network, DEFAULT_TIMEOUT},
    hooks::Hook,
    http::Client,
};
use crate::Result;
use reqwest::Client as HttpClient;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::time::Duration;
use url::Url;

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

        Ok(Client::new(base_url, http_client, self.hooks))
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
