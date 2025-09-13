//! Chain information example for the OneMoney Rust SDK.
//!
//! This example demonstrates chain-related operations:
//! - Getting predefined chain ID (fast, no network request)
//! - Fetching chain ID from the network API (with network verification)

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{create_example_client, print_detailed_error, print_environment_banner};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Chain Information");
    let client = create_example_client();

    // 1. Get Predefined Chain ID (Fast)
    println!("\n1. Get Predefined Chain ID");
    println!("==========================");
    let chain_id = client.predefined_chain_id();
    println!("Expected chain ID: {}", chain_id);

    // 2. Fetch Chain ID from Network
    println!("\n2. Fetch Chain ID from Network");
    println!("==============================");

    match client.fetch_chain_id_from_network().await {
        Ok(api_chain_id) => {
            println!("API chain ID: {}", api_chain_id);
            if api_chain_id == chain_id {
                println!("Chain ID matches expected value!");
            } else {
                println!(
                    "WARNING: Chain ID mismatch! Expected {}, got {}",
                    chain_id, api_chain_id
                );
            }
        }
        Err(e) => {
            print_detailed_error("Could not fetch chain ID from network", &e);
            println!("Using expected chain ID: {}", chain_id);
        }
    }

    Ok(())
}
