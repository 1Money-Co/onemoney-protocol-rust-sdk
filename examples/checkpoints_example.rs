//! Checkpoint management example for the OneMoney Rust SDK.
//!
//! This example demonstrates checkpoint-related operations including:
//! - Getting checkpoint by number
//! - Getting checkpoint by hash
//! - Getting the latest checkpoint number
//! - Getting checkpoint receipts

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{create_example_client, print_detailed_error, print_environment_banner};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Checkpoint Management");
    let client = create_example_client();

    // 1. Get Latest Checkpoint Number
    println!("\n1. Get Latest Checkpoint Number");
    println!("================================");

    let latest_checkpoint_number = match client.get_checkpoint_number().await {
        Ok(checkpoint_number) => {
            println!("Latest {}", checkpoint_number);
            checkpoint_number.number
        }
        Err(e) => {
            print_detailed_error("Could not get latest checkpoint number", &e);
            return Err(e.into());
        }
    };

    // 2. Get Checkpoint by Number (without full transactions)
    println!("\n2. Get Checkpoint by Number (hashes only)");
    println!("==========================================");

    let checkpoint_hash = match client
        .get_checkpoint_by_number(latest_checkpoint_number, false)
        .await
    {
        Ok(checkpoint) => {
            println!("{}", checkpoint);
            checkpoint.hash.clone()
        }
        Err(e) => {
            print_detailed_error(
                &format!("Could not get checkpoint {}", latest_checkpoint_number),
                &e,
            );
            return Err(e.into());
        }
    };

    // 3. Get Checkpoint by Number (with full transactions)
    println!("\n3. Get Checkpoint by Number (full transactions)");
    println!("================================================");

    match client
        .get_checkpoint_by_number(latest_checkpoint_number, true)
        .await
    {
        Ok(checkpoint) => {
            println!("{}", checkpoint);
        }
        Err(e) => {
            print_detailed_error("Could not get checkpoint with full details", &e);
        }
    }

    // 4. Get Checkpoint by Hash
    println!("\n4. Get Checkpoint by Hash");
    println!("==========================");

    match client.get_checkpoint_by_hash(&checkpoint_hash, false).await {
        Ok(checkpoint) => {
            println!("{}", checkpoint);
        }
        Err(e) => {
            print_detailed_error(
                &format!("Checkpoint with hash {} not found", checkpoint_hash),
                &e,
            );
        }
    }

    // 5. Error Handling Examples
    println!("\n5. Error Handling Examples");
    println!("==========================");

    // Try to get a very old checkpoint (likely doesn't exist)
    let not_exsit_checkpoint = 9999999999;
    match client
        .get_checkpoint_by_number(not_exsit_checkpoint, false)
        .await
    {
        Ok(_) => {
            println!("Surprisingly, checkpoint {} exists!", not_exsit_checkpoint);
        }
        Err(e) => {
            print_detailed_error("Expected error for non-existent checkpoint", &e);
        }
    }

    // Try to get checkpoint with invalid hash
    let invalid_hash = "0xinvalidhash";
    match client.get_checkpoint_by_hash(invalid_hash, false).await {
        Ok(_) => {
            println!("Unexpected: invalid hash worked!");
        }
        Err(e) => {
            print_detailed_error("Expected error for invalid hash", &e);
        }
    }

    println!("\nCheckpoint examples completed successfully!");

    Ok(())
}
