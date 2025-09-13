//! Network selection examples for the OneMoney Rust SDK.
//!
//! This example demonstrates:
//! - Different network configurations (Mainnet, Testnet, Local)
//! - Testing network connectivity
//! - Network information retrieval

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{
    create_example_client, get_example_environment, print_detailed_error, print_environment_banner,
};
use onemoney_protocol::{ClientBuilder, Network};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_environment_banner("Network Selection");
    let current_env = get_example_environment();
    let client = create_example_client();

    println!("Current configured environment: {}", current_env.name());
    println!("Using centralized configuration for all examples\n");

    // 1. Network Configuration Overview
    println!("1. Network Configuration Overview");
    println!("=================================");

    let current_network = match current_env {
        environment::ExampleEnvironment::Mainnet => Network::Mainnet,
        environment::ExampleEnvironment::Testnet => Network::Testnet,
        environment::ExampleEnvironment::Local => Network::Local,
    };

    println!("Available networks:");
    println!(
        "   - Mainnet: {} (Production network)",
        Network::Mainnet.url()
    );
    println!("   - Testnet: {} (Test network)", Network::Testnet.url());
    println!("   - Local: {} (Local development)", Network::Local.url());
    println!();
    println!(
        "Currently using: {:?} -> {}",
        current_network,
        current_network.url()
    );

    // 2. Test Network Connectivity
    println!("\n2. Test Network Connectivity");
    println!("============================");

    match client.fetch_chain_id_from_network().await {
        Ok(chain_id) => {
            println!("Connected to {} network", current_env.name());
            println!("   Chain ID: {}", chain_id);
        }
        Err(e) => {
            print_detailed_error(
                &format!("Failed to connect to {} network", current_env.name()),
                &e,
            );
            println!("   This might indicate network issues or incorrect configuration");
        }
    }

    // 3. Get Network State Information
    println!("\n3. Network State Information");
    println!("============================");

    match client.get_latest_epoch_checkpoint().await {
        Ok(state) => {
            println!("{}", state);
        }
        Err(e) => {
            print_detailed_error("Could not get network state", &e);
        }
    }

    // 4. ClientBuilder Examples
    println!("\n4. ClientBuilder Configuration Examples");
    println!("=======================================");

    println!("Creating clients for different networks:");

    // Mainnet client
    println!("\n   - Mainnet client:");
    match ClientBuilder::new()
        .network(Network::Mainnet)
        .timeout(Duration::from_secs(30))
        .build()
    {
        Ok(mainnet_client) => {
            println!("     Mainnet client created -> {}", Network::Mainnet.url());
            // Test connectivity (but don't fail if mainnet is unreachable)
            match mainnet_client.fetch_chain_id_from_network().await {
                Ok(chain_id) => println!("     Mainnet Chain ID: {}", chain_id),
                Err(_) => println!("     Mainnet not reachable (this is normal in development)"),
            }
        }
        Err(e) => {
            print_detailed_error("Failed to create mainnet client", &e);
        }
    }

    // Testnet client
    println!("\n   - Testnet client:");
    match ClientBuilder::new()
        .network(Network::Testnet)
        .timeout(Duration::from_secs(30))
        .build()
    {
        Ok(testnet_client) => {
            println!("     Testnet client created -> {}", Network::Testnet.url());
            match testnet_client.fetch_chain_id_from_network().await {
                Ok(chain_id) => println!("     Testnet Chain ID: {}", chain_id),
                Err(_) => println!("     Testnet not reachable"),
            }
        }
        Err(e) => {
            print_detailed_error("Failed to create testnet client", &e);
        }
    }

    // Custom URL client
    println!("\n   - Custom URL client:");
    println!("     Example: ClientBuilder::new()");
    println!("         .network(Network::Custom(\"https://my-custom-node.example.com\".into()))");
    println!("         .timeout(Duration::from_secs(60))");
    println!("         .build()?;");

    // 5. Network Switching Example
    println!("\n5. Network Switching Example");
    println!("============================");

    println!("To switch networks, change the environment variable:");
    println!("   export ONEMONEY_EXAMPLE_ENV=mainnet    # Use mainnet");
    println!("   export ONEMONEY_EXAMPLE_ENV=testnet    # Use testnet");
    println!("   export ONEMONEY_EXAMPLE_ENV=local      # Use local node");
    println!();
    println!("Or modify DEFAULT_ENVIRONMENT in examples/common.rs");

    // 6. Error Handling Examples
    println!("\n6. Error Handling Examples");
    println!("==========================");

    println!("Testing with invalid URL:");
    match ClientBuilder::new()
        .network(Network::Custom(
            "https://invalid-url-that-does-not-exist.example.com".into(),
        ))
        .timeout(Duration::from_secs(5))
        .build()
    {
        Ok(invalid_client) => match invalid_client.fetch_chain_id_from_network().await {
            Ok(_) => println!("   Unexpected success with invalid URL"),
            Err(e) => {
                print_detailed_error("Expected error with invalid URL", &e);
            }
        },
        Err(e) => {
            print_detailed_error("Client creation failed", &e);
        }
    }

    println!("\nNetwork Examples Completed!");
    println!("===========================");
    println!("Available Networks:");
    println!("   - Mainnet: Production network with real assets");
    println!("   - Testnet: Test network for development and testing");
    println!("   - Local: Local development node");
    println!("\nNetwork Configuration:");
    println!("   - Use environment variables or modify common.rs");
    println!("   - ClientBuilder provides flexible configuration");
    println!("   - Always test connectivity before production use");
    println!("\nNext steps:");
    println!("   - Check out transactions_example.rs for API examples");
    println!("   - See states_example.rs for network state information");
    println!("   - Review chains_example.rs for chain-specific operations");

    Ok(())
}
