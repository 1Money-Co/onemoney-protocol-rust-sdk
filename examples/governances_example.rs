//! Governance example for the OneMoney Rust SDK.
//!
//! This example demonstrates governance-related operations including:
//! - Getting epoch information by ID
//! - Understanding certificate types (Genesis vs Epoch)
//! - Viewing validator set information
//! - Inspecting operator details

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{create_example_client, print_detailed_error, print_environment_banner};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Governance Management");
    let client = create_example_client();

    // 1. Get Current Epoch (epoch 0 is genesis)
    println!("\n1. Get Genesis Epoch (Epoch 0)");
    println!("================================");

    match client.get_epoch_by_id(0).await {
        Ok(epoch) => {
            println!("{}", epoch);
        }
        Err(e) => {
            print_detailed_error("Could not get genesis epoch", &e);
            println!("   This usually means the network is not properly configured");
        }
    }

    // 2. Get Recent Epoch
    println!("\n2. Get Recent Epoch (Epoch 1)");
    println!("==============================");

    match client.get_epoch_by_id(1).await {
        Ok(epoch) => {
            println!("{}", epoch);
        }
        Err(e) => {
            print_detailed_error("Could not get epoch 1", &e);
            println!("   The network may not have advanced beyond genesis yet");
        }
    }

    // 3. Error Handling Examples
    println!("\n3. Error Handling Examples");
    println!("==========================");

    // Try to get a non-existent epoch (far in the future)
    let future_epoch = 999999;
    println!("Testing non-existent epoch (epoch {}):", future_epoch);
    match client.get_epoch_by_id(future_epoch).await {
        Ok(_) => {
            println!("   Surprisingly, epoch {} exists!", future_epoch);
        }
        Err(e) => {
            print_detailed_error("Expected error for non-existent epoch", &e);
        }
    }

    println!("\nGovernance examples completed successfully!");

    Ok(())
}
