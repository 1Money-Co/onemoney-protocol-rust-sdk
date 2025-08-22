//! Client core functionality and configuration.

pub mod builder;
pub mod config;
pub mod hooks;
pub mod http;

// Re-export public interfaces
pub use builder::ClientBuilder;
pub use config::{Network, api_path, endpoints};
pub use hooks::{ConsoleLogger, Hook, LogLevel, Logger, LoggingHook};
pub use http::Client;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_client_module_exports() {
        // Test that all core public interfaces are exported correctly

        // Test ClientBuilder is accessible
        let builder = ClientBuilder::new();
        assert!(
            builder.build().is_ok(),
            "ClientBuilder should be accessible"
        );

        // Test Network enum is accessible
        let networks = [Network::Mainnet, Network::Testnet, Network::Local];
        for network in networks {
            assert!(
                !network.url().is_empty(),
                "Network URLs should not be empty"
            );
        }

        // Test api_path function is accessible
        let path = api_path("/test");
        assert!(
            path.starts_with("/v1"),
            "API path should have version prefix"
        );

        // Test Hook trait is accessible (through LoggingHook)
        let logger = ConsoleLogger;
        let _hook = LoggingHook::new(Box::new(logger));
        // Hook functionality is accessible if compilation succeeds
    }

    #[test]
    fn test_client_creation_workflow() {
        // Test the complete client creation workflow using re-exported interfaces
        let client = ClientBuilder::new()
            .network(Network::Local)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Client creation should work with re-exported interfaces");

        let debug_str = format!("{:?}", client);
        assert!(
            debug_str.contains("Client"),
            "Client should be properly created"
        );
    }

    #[test]
    fn test_endpoints_accessibility() {
        // Test that endpoint constants are accessible through re-exports
        use endpoints::*;

        // Test various endpoint modules
        assert_eq!(chains::CHAIN_ID, "/chains/chain_id");
        assert_eq!(
            states::LATEST_EPOCH_CHECKPOINT,
            "/states/latest_epoch_checkpoint"
        );
        assert_eq!(tokens::MINT, "/tokens/mint");
        assert_eq!(transactions::PAYMENT, "/transactions/payment");
        assert_eq!(accounts::NONCE, "/accounts/nonce");
    }

    #[test]
    fn test_logging_functionality() {
        // Test that logging functionality works through re-exports
        let logger = ConsoleLogger;
        let _hook = LoggingHook::new(Box::new(logger));

        // Should not panic when creating logging components
        // Logging components are accessible if compilation succeeds

        // Test LogLevel enum
        let levels = [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
        ];
        for _level in levels {
            let _logger = ConsoleLogger;
            // All log levels are creatable if compilation succeeds
        }
    }

    #[test]
    fn test_network_configuration() {
        // Test that Network configuration works correctly
        for network in [Network::Mainnet, Network::Testnet, Network::Local] {
            let client = ClientBuilder::new()
                .network(network)
                .build()
                .expect("Network configuration should work");

            let debug_str = format!("{:?}", client);
            assert!(debug_str.contains("Client"));

            // Test network properties
            let url = network.url();
            assert!(!url.is_empty(), "Network URL should not be empty");
            assert!(
                url.starts_with("http"),
                "Network URL should be valid HTTP(S) URL"
            );
        }
    }

    #[test]
    fn test_module_organization() {
        // Test that module organization is logical and accessible

        // All core functionality should be available
        let _builder = ClientBuilder::new();
        let _network = Network::default();
        let _logger = ConsoleLogger;

        // API path construction should work
        let path = api_path("/test/endpoint");
        assert_eq!(path, "/v1/test/endpoint");

        // Endpoint constants should be organized properly
        assert!(endpoints::tokens::MINT.contains("tokens"));
        assert!(endpoints::chains::CHAIN_ID.contains("chains"));
        assert!(endpoints::states::LATEST_EPOCH_CHECKPOINT.contains("states"));
    }
}
