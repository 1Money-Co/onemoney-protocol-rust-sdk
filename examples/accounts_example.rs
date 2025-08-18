//! Account management example for the OneMoney Rust SDK.
//!
//! This example demonstrates actual account-related operations:
//! - Getting account nonces
//! - Managing token accounts
//! - Handling errors gracefully

#[path = "common.rs"]
mod common;
use common as environment;

use environment::{
    create_example_client, get_example_environment, print_detailed_error, print_environment_banner,
    ExampleConfig,
};
use onemoney_protocol::OneMoneyAddress;
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create main client
    print_environment_banner("Account Management");
    let client = create_example_client();

    // Get example configuration
    let config = ExampleConfig::get();
    config.print_config_warning();

    let wallet_address = OneMoneyAddress::from_str(config.wallet_address)?;
    let token_mint_address = OneMoneyAddress::from_str(config.token_mint_address)?;

    let current_env = get_example_environment();
    println!(
        "Current environment: {} - using for all operations",
        current_env.name()
    );

    // 1. Account Nonce Operations
    println!("\n1. Account Nonce Operations");
    println!("============================");

    match client.get_account_nonce(wallet_address).await {
        Ok(nonce_info) => {
            println!("{}", nonce_info);
        }
        Err(e) => {
            print_detailed_error("Could not get account nonce", &e);
            println!(
                "   This usually means the account doesn't exist yet or network is unreachable"
            );
        }
    }

    // 2. Token Account Information
    println!("\n2. Token Account Information");
    println!("============================");

    match client
        .get_token_account(wallet_address, token_mint_address)
        .await
    {
        Ok(token_account) => {
            println!("{}", token_account);
        }
        Err(e) => {
            print_detailed_error("Could not get token account", &e);
            println!("   This usually means the token account doesn't exist for this wallet");
        }
    }

    // 3. Error Handling Examples
    println!("\n3. Error Handling Examples");
    println!("===========================");

    // Invalid address format
    println!("Testing invalid address format:");
    match OneMoneyAddress::from_str("invalid_address") {
        Ok(_) => println!("   Unexpectedly parsed invalid address"),
        Err(e) => println!("   Correctly rejected invalid address: {}", e),
    }

    // Non-existent account (should handle gracefully)
    let fake_address = OneMoneyAddress::from_str("0x0000000000000000000000000000000000000000")?;
    println!("Testing non-existent account:");
    match client.get_account_nonce(fake_address).await {
        Ok(nonce) => println!("   Zero address: {}", nonce),
        Err(e) => {
            print_detailed_error("Non-existent account error", &e);
        }
    }

    // Test invalid token mint
    println!("Testing invalid token mint:");
    let invalid_mint = OneMoneyAddress::from_str("0x1111111111111111111111111111111111111111")?;
    match client.get_token_account(wallet_address, invalid_mint).await {
        Ok(account) => println!("   Unexpected: {}", account),
        Err(e) => {
            print_detailed_error("Invalid token mint error", &e);
        }
    }

    Ok(())
}
