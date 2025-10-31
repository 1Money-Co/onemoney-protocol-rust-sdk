#[path = "common.rs"]
mod common;
use common as environment;

use environment::{create_example_client, print_environment_banner};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Transaction Management");
    let client = create_example_client();

    // Fetch finalized transaction
    let tx_hash = "0x7c4738387aad15db50d2219db9fc889a010541c36d0472b3f20e5244a8822b1e";
    match client.get_finalized_transaction_by_hash(tx_hash).await {
        Ok(finalized_tx) => {
            println!("Finalized transaction:");
            println!("{:?}", finalized_tx);
            println!("   Transaction has been finalized on chain");
        }
        Err(e) => {
            println!("   Finalized transaction not available: {}", e);
            println!("   The transaction may not be finalized yet");
            println!("   Finalization typically requires multiple epoch confirmations");
        }
    }

    Ok(())
}
