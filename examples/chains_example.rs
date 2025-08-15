//! Chain information example for the OneMoney Rust SDK.
//!
//! This example demonstrates chain-related operations:
//! - Getting chain ID from the API

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

    // 1. Get Chain ID
    println!("\n1. Get Chain ID");
    println!("===============");

    match client.get_chain_id().await {
        Ok(chain_id) => {
            println!("Chain ID: {}", chain_id);
        }
        Err(e) => {
            print_detailed_error("Could not get chain ID", &e);
        }
    }

    Ok(())
}
