//! Environment management for OneMoney SDK examples.
//!
//! This module provides centralized control over which network environment
//! all examples use. Change the DEFAULT_ENVIRONMENT constant to switch
//! between mainnet, testnet, and local environments globally.

use onemoney_protocol::{Client, Error};
use std::env;

/// Supported network environments for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleEnvironment {
    /// Production mainnet environment - use with real funds.
    Mainnet,
    /// Test network environment - safe for testing and development.
    Testnet,
    /// Local development environment - requires local node running.
    Local,
}

impl ExampleEnvironment {
    /// Get the human-readable name of this environment.
    pub fn name(self) -> &'static str {
        match self {
            ExampleEnvironment::Mainnet => "Mainnet",
            ExampleEnvironment::Testnet => "Testnet",
            ExampleEnvironment::Local => "Local",
        }
    }

    /// Get a description of this environment.
    pub fn description(self) -> &'static str {
        match self {
            ExampleEnvironment::Mainnet => "Production network with real value",
            ExampleEnvironment::Testnet => "Test network for development and testing",
            ExampleEnvironment::Local => "Local development network",
        }
    }

    /// Check if this environment is safe for testing.
    pub fn is_safe_for_testing(self) -> bool {
        matches!(
            self,
            ExampleEnvironment::Testnet | ExampleEnvironment::Local
        )
    }

    /// Check if this environment requires real funds.
    pub fn requires_real_funds(self) -> bool {
        matches!(self, ExampleEnvironment::Mainnet)
    }
}

/// Default environment for all examples.
///
/// **CHANGE THIS TO SWITCH ALL EXAMPLES TO A DIFFERENT ENVIRONMENT**
///
/// Options:
/// - `ExampleEnvironment::Mainnet` - Production network (use with caution!)
/// - `ExampleEnvironment::Testnet` - Test network (recommended for development)
/// - `ExampleEnvironment::Local` - Local development network
const DEFAULT_ENVIRONMENT: ExampleEnvironment = ExampleEnvironment::Testnet;

/// Get the current environment for examples.
///
/// This checks for the ONEMONEY_EXAMPLE_ENV environment variable first,
/// then falls back to the DEFAULT_ENVIRONMENT constant.
///
/// Environment variable values:
/// - "mainnet" or "MAINNET" -> Mainnet
/// - "testnet" or "TESTNET" -> Testnet
/// - "local" or "LOCAL" -> Local
///
/// # Example
///
/// ```bash
/// # Run with testnet (default)
/// cargo run --example transactions_example
///
/// # Run with mainnet via environment variable
/// ONEMONEY_EXAMPLE_ENV=mainnet cargo run --example transactions_example
///
/// # Run with local via environment variable
/// ONEMONEY_EXAMPLE_ENV=local cargo run --example transactions_example
/// ```
pub fn get_example_environment() -> ExampleEnvironment {
    if let Ok(env_var) = env::var("ONEMONEY_EXAMPLE_ENV") {
        match env_var.to_lowercase().as_str() {
            "mainnet" => ExampleEnvironment::Mainnet,
            "testnet" => ExampleEnvironment::Testnet,
            "local" => ExampleEnvironment::Local,
            _ => {
                eprintln!(
                    "WARNING: Unknown ONEMONEY_EXAMPLE_ENV value '{}', using default",
                    env_var
                );
                DEFAULT_ENVIRONMENT
            }
        }
    } else {
        DEFAULT_ENVIRONMENT
    }
}

/// Create a client using the current example environment.
///
/// This is the main function that all examples should use to create their client.
/// It automatically uses the configured environment.
#[allow(dead_code)]
pub fn create_example_client() -> Client {
    let env = get_example_environment();
    match env {
        ExampleEnvironment::Mainnet => Client::mainnet(),
        ExampleEnvironment::Testnet => Client::testnet(),
        ExampleEnvironment::Local => Client::local(),
    }
}

/// Print environment information banner for examples.
///
/// This should be called at the start of each example to inform the user
/// which environment is being used.
pub fn print_environment_banner(example_name: &str) {
    let env = get_example_environment();

    println!("=== OneMoney SDK Example: {} ===", example_name);
    println!("Environment: {} ({})", env.name(), env.description());

    if env.requires_real_funds() {
        println!("WARNING: This is MAINNET - real funds will be used!");
        println!("Make sure you understand the implications before proceeding.");
    } else if env == ExampleEnvironment::Local {
        println!("NOTE: Using local environment - ensure your local node is running");
        println!("Local node should be accessible at http://127.0.0.1:18555/");
    }

    if env.is_safe_for_testing() {
        println!("Safe for testing and development");
    }

    println!("To change environment:");
    println!("  1. Set DEFAULT_ENVIRONMENT in examples/environment.rs");
    println!(
        "  2. Or use: ONEMONEY_EXAMPLE_ENV=mainnet|testnet|local cargo run --example {}",
        example_name.replace("_example", "")
    );
    println!();
}

