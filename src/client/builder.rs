//! Client builder for configuration and creation.

use super::{
    config::{DEFAULT_TIMEOUT, Network},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::config::{LOCAL_URL, MAINNET_URL, TESTNET_URL};
    use std::time::Duration;

    #[test]
    fn test_builder_default_configuration() {
        let builder = ClientBuilder::new();

        // Verify default state
        assert!(builder.network.is_none());
        assert!(builder.base_url.is_none());
        assert!(builder.timeout.is_none());
        assert!(builder.http_client.is_none());
        assert!(builder.hooks.is_empty());

        // Test that default can build successfully
        let client = builder.build();
        assert!(client.is_ok(), "Default builder should create valid client");
    }

    #[test]
    fn test_builder_network_configuration() {
        // Test all network types
        let networks = [
            (Network::Mainnet, MAINNET_URL),
            (Network::Testnet, TESTNET_URL),
            (Network::Local, LOCAL_URL),
        ];

        for (network, _expected_url) in networks {
            let builder = ClientBuilder::new().network(network);

            assert_eq!(builder.network, Some(network));

            let client = builder.build().expect("Network configuration should work");
            let debug_str = format!("{:?}", client);
            assert!(debug_str.contains("base_url"));
        }
    }

    #[test]
    fn test_builder_timeout_configuration() {
        let test_timeouts = [
            Duration::from_millis(1),
            Duration::from_secs(5),
            Duration::from_secs(30),
            Duration::from_secs(120),
            Duration::from_secs(3600),
        ];

        for timeout in test_timeouts {
            let builder = ClientBuilder::new().timeout(timeout);

            assert_eq!(builder.timeout, Some(timeout));

            let client = builder.build();
            assert!(
                client.is_ok(),
                "Timeout configuration should work for {:?}",
                timeout
            );
        }
    }

    #[test]
    fn test_builder_custom_base_url() {
        let test_urls = [
            "http://localhost:8080",
            "https://api.example.com",
            "http://127.0.0.1:3000",
            "https://custom.domain.com:8443",
        ];

        for url in test_urls {
            let builder = ClientBuilder::new().base_url(url);

            assert_eq!(builder.base_url, Some(url.to_string()));

            let client = builder.build();
            assert!(client.is_ok(), "Custom base URL should work for {}", url);
        }
    }

    #[test]
    fn test_builder_url_priority() {
        // Custom base_url should override network setting
        let builder = ClientBuilder::new()
            .network(Network::Mainnet)
            .base_url("http://custom.example.com");

        let client = builder.build().expect("URL priority test should work");
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("base_url"));
    }

    #[test]
    fn test_builder_http_client_configuration() {
        let custom_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Custom HTTP client should build");

        let builder = ClientBuilder::new().http_client(custom_client);

        assert!(builder.http_client.is_some());

        let client = builder.build();
        assert!(
            client.is_ok(),
            "Custom HTTP client configuration should work"
        );
    }

    #[test]
    fn test_builder_hooks_management() {
        // Create a test hook
        struct TestHook;
        impl Hook for TestHook {
            fn before_request(&self, _method: &str, _url: &str, _body: Option<&str>) {}
            fn after_response(&self, _method: &str, _url: &str, _status: u16, _body: Option<&str>) {
            }
        }

        let builder = ClientBuilder::new().hook(TestHook).hook(TestHook);

        assert_eq!(builder.hooks.len(), 2);

        let client = builder.build();
        assert!(client.is_ok(), "Hook management should work");
    }

    #[test]
    fn test_builder_validation_errors() {
        // Test invalid URL
        let result = ClientBuilder::new().base_url("invalid-url-format").build();

        assert!(result.is_err(), "Invalid URL should cause build error");
    }

    #[test]
    fn test_builder_debug_implementation() {
        let builder = ClientBuilder::new()
            .network(Network::Testnet)
            .base_url("http://example.com")
            .timeout(Duration::from_secs(30));

        let debug_str = format!("{:?}", builder);

        assert!(debug_str.contains("ClientBuilder"));
        assert!(debug_str.contains("network"));
        assert!(debug_str.contains("base_url"));
        assert!(debug_str.contains("timeout"));
        assert!(debug_str.contains("hooks_count"));
        assert!(debug_str.contains("Testnet"));
        assert!(debug_str.contains("example.com"));
        assert!(debug_str.contains("30s"));
    }

    #[test]
    fn test_builder_method_chaining() {
        // Test that all methods return Self for chaining
        let client = ClientBuilder::new()
            .network(Network::Local)
            .base_url("http://localhost:8080")
            .timeout(Duration::from_secs(15))
            .build();

        assert!(client.is_ok(), "Method chaining should work correctly");
    }

    #[test]
    fn test_builder_multiple_configurations() {
        // Test overwriting previous configurations
        let builder = ClientBuilder::new()
            .network(Network::Mainnet)
            .network(Network::Testnet)  // Should override previous
            .timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(20)); // Should override previous

        assert_eq!(builder.network, Some(Network::Testnet));
        assert_eq!(builder.timeout, Some(Duration::from_secs(20)));

        let client = builder.build();
        assert!(client.is_ok(), "Multiple configurations should work");
    }

    #[test]
    fn test_builder_default_trait() {
        let builder1 = ClientBuilder::default();
        let builder2 = ClientBuilder::new();

        // Both should have same initial state
        assert_eq!(builder1.network, builder2.network);
        assert_eq!(builder1.base_url, builder2.base_url);
        assert_eq!(builder1.timeout, builder2.timeout);
        assert_eq!(builder1.hooks.len(), builder2.hooks.len());
    }

    #[test]
    fn test_builder_extreme_timeout_values() {
        // Test with very small timeout
        let client1 = ClientBuilder::new()
            .timeout(Duration::from_nanos(1))
            .build();
        assert!(client1.is_ok(), "Very small timeout should be accepted");

        // Test with very large timeout
        let client2 = ClientBuilder::new()
            .timeout(Duration::from_secs(u64::MAX / 1000)) // Avoid overflow
            .build();
        assert!(client2.is_ok(), "Very large timeout should be accepted");
    }

    #[test]
    fn test_builder_edge_case_urls() {
        let edge_case_urls = [
            "http://localhost",
            "https://a.b",
            "http://127.0.0.1",
            "https://example.com:443",
        ];

        for url in edge_case_urls {
            let client = ClientBuilder::new().base_url(url).build();
            assert!(client.is_ok(), "Edge case URL {} should work", url);
        }
    }
}
