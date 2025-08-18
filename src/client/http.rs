//! HTTP client implementation.

use super::{builder::ClientBuilder, config::Network, hooks::Hook};
use crate::{error::ErrorResponse, Error, Result};
use reqwest::{header, Client as HttpClient};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use url::Url;

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

    /// Create a new client instance.
    pub(crate) fn new(base_url: Url, http_client: HttpClient, hooks: Vec<Box<dyn Hook>>) -> Self {
        Self {
            base_url,
            http_client,
            hooks,
        }
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
