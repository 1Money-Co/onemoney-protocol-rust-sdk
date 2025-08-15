//! State management example for the OneMoney Rust SDK.
//!
//! This example demonstrates state-related operations:
//! - Getting latest epoch and checkpoint information

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{create_example_client, print_detailed_error, print_environment_banner};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("State Management");
    let client = create_example_client();

    // 1. Get Latest Epoch and Checkpoint
    println!("\n1. Get Latest Epoch and Checkpoint");
    println!("===================================");

    match client.get_latest_epoch_checkpoint().await {
        Ok(state) => {
            println!("{}", state);
        }
        Err(e) => {
            print_detailed_error("Could not get latest state", &e);
            return Err(e.into());
        }
    }

    Ok(())
}
