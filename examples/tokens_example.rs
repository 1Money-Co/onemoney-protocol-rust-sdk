//! Token operations example for the OneMoney Rust SDK.
//!
//! This example demonstrates token-related operations:
//! - Getting token metadata
//! - Minting tokens
//! - Burning tokens
//! - Authority management
//! - Token pause/unpause controls
//! - Blacklist/whitelist management
//! - Metadata updates

#[path = "common.rs"]
mod common;
use common as environment;

use alloy_primitives::{Address, U256};
use environment::{
    ExampleConfig, create_example_client, print_detailed_error, print_environment_banner,
};
use onemoney_protocol::{
    Authority, AuthorityAction, BlacklistAction, MetadataKVPair, PauseAction,
    TokenAuthorityPayload, TokenBlacklistPayload, TokenBurnPayload, TokenMetadataUpdatePayload,
    TokenMintPayload, TokenPausePayload, TokenWhitelistPayload, WhitelistAction,
};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print environment info and create client
    print_environment_banner("Token Operations");
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
    println!("   Token: {}", token_address);

    // Get dynamic parameters from API
    println!("\n0. Fetching Dynamic Parameters");
    println!("==============================");

    let state = match client.get_latest_epoch_checkpoint().await {
        Ok(s) => {
            println!(
                "Latest state: Epoch {}, Checkpoint {}",
                s.epoch, s.checkpoint
            );
            s
        }
        Err(e) => {
            print_detailed_error("Could not get latest state", &e);
            return Ok(());
        }
    };
    sleep(Duration::from_secs(1)).await;

    let chain_id = match client.get_chain_id().await {
        Ok(id) => {
            println!("Chain ID: {}", id);
            id
        }
        Err(e) => {
            print_detailed_error("Could not get chain ID", &e);
            return Ok(());
        }
    };
    sleep(Duration::from_secs(1)).await;

    let mut current_nonce = match client.get_account_nonce(sender_address).await {
        Ok(nonce_info) => {
            println!("Account nonce: {}", nonce_info.nonce);
            nonce_info.nonce
        }
        Err(e) => {
            print_detailed_error("Could not get account nonce", &e);
            println!("   Using nonce 0 (for new accounts)");
            0
        }
    };
    sleep(Duration::from_secs(1)).await;

    // 1. Get token metadata
    println!("\n1. Get Token Metadata");
    println!("=====================");

    let token_info = match client.get_token_metadata(token_address).await {
        Ok(mint_info) => {
            println!("{}", mint_info);
            Some(mint_info)
        }
        Err(e) => {
            print_detailed_error("Could not get token metadata", &e);
            None
        }
    };
    sleep(Duration::from_secs(1)).await;

    // 2. Mint tokens
    println!("\n2. Mint Tokens");
    println!("==============");

    let mint_payload = TokenMintPayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        recipient: sender_address, // Mint to sender's own account
        value: U256::from(1000000000000000000u64), // 1 token
        token: token_address,
    };
    current_nonce += 1; // Increment for next transaction

    match client.mint_token(mint_payload, private_key).await {
        Ok(response) => {
            println!("Tokens minted - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not mint tokens", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 3. Burn tokens
    println!("\n3. Burn Tokens");
    println!("==============");

    let burn_payload = TokenBurnPayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        recipient: sender_address, // Burn from sender's own account
        value: U256::from(500000000000000000u64), // 0.5 tokens
        token: token_address,
    };
    current_nonce += 1; // Increment for next transaction

    match client.burn_token(burn_payload, private_key).await {
        Ok(response) => {
            println!("Tokens burned - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not burn tokens", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 4. Grant authority
    println!("\n4. Grant Authority");
    println!("==================");

    let grant_payload = TokenAuthorityPayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        action: AuthorityAction::Grant,
        authority_type: Authority::MintBurnTokens,
        authority_address: recipient_address,
        token: token_address,
        value: U256::from(1000000000000000000u64), // 1 token allowance
    };
    current_nonce += 1; // Increment for next transaction

    match client.grant_authority(grant_payload, private_key).await {
        Ok(response) => {
            println!("Authority granted - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not grant authority", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 5. Revoke authority
    println!("\n5. Revoke Authority");
    println!("===================");

    let revoke_payload = TokenAuthorityPayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        action: AuthorityAction::Revoke,
        authority_type: Authority::MintBurnTokens,
        authority_address: recipient_address,
        token: token_address,
        value: U256::from(1000000000000000000u64), // 1 token allowance (same as granted)
    };
    current_nonce += 1; // Increment for next transaction

    match client.revoke_authority(revoke_payload, private_key).await {
        Ok(response) => {
            println!("Authority revoked - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not revoke authority", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 6. Pause token
    println!("\n6. Pause Token");
    println!("==============");

    let pause_payload = TokenPausePayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        action: PauseAction::Pause,
        token: token_address,
    };
    current_nonce += 1; // Increment for next transaction

    match client.pause_token(pause_payload, private_key).await {
        Ok(response) => {
            println!("Token paused - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not pause token", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 7. Unpause token
    println!("\n7. Unpause Token");
    println!("================");

    let unpause_payload = TokenPausePayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        action: PauseAction::Unpause,
        token: token_address,
    };
    current_nonce += 1; // Increment for next transaction

    match client.pause_token(unpause_payload, private_key).await {
        Ok(response) => {
            println!("Token unpaused - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not unpause token", &e);
        }
    }
    sleep(Duration::from_secs(1)).await;

    // 8. Manage blacklist (add address) - only for public tokens
    println!("\n8. Manage Blacklist");
    println!("===================");

    if let Some(ref info) = token_info {
        if !info.is_private {
            println!("Token is public - proceeding with blacklist operation");
            let blacklist_payload = TokenBlacklistPayload {
                recent_epoch: state.epoch,
                recent_checkpoint: state.checkpoint,
                chain_id,
                nonce: current_nonce,
                action: BlacklistAction::Add,
                address: recipient_address,
                token: token_address,
            };
            current_nonce += 1; // Increment for next transaction

            match client
                .manage_blacklist(blacklist_payload, private_key)
                .await
            {
                Ok(response) => {
                    println!("Address blacklisted - Tx: {}", response.hash);
                }
                Err(e) => {
                    print_detailed_error("Could not manage blacklist", &e);
                }
            }
        } else {
            println!("Token is private - skipping blacklist operation (not applicable)");
        }
    } else {
        println!("Token metadata not available - skipping blacklist operation");
    }
    sleep(Duration::from_secs(1)).await;

    // 8.1. Remove address from blacklist - only for public tokens
    println!("\n8.1. Remove Address from Blacklist");
    println!("===================================");

    if let Some(ref info) = token_info {
        if !info.is_private {
            println!("Token is public - proceeding with blacklist removal operation");
            let remove_blacklist_payload = TokenBlacklistPayload {
                recent_epoch: state.epoch,
                recent_checkpoint: state.checkpoint,
                chain_id,
                nonce: current_nonce,
                action: BlacklistAction::Remove,
                address: recipient_address,
                token: token_address,
            };
            current_nonce += 1; // Increment for next transaction

            match client
                .manage_blacklist(remove_blacklist_payload, private_key)
                .await
            {
                Ok(response) => {
                    println!("Address removed from blacklist - Tx: {}", response.hash);
                }
                Err(e) => {
                    print_detailed_error("Could not remove address from blacklist", &e);
                }
            }
        } else {
            println!("Token is private - skipping blacklist removal operation (not applicable)");
        }
    } else {
        println!("Token metadata not available - skipping blacklist removal operation");
    }
    sleep(Duration::from_secs(1)).await;

    // 9. Manage whitelist (add address) - only for private tokens
    println!("\n9. Manage Whitelist");
    println!("===================");

    if let Some(ref info) = token_info {
        if info.is_private {
            println!("Token is private - proceeding with whitelist operation");
            let whitelist_payload = TokenWhitelistPayload {
                recent_epoch: state.epoch,
                recent_checkpoint: state.checkpoint,
                chain_id,
                nonce: current_nonce,
                action: WhitelistAction::Add,
                address: recipient_address,
                token: token_address,
            };
            current_nonce += 1; // Increment for next transaction

            match client
                .manage_whitelist(whitelist_payload, private_key)
                .await
            {
                Ok(response) => {
                    println!("Address whitelisted - Tx: {}", response.hash);
                }
                Err(e) => {
                    print_detailed_error("Could not manage whitelist", &e);
                }
            }
        } else {
            println!("Token is public - skipping whitelist operation (not applicable)");
        }
    } else {
        println!("Token metadata not available - skipping whitelist operation");
    }
    sleep(Duration::from_secs(1)).await;

    // 10. Update token metadata
    println!("\n10. Update Token Metadata");
    println!("=========================");

    let metadata_payload = TokenMetadataUpdatePayload {
        recent_epoch: state.epoch,
        recent_checkpoint: state.checkpoint,
        chain_id,
        nonce: current_nonce,
        name: "Updated Test Token".to_string(),
        uri: "https://example.com/updated-metadata.json".to_string(),
        additional_metadata: vec![MetadataKVPair {
            key: "version".to_string(),
            value: "2.0".to_string(),
        }],
        token: token_address,
    };

    match client
        .update_token_metadata(metadata_payload, private_key)
        .await
    {
        Ok(response) => {
            println!("Metadata updated - Tx: {}", response.hash);
        }
        Err(e) => {
            print_detailed_error("Could not update token metadata", &e);
        }
    }
    sleep(Duration::from_secs(2)).await;

    println!("\nToken Operations Example Completed!");
    println!("===================================");
    println!("SDK Features (L1-Compatible):");
    println!("   - Token metadata queries with get_token_metadata()");
    println!("   - Token minting operations with proper authority checks");
    println!("   - Token burning with balance validation");
    println!("   - Authority management (grant/revoke permissions)");
    println!("   - Token pause/unpause controls");
    println!("   - Blacklist management (public tokens only)");
    println!("   - Whitelist management (private tokens only)");
    println!("   - Token metadata updates");

    println!("\nAvailable Token Endpoints:");
    println!("   - GET /v1/tokens/token_metadata - Get token metadata");
    println!("   - POST /v1/tokens/mint - Mint tokens");
    println!("   - POST /v1/tokens/burn - Burn tokens");
    println!("   - POST /v1/tokens/grant_authority - Grant/revoke token authority");
    println!("   - POST /v1/tokens/pause - Pause/unpause token");
    println!("   - POST /v1/tokens/manage_blacklist - Manage token blacklist");
    println!("   - POST /v1/tokens/manage_whitelist - Manage token whitelist");
    println!("   - POST /v1/tokens/update_metadata - Update token metadata");

    // 11. Query Latest Checkpoint with Full Transaction Details
    println!("\n11. Query Latest Checkpoint with Full Transaction Details");
    println!("========================================================");

    // Get the latest checkpoint number
    let latest_checkpoint_number = match client.get_checkpoint_number().await {
        Ok(checkpoint_number) => {
            println!("Latest {}", checkpoint_number);
            checkpoint_number.number
        }
        Err(e) => {
            print_detailed_error("Could not get latest checkpoint number", &e);
            return Ok(());
        }
    };

    // Get the checkpoint with full transaction details
    match client
        .get_checkpoint_by_number(latest_checkpoint_number, true)
        .await
    {
        Ok(checkpoint) => {
            println!("\nLatest Checkpoint with Full Transaction Details:");
            println!("===============================================");
            println!("{}", checkpoint);

            // Analyze token operations in this checkpoint
            match &checkpoint.transactions {
                onemoney_protocol::CheckpointTransactions::Full(transactions) => {
                    let mut token_operations = 0;
                    let mut mint_operations = 0;
                    let mut burn_operations = 0;
                    let mut transfer_operations = 0;
                    let mut authority_operations = 0;
                    let mut pause_operations = 0;
                    let mut blacklist_operations = 0;
                    let mut whitelist_operations = 0;
                    let mut metadata_operations = 0;
                    let mut create_operations = 0;

                    for tx in transactions {
                        match &tx.data {
                            onemoney_protocol::TxPayload::TokenCreate { .. } => {
                                token_operations += 1;
                                create_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenTransfer { .. } => {
                                token_operations += 1;
                                transfer_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenMint { .. } => {
                                token_operations += 1;
                                mint_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenBurn { .. } => {
                                token_operations += 1;
                                burn_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenGrantAuthority { .. } => {
                                token_operations += 1;
                                authority_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenRevokeAuthority { .. } => {
                                token_operations += 1;
                                authority_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenPause { .. } => {
                                token_operations += 1;
                                pause_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenUnpause { .. } => {
                                token_operations += 1;
                                pause_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenBlacklistAccount { .. } => {
                                token_operations += 1;
                                blacklist_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenWhitelistAccount { .. } => {
                                token_operations += 1;
                                whitelist_operations += 1;
                            }
                            onemoney_protocol::TxPayload::TokenUpdateMetadata { .. } => {
                                token_operations += 1;
                                metadata_operations += 1;
                            }
                            _ => {
                                // Other transaction types (governance, raw, etc.)
                            }
                        }
                    }

                    println!("\nToken Operations Analysis:");
                    println!("=========================");
                    println!("Total transactions in checkpoint: {}", transactions.len());
                    println!("Total token operations: {}", token_operations);
                    if create_operations > 0 {
                        println!("  - Token creation operations: {}", create_operations);
                    }
                    if transfer_operations > 0 {
                        println!("  - Token transfer operations: {}", transfer_operations);
                    }
                    if mint_operations > 0 {
                        println!("  - Token mint operations: {}", mint_operations);
                    }
                    if burn_operations > 0 {
                        println!("  - Token burn operations: {}", burn_operations);
                    }
                    if authority_operations > 0 {
                        println!("  - Token authority operations: {}", authority_operations);
                    }
                    if pause_operations > 0 {
                        println!("  - Token pause/unpause operations: {}", pause_operations);
                    }
                    if blacklist_operations > 0 {
                        println!("  - Token blacklist operations: {}", blacklist_operations);
                    }
                    if whitelist_operations > 0 {
                        println!("  - Token whitelist operations: {}", whitelist_operations);
                    }
                    if metadata_operations > 0 {
                        println!("  - Token metadata operations: {}", metadata_operations);
                    }

                    if token_operations == 0 {
                        println!("  No token operations found in this checkpoint.");
                        println!(
                            "  This is normal - checkpoints may contain other transaction types."
                        );
                    }
                }
                onemoney_protocol::CheckpointTransactions::Hashes(_) => {
                    println!(
                        "  Checkpoint contains only transaction hashes (should not happen with full=true)"
                    );
                }
            }
        }
        Err(e) => {
            print_detailed_error("Could not get checkpoint with full details", &e);
        }
    }

    println!("\nNext steps:");
    println!("   - Review transactions_example.rs for payment operations");
    println!("   - Check accounts_example.rs for account balance queries");
    println!("   - See checkpoints_example.rs for blockchain state info");
    println!("   - Examine the checkpoint data structure above to verify token operation display");

    Ok(())
}
