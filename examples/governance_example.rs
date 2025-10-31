//! Governance epoch example for the OneMoney Rust SDK.
//!
//! Demonstrates how to:
//! - Fetch the latest governance epoch
//! - Retrieve a specific epoch by ID
//! - Inspect certificate payload formats (JSON vs BCS)

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{
    create_example_client, get_example_environment, print_detailed_error, print_environment_banner,
};
use onemoney_protocol::EpochResponse;
use std::error::Error;

fn print_epoch_details(label: &str, epoch: &EpochResponse) -> Result<(), Box<dyn Error>> {
    use serde_json::to_string_pretty;

    println!("{label}:");
    println!("  Epoch ID: {}", epoch.epoch_id);
    println!("  Certificate Hash: {}", epoch.certificate_hash);

    if let Some(json_certificate) = epoch.certificate_json() {
        println!("  Certificate Format: JSON");
        let formatted = to_string_pretty(json_certificate)?;
        for line in formatted.lines() {
            println!("    {}", line);
        }
    } else if let Some(_certificate_hex) = epoch.certificate_bcs_hex() {
        println!("  Certificate Format: BCS");
        println!("    [certificate omitted: sensitive data]");
    } else {
        println!("  Certificate Format: Unknown");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_environment_banner("Governance Epoch Operations");
    let client = create_example_client();

    let env = get_example_environment();
    println!(
        "Current environment: {} ({})",
        env.name(),
        env.description()
    );

    println!("\n1. Fetching the latest governance epoch");
    println!("======================================");
    let current_epoch = match client.get_current_epoch().await {
        Ok(epoch) => {
            print_epoch_details("Latest Epoch", &epoch)?;
            Some(epoch)
        }
        Err(e) => {
            print_detailed_error("Could not fetch current epoch", &e);
            None
        }
    };

    if let Some(epoch) = current_epoch {
        let previous_epoch_id = epoch.epoch_id.saturating_sub(1);

        if previous_epoch_id != epoch.epoch_id {
            println!("\n2. Fetching previous epoch by ID");
            println!("================================");
            match client.get_epoch_by_id(previous_epoch_id).await {
                Ok(previous_epoch) => {
                    print_epoch_details("Previous Epoch", &previous_epoch)?;
                }
                Err(e) => {
                    print_detailed_error(
                        &format!("Could not fetch epoch {}", previous_epoch_id),
                        &e,
                    );
                }
            }
        } else {
            println!("\nPrevious epoch lookup skipped (current epoch is genesis).");
        }

        println!("\n3. Handling missing epoch IDs gracefully");
        println!("========================================");
        let invalid_epoch_id = epoch.epoch_id + 10_000;
        match client.get_epoch_by_id(invalid_epoch_id).await {
            Ok(found) => {
                println!(
                    "Unexpectedly found epoch {} when testing error handling",
                    found.epoch_id
                );
            }
            Err(e) => {
                println!(
                    "Correctly handled missing epoch {}: {}",
                    invalid_epoch_id, e
                );
            }
        }
    }

    Ok(())
}
