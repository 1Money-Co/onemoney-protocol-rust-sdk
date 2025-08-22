//! HTTP transport layer for API communication.

pub mod retry;

// Re-export public interfaces
pub use retry::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_transport_module_exports() {
        // Test that all public interfaces are properly exported
        let _config = RetryConfig::default();
        // Module exports are accessible if compilation succeeds
    }

    #[test]
    fn test_retry_config_accessibility() {
        // Test that RetryConfig can be created and used
        let config = RetryConfig::new();

        // RetryConfig should be accessible
        // RetryConfig is accessible if compilation succeeds

        // Test that we can use the config (even if it's just default)
        let _default_config = config;
        // RetryConfig is usable if compilation succeeds
    }

    #[test]
    fn test_module_structure() {
        // Test that module structure is as expected
        // This ensures the module organization remains stable
        let config = RetryConfig::default();
        assert!(
            config.max_attempts > 0,
            "Default retry config should have retries"
        );
        assert!(
            config.initial_delay > Duration::ZERO,
            "Default should have positive delay"
        );
    }
}
