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
    pub(crate) network: Network,
    http_client: HttpClient,
    hooks: Vec<Box<dyn Hook>>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("network", &self.network)
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

    /// Create a new client for custom network.
    pub fn custom(base_url: String) -> Result<Self> {
        ClientBuilder::new()
            .network(Network::Custom(base_url.into()))
            .build()
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Create a new client instance.
    pub(crate) fn new(
        network: Network,
        http_client: HttpClient,
        hooks: Vec<Box<dyn Hook>>,
    ) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(network.url())?,
            network,
            http_client,
            hooks,
        })
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

    /// Test helper method to expose handle_error_response for comprehensive testing.
    ///
    /// **This method is intended only for testing and should not be used in production code.**
    #[doc(hidden)]
    pub fn test_handle_error_response(&self, status_code: u16, body: &str) -> Error {
        self.handle_error_response(status_code, body)
    }

    /// Test helper method to expose classify_error for comprehensive testing.
    ///
    /// **This method is intended only for testing and should not be used in production code.**
    #[doc(hidden)]
    pub fn test_classify_error(status_code: u16, error_code: &str, message: &str) -> Error {
        Self::classify_error(status_code, error_code, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[allow(dead_code)]
    struct TestResponse {
        id: u64,
        message: String,
    }

    #[derive(Serialize)]
    #[allow(dead_code)]
    struct TestRequest {
        data: String,
    }

    #[test]
    fn test_client_creation_methods() {
        // Test mainnet client creation
        let mainnet_client = Client::mainnet();
        assert!(mainnet_client.is_ok());
        let client = mainnet_client.unwrap();
        assert!(client.base_url.as_str().contains("mainnet"));

        // Test testnet client creation
        let testnet_client = Client::testnet();
        assert!(testnet_client.is_ok());
        let client = testnet_client.unwrap();
        assert!(client.base_url.as_str().contains("testnet"));

        // Test local client creation
        let local_client = Client::local();
        assert!(local_client.is_ok());
        let client = local_client.unwrap();
        assert!(client.base_url.as_str().contains("127.0.0.1"));
    }

    #[test]
    fn test_client_debug_implementation() {
        let client = Client::mainnet().expect("Failed to create mainnet client");
        let debug_str = format!("{:?}", client);

        assert!(debug_str.contains("Client"));
        assert!(debug_str.contains("base_url"));
        assert!(debug_str.contains("hooks_count"));
        assert!(debug_str.contains("0")); // Default hooks count
    }

    #[test]
    fn test_error_classification_validation_errors() {
        // Test validation error classification
        let error =
            Client::test_classify_error(400, "validation_address", "Invalid address format");
        assert!(
            matches!(error, Error::InvalidParameter { parameter, .. } if parameter == "address")
        );

        let error = Client::test_classify_error(400, "validation_amount", "Invalid amount");
        assert!(
            matches!(error, Error::InvalidParameter { parameter, .. } if parameter == "amount")
        );

        let error =
            Client::test_classify_error(400, "validation_unknown", "Unknown validation error");
        assert!(
            matches!(error, Error::InvalidParameter { parameter, .. } if parameter == "unknown")
        );
    }

    #[test]
    fn test_error_classification_authentication_errors() {
        let error =
            Client::test_classify_error(401, "invalid_signature", "Signature verification failed");
        assert!(matches!(error, Error::Authentication { .. }));

        let error = Client::test_classify_error(401, "expired_token", "Token has expired");
        assert!(matches!(error, Error::Authentication { .. }));
    }

    #[test]
    fn test_error_classification_authorization_errors() {
        let error = Client::test_classify_error(403, "insufficient_permissions", "Access denied");
        assert!(matches!(error, Error::Authorization { .. }));

        let error =
            Client::test_classify_error(403, "forbidden_resource", "Resource access forbidden");
        assert!(matches!(error, Error::Authorization { .. }));
    }

    #[test]
    fn test_error_classification_resource_not_found_errors() {
        let error =
            Client::test_classify_error(404, "resource_transaction", "Transaction not found");
        assert!(
            matches!(error, Error::ResourceNotFound { resource_type, .. } if resource_type == "transaction")
        );

        let error = Client::test_classify_error(404, "resource_account", "Account not found");
        assert!(
            matches!(error, Error::ResourceNotFound { resource_type, .. } if resource_type == "account")
        );

        let error = Client::test_classify_error(404, "resource_unknown", "Resource not found");
        assert!(
            matches!(error, Error::ResourceNotFound { resource_type, .. } if resource_type == "unknown")
        );
    }

    #[test]
    fn test_error_classification_timeout_errors() {
        let error = Client::test_classify_error(408, "request_timeout", "Request timed out");
        assert!(matches!(error, Error::RequestTimeout { .. }));
    }

    #[test]
    fn test_error_classification_business_logic_errors() {
        let error =
            Client::test_classify_error(422, "business_insufficient_funds", "Insufficient balance");
        assert!(
            matches!(error, Error::BusinessLogic { operation, .. } if operation == "insufficient_funds")
        );

        let error = Client::test_classify_error(422, "business_token_paused", "Token is paused");
        assert!(
            matches!(error, Error::BusinessLogic { operation, .. } if operation == "token_paused")
        );
    }

    #[test]
    fn test_error_classification_rate_limit_errors() {
        let error = Client::test_classify_error(429, "rate_limit_exceeded", "Too many requests");
        assert!(matches!(error, Error::RateLimitExceeded { .. }));
    }

    #[test]
    fn test_error_classification_server_errors() {
        let error =
            Client::test_classify_error(500, "system_database_error", "Database connection failed");
        assert!(matches!(error, Error::HttpTransport { .. }));

        let error = Client::test_classify_error(
            503,
            "system_service_unavailable",
            "Service temporarily unavailable",
        );
        assert!(matches!(error, Error::HttpTransport { .. }));
    }

    #[test]
    fn test_error_classification_generic_api_errors() {
        // Test unknown error code
        let error = Client::test_classify_error(400, "unknown_error", "Unknown error occurred");
        assert!(
            matches!(error, Error::Api { status_code: 400, error_code, .. } if error_code == "unknown_error")
        );

        // Test unexpected status code
        let error = Client::test_classify_error(418, "teapot", "I'm a teapot");
        assert!(
            matches!(error, Error::Api { status_code: 418, error_code, .. } if error_code == "teapot")
        );
    }

    #[test]
    fn test_handle_error_response_with_structured_json() {
        let client = Client::mainnet().expect("Failed to create client");

        // Test structured error response parsing
        let structured_error =
            r#"{"error_code": "validation_address", "message": "Invalid address format"}"#;
        let error = client.test_handle_error_response(400, structured_error);
        assert!(
            matches!(error, Error::InvalidParameter { parameter, .. } if parameter == "address")
        );

        // Test structured business logic error
        let business_error = r#"{"error_code": "business_insufficient_funds", "message": "Insufficient balance for transaction"}"#;
        let error = client.test_handle_error_response(422, business_error);
        assert!(
            matches!(error, Error::BusinessLogic { operation, .. } if operation == "insufficient_funds")
        );
    }

    #[test]
    fn test_handle_error_response_fallback_to_status_code() {
        let client = Client::mainnet().expect("Failed to create client");

        // Test fallback to status code classification when JSON parsing fails
        let invalid_json = "Not a JSON response";

        let error = client.test_handle_error_response(400, invalid_json);
        assert!(matches!(error, Error::InvalidParameter { .. }));

        let error = client.test_handle_error_response(401, invalid_json);
        assert!(matches!(error, Error::Authentication { .. }));

        let error = client.test_handle_error_response(403, invalid_json);
        assert!(matches!(error, Error::Authorization { .. }));

        let error = client.test_handle_error_response(404, invalid_json);
        assert!(matches!(error, Error::ResourceNotFound { .. }));

        let error = client.test_handle_error_response(408, invalid_json);
        assert!(matches!(error, Error::RequestTimeout { .. }));

        let error = client.test_handle_error_response(422, invalid_json);
        assert!(matches!(error, Error::BusinessLogic { .. }));

        let error = client.test_handle_error_response(429, invalid_json);
        assert!(matches!(error, Error::RateLimitExceeded { .. }));

        let error = client.test_handle_error_response(500, invalid_json);
        assert!(matches!(error, Error::HttpTransport { .. }));

        let error = client.test_handle_error_response(418, invalid_json);
        assert!(matches!(
            error,
            Error::Api {
                status_code: 418,
                ..
            }
        ));
    }

    #[test]
    fn test_network_url_configuration() {
        // Test that different networks use correct base URLs
        let mainnet = Client::mainnet().unwrap();
        assert!(mainnet.base_url.as_str().contains("mainnet.1money.network"));

        let testnet = Client::testnet().unwrap();
        assert!(testnet.base_url.as_str().contains("testnet.1money.network"));

        let local = Client::local().unwrap();
        assert!(local.base_url.as_str().contains("127.0.0.1:18555"));
    }

    #[test]
    fn test_client_new_method() {
        use reqwest::Client as HttpClient;
        use url::Url;

        let base_url = Url::parse("https://test.example.com").expect("Valid URL");
        let http_client = HttpClient::new();
        let hooks: Vec<Box<dyn Hook>> = vec![];

        let client = Client::new(
            Network::Custom(base_url.to_string().into()),
            http_client,
            hooks,
        )
        .unwrap();

        assert_eq!(client.base_url, base_url);
        assert_eq!(client.hooks.len(), 0);
    }
}
