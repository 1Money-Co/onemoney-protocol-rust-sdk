//! HTTP client implementation.

use super::{builder::ClientBuilder, config::Network, hooks::Hook};
use crate::{Error, Result, error::ErrorResponse};
use reqwest::{Client as HttpClient, header};
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
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
    pub fn mainnet() -> Result<Self> {
        ClientBuilder::new().network(Network::Mainnet).build()
    }

    /// Create a new client for testnet.
    pub fn testnet() -> Result<Self> {
        ClientBuilder::new().network(Network::Testnet).build()
    }

    /// Create a new client for local development.
    pub fn local() -> Result<Self> {
        ClientBuilder::new().network(Network::Local).build()
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
        // Try to parse as structured error response first (L1 compatible)
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(body) {
            // Classify error based on status code and error code
            Self::classify_error(
                status_code,
                &error_response.error_code,
                &error_response.message,
            )
        } else {
            // Fallback based on status code
            match status_code {
                400 => Error::invalid_parameter("request", body),
                401 => Error::authentication(body),
                403 => Error::authorization(body),
                404 => Error::resource_not_found("unknown", body),
                408 => Error::request_timeout("unknown", 0),
                422 => Error::business_logic("validation", body),
                429 => Error::rate_limit_exceeded(None),
                500..=599 => Error::http_transport(body, Some(status_code)),
                _ => Error::api(status_code, "unknown".to_string(), body.to_string()),
            }
        }
    }

    /// Classify structured errors based on L1 error patterns.
    fn classify_error(status_code: u16, error_code: &str, message: &str) -> Error {
        match (status_code, error_code) {
            // 400 Bad Request - Validation Errors
            (400, code) if code.starts_with("validation_") => {
                let param = code.strip_prefix("validation_").unwrap_or("unknown");
                Error::invalid_parameter(param, message)
            }

            // 401 Unauthorized
            (401, _) => Error::authentication(message),

            // 403 Forbidden
            (403, _) => Error::authorization(message),

            // 404 Not Found - Resource Errors
            (404, code) if code.starts_with("resource_") => {
                let resource = code.strip_prefix("resource_").unwrap_or("unknown");
                Error::resource_not_found(resource, message)
            }

            // 408 Request Timeout
            (408, "request_timeout") => Error::request_timeout(message, 0),

            // 422 Unprocessable Entity - Business Logic
            (422, code) if code.starts_with("business_") => {
                let operation = code.strip_prefix("business_").unwrap_or("unknown");
                Error::business_logic(operation, message)
            }

            // 429 Too Many Requests
            (429, "rate_limit_exceeded") => Error::rate_limit_exceeded(None),

            // 500+ Server Errors
            (500..=599, code) if code.starts_with("system_") => {
                Error::http_transport(message, Some(status_code))
            }

            // Default to generic API error
            _ => Error::api(status_code, error_code.to_string(), message.to_string()),
        }
    }
}
