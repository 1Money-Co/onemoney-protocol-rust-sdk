//! Retry logic and error handling utilities.

use std::time::Duration;

/// Retry configuration for HTTP requests.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_attempts: u32,
    /// Initial delay between retries.
    pub initial_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
    /// Multiplier for exponential backoff.
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of retry attempts.
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set the initial delay between retries.
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the maximum delay between retries.
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the backoff multiplier for exponential backoff.
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Calculate the delay for the given attempt number.
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let delay_ms = self.initial_delay.as_millis() as f64
            * self.backoff_multiplier.powi((attempt - 1) as i32);

        let delay = Duration::from_millis(delay_ms as u64);

        if delay > self.max_delay {
            self.max_delay
        } else {
            delay
        }
    }

    /// Check if the given attempt number should be retried.
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

/// Check if a HTTP status code indicates a retryable error.
pub fn is_retryable_status(status: u16) -> bool {
    matches!(
        status,
        429 |           // Too Many Requests
        500..=599 // Server errors
    )
}

/// Check if an error is retryable.
pub fn is_retryable_error(error: &reqwest::Error) -> bool {
    // Retry on network errors, timeouts, etc.
    error.is_timeout() || error.is_connect() || error.is_request()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(100));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.backoff_multiplier, 2.0);
    }

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::new()
            .max_attempts(5)
            .initial_delay(Duration::from_millis(200))
            .max_delay(Duration::from_secs(30))
            .backoff_multiplier(1.5);

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.initial_delay, Duration::from_millis(200));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.backoff_multiplier, 1.5);
    }

    #[test]
    fn test_delay_calculation() {
        let config = RetryConfig::new()
            .initial_delay(Duration::from_millis(100))
            .backoff_multiplier(2.0)
            .max_delay(Duration::from_secs(5));

        assert_eq!(config.delay_for_attempt(0), Duration::ZERO);
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(100));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(200));
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(400));

        // Should cap at max_delay
        let long_delay = config.delay_for_attempt(10);
        assert!(long_delay <= config.max_delay);
    }

    #[test]
    fn test_should_retry() {
        let config = RetryConfig::new().max_attempts(3);

        assert!(config.should_retry(0));
        assert!(config.should_retry(1));
        assert!(config.should_retry(2));
        assert!(!config.should_retry(3));
        assert!(!config.should_retry(4));
    }

    #[test]
    fn test_is_retryable_status() {
        assert!(!is_retryable_status(200)); // OK
        assert!(!is_retryable_status(400)); // Bad Request
        assert!(!is_retryable_status(404)); // Not Found
        assert!(is_retryable_status(429)); // Too Many Requests
        assert!(is_retryable_status(500)); // Internal Server Error
        assert!(is_retryable_status(502)); // Bad Gateway
        assert!(is_retryable_status(503)); // Service Unavailable
    }
}
