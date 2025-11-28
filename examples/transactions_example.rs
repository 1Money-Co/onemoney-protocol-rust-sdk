//! Transaction management example for the OneMoney Rust SDK.
//!
//! This example demonstrates actual transaction-related operations:
//! - Getting transaction information by hash
//! - Getting transaction receipts
//! - Estimating transaction fees
//! - Creating payment transactions (demo)

#[path = "common.rs"]
mod common;
use common as environment;

use alloy_primitives::{Address, U256};
use environment::{
    ExampleConfig, create_example_client, print_detailed_error, print_environment_banner,
};
use onemoney_protocol::{FeeEstimateRequest, PaymentPayload};
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Transaction Management");
    let client = create_example_client();

    // Get example configuration
    let config = ExampleConfig::get();
    config.print_config_warning();

    let sender_address = Address::from_str(config.wallet_address)?;
    let recipient_address = Address::from_str(config.recipient_address)?;
    let private_key = config.private_key;
    let token_address = Address::from_str(config.token_mint_address)?;

    println!("\nDemo Configuration:");
    println!("   Sender: {}", sender_address);
    println!("   Recipient: {}", recipient_address);
    println!("   Token: {} ", token_address);

    // 1. Get Account Nonce
    println!("\n1. Get Account Nonce");
    println!("====================");

    let nonce = match client.get_account_nonce(sender_address).await {
        Ok(nonce_info) => {
            println!("{}", nonce_info);
            nonce_info.nonce
        }
        Err(e) => {
            print_detailed_error("Could not get account nonce", &e);
            println!("   Using nonce 0 (for new accounts)");
            0
        }
    };

    // 2. Estimate Transaction Fees and Check Balance
    println!("\n2. Estimate Transaction Fees and Check Balance");
    println!("=============================================");

    let amount = U256::from(10000u64); // 1 token

    let fee_request = FeeEstimateRequest {
        from: sender_address.to_string(),
        value: amount.to_string(),
        token: Some(token_address.to_string()),
    };

    let estimated_fee_str = match client.estimate_fee(fee_request).await {
        Ok(fee_estimate) => {
            println!("{}", fee_estimate);
            fee_estimate.fee.clone()
        }
        Err(e) => {
            print_detailed_error("Could not estimate fee", &e);
            println!("   Proceeding without fee check...");
            "0".to_string() // Fallback
        }
    };

    // Check token account balance
    println!("\nChecking sender's token balance...");
    match client
        .get_associated_token_account(sender_address, token_address)
        .await
    {
        Ok(token_account) => {
            println!("{}", token_account);

            let estimated_fee = U256::from_str(&estimated_fee_str).unwrap_or(U256::from(0u64));
            let total_required = amount + estimated_fee;
            let available_balance =
                U256::from_str(&token_account.balance).unwrap_or(U256::from(0u64));

            if available_balance < total_required {
                println!("  Status: WARNING - Insufficient balance");
            } else {
                println!("  Status: Sufficient balance confirmed");
            }
        }
        Err(e) => {
            print_detailed_error("Could not check token balance", &e);
            println!("   This may indicate the token account doesn't exist yet");
        }
    }

    // 3. Create and Send Payment Transaction
    println!("\n3. Create and Send Payment Transaction");
    println!("=====================================");

    // Get chain ID for transaction
    let chain_id = match client.fetch_chain_id_from_network().await {
        Ok(id) => id,
        Err(e) => {
            print_detailed_error("Could not get chain ID", &e);
            return Ok(());
        }
    };

    let payment_payload = PaymentPayload {
        chain_id,
        nonce,
        recipient: recipient_address,
        value: amount,
        token: token_address,
    };

    println!("Payment payload created:");
    println!("{}", payment_payload);

    println!("\nSending payment transaction to the network...");

    let payment_response = match client.send_payment(payment_payload, private_key).await {
        Ok(payment_response) => {
            println!("{}", payment_response);
            println!("Payment transaction sent successfully");
            payment_response
        }
        Err(e) => {
            print_detailed_error("Could not send payment transaction", &e);

            // Provide detailed diagnostic information based on error type
            let error_string = format!("{}", e);
            println!("\nDiagnostic Information:");

            if error_string.contains("insufficient funds") {
                println!("   INSUFFICIENT FUNDS:");
                println!("      - Your account doesn't have enough tokens for this transaction");
                println!("      - Required: {} tokens + gas fees", amount);
                println!("      - Solution: Add more funds to your account");
            } else if error_string.contains("invalid nonce") {
                println!("   NONCE MISMATCH:");
                println!("      - The nonce {} is incorrect for this account", nonce);
                println!("      - Solution: Use get_account_nonce() to get the correct nonce");
            } else if error_string.contains("validation") {
                println!("   VALIDATION ERROR:");
                println!("      - One of the transaction parameters is invalid");
                println!("      - Check: addresses, amounts, token address, chain ID");
            } else if error_string.contains("timeout") || error_string.contains("network") {
                println!("   NETWORK ISSUE:");
                println!("      - Connection to the blockchain network failed");
                println!("      - Solution: Check internet connection and try again");
            } else {
                println!("   UNKNOWN ERROR:");
                println!("      - This might be a temporary issue");
                println!("      - Solution: Try again in a few moments");
            }

            println!("\nCommon Solutions:");
            println!("   1. Verify your account has sufficient balance");
            println!("   2. Check that all addresses are valid and checksummed");
            println!("   3. Ensure you're connected to the correct network");
            println!("   4. Try refreshing the nonce by calling get_account_nonce()");
            println!("   5. Check the current network status and try again");

            return Ok(());
        }
    };

    let tx_hash = &payment_response.hash;

    // Smart transaction confirmation with SDK helper
    println!("\nWaiting for transaction confirmation via SDK helper...");
    match client
        .wait_for_transaction_receipt(&tx_hash.to_string())
        .await
    {
        Ok(receipt) => {
            println!("Transaction confirmed:");
            println!("{}", receipt);
        }
        Err(e) => {
            print_detailed_error("Timed out waiting for receipt (30s default)", &e);
        }
    }

    // Custom timeout example (useful for slower networks)
    println!("\nWaiting for transaction receipt with custom timeout (60s)...");
    match client
        .wait_for_transaction_receipt_with_timeout(
            &tx_hash.to_string(),
            tokio::time::Duration::from_secs(60),
        )
        .await
    {
        Ok(receipt) => {
            println!("Transaction confirmed with custom timeout:");
            println!("{}", receipt);
        }
        Err(e) => {
            print_detailed_error("Custom timeout receipt wait failed", &e);
        }
    }

    // Fetch finalized transaction
    println!("\nFetching finalized transaction...");
    match client
        .get_finalized_transaction_by_hash(&tx_hash.to_string())
        .await
    {
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

    // 4. Error Handling Examples
    println!("\n4. Error Handling Examples");
    println!("==========================");

    // Invalid transaction hash
    println!("Testing with invalid transaction hash:");
    match client.get_transaction_by_hash("invalid_hash").await {
        Ok(_) => println!("   Unexpected success with invalid hash"),
        Err(e) => {
            print_detailed_error("Correctly rejected invalid hash", &e);
        }
    }

    // 5. Transaction Summary and Best Practices
    println!("\n5. Transaction Summary and Best Practices");
    println!("========================================");

    println!("Transaction Lifecycle Summary:");
    println!("   1. Account nonce retrieval");
    println!("   2. Fee estimation and balance checking");
    println!("   3. Transaction payload creation");
    println!("   4. Cryptographic signing (ECDSA)");
    println!("   5. Network submission");
    println!("   6. Confirmation monitoring with retry logic");
    println!("   7. Receipt retrieval for execution details");

    println!("\nProduction Best Practices:");
    println!("   - Always check balance before sending transactions");
    println!("   - Use proper error handling with retry mechanisms");
    println!("   - Monitor transaction status until confirmed");
    println!("   - Keep private keys secure and never log them");
    println!("   - Use checksummed addresses to prevent typos");
    println!("   - Validate all inputs before transaction creation");
    println!("   - Implement timeout and retry logic for network calls");

    println!("\nAdvanced Features:");
    println!("   - Batch transaction processing");
    println!("   - Custom gas price optimization");
    println!("   - Transaction mempool monitoring");
    println!("   - Multi-signature support (coming soon)");

    println!("\nTransaction Management Example Completed!");
    println!("=========================================");
    println!("SDK Features (L1-Compatible):");
    println!("   - L1-compatible signature calculation using signature_hash()");
    println!("   - Manual epoch and checkpoint management with send_payment()");
    println!("   - Unified Signable trait for all transaction types");
    println!("   - Compatible with L1 client's build_txn approach");

    println!("\nAvailable Endpoints:");
    println!("   - GET /v1/transactions/by_hash - Get transaction by hash");
    println!("   - GET /v1/transactions/finalized/by_hash - Get finalized transaction by hash");
    println!("   - GET /v1/transactions/receipt/by_hash - Get transaction receipt");
    println!("   - POST /v1/transactions/payment - Send payment transaction");
    println!("   - GET /v1/transactions/estimate_fee - Estimate transaction fees");
    println!("\nNext steps:");
    println!("   - Use send_payment() for payment transactions");
    println!("   - See tokens_example.rs for token operations");
    println!("   - Review checkpoints_example.rs for checkpoint data");
    println!("   - Check accounts_example.rs for account operations");

    Ok(())
}