/// Configuration for example addresses and test data.
///
/// This provides consistent test addresses across all examples while
/// making it easy to change them in one place.
#[allow(dead_code)]
pub struct ExampleConfig {
    /// Example wallet address for demonstrations.
    pub wallet_address: &'static str,
    /// Example recipient address for transactions.
    pub recipient_address: &'static str,
    /// Example token mint address.
    pub token_mint_address: &'static str,
    /// Example private key (DO NOT use in production!).
    pub private_key: &'static str,
}

impl ExampleConfig {
    /// Get example configuration suitable for the current environment.
    #[allow(dead_code)]
    pub fn get() -> Self {
        let env = get_example_environment();

        match env {
            ExampleEnvironment::Mainnet => {
                // For mainnet, use more generic placeholder addresses
                // Users should replace these with their own addresses
                ExampleConfig {
                    wallet_address: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
                    recipient_address: "0x1234567890abcdef1234567890abcdef12345678",
                    token_mint_address: "0x9876543210fedcba9876543210fedcba98765432",
                    private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                }
            }
            ExampleEnvironment::Testnet => {
                // For testnet, use test-specific addresses that are safe to use
                ExampleConfig {
                    wallet_address: "0x5B630881f7c7c2d67577848A28C4d7483874aF33",
                    recipient_address: "0xc5c795223c69f48166b0ab12f081ce7a908b7786",
                    token_mint_address: "0xecc78602d4886808cb4a103e83265207e04353d3",
                    private_key: "0x6b4be1d8d5497a6cbf90d0f421cffd7e5fe855e19b177f5814abd084295424b7",
                }
            }
            ExampleEnvironment::Local => {
                // For local, use addresses that work with local development setup
                ExampleConfig {
                    wallet_address: "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0",
                    recipient_address: "0x1234567890abcdef1234567890abcdef12345678",
                    token_mint_address: "0x9876543210fedcba9876543210fedcba98765432",
                    private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                }
            }
        }
    }

    /// Print a warning about the example configuration.
    #[allow(dead_code)]
    pub fn print_config_warning(&self) {
        let env = get_example_environment();

        println!("Example Configuration:");
        println!("  Wallet: {}", self.wallet_address);
        println!("  Recipient: {}", self.recipient_address);
        println!("  Token Mint: {}", self.token_mint_address);

        if env.requires_real_funds() {
            println!();
            println!("IMPORTANT: Replace these example addresses with your own for mainnet!");
            println!("NEVER use the example private key on mainnet - it's publicly known!");
        } else {
            println!(
                "  Private Key: {}...{}",
                &self.private_key[..10],
                &self.private_key[self.private_key.len() - 4..]
            );
            println!(
                "NOTE: Example addresses and keys are safe to use on {}",
                env.name()
            );
        }
        println!();
    }
}

/// Format and print error information with detailed breakdown for API errors.
#[allow(dead_code)]
pub fn print_detailed_error(context: &str, error: &Error) {
    if let Some(status_code) = error.status_code() {
        if let Some(error_code) = error.error_code() {
            // This is an API error with detailed information
            println!(
                "ERROR: {} - HTTP {}: {} - {}",
                context, status_code, error_code, error
            );
        } else {
            // This is an API error but without detailed error_code (shouldn't happen)
            println!("ERROR: {} - HTTP {}: {}", context, status_code, error);
        }
    } else {
        // This is not an API error (network, parsing, etc.)
        println!("ERROR: {} - {}", context, error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_properties() {
        assert_eq!(ExampleEnvironment::Mainnet.name(), "Mainnet");
        assert_eq!(ExampleEnvironment::Testnet.name(), "Testnet");
        assert_eq!(ExampleEnvironment::Local.name(), "Local");

        assert!(!ExampleEnvironment::Mainnet.is_safe_for_testing());
        assert!(ExampleEnvironment::Testnet.is_safe_for_testing());
        assert!(ExampleEnvironment::Local.is_safe_for_testing());

        assert!(ExampleEnvironment::Mainnet.requires_real_funds());
        assert!(!ExampleEnvironment::Testnet.requires_real_funds());
        assert!(!ExampleEnvironment::Local.requires_real_funds());
    }

    #[test]
    fn test_default_environment() {
        // Should be testnet for safety
        assert_eq!(DEFAULT_ENVIRONMENT, ExampleEnvironment::Testnet);
    }

    #[test]
    fn test_example_config() {
        let config = ExampleConfig::get();

        // Addresses should be valid format (start with 0x and have correct length)
        assert!(config.wallet_address.starts_with("0x"));
        assert!(config.recipient_address.starts_with("0x"));
        assert!(config.token_mint_address.starts_with("0x"));
        assert!(config.private_key.starts_with("0x"));

        assert_eq!(config.wallet_address.len(), 42); // 0x + 40 hex chars
        assert_eq!(config.recipient_address.len(), 42);
        assert_eq!(config.token_mint_address.len(), 42);
        assert_eq!(config.private_key.len(), 66); // 0x + 64 hex chars
    }
}
