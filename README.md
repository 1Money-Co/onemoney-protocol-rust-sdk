# OneMoney Rust SDK

Official Rust SDK for the OneMoney L1 blockchain REST API.

[![Crates.io](https://img.shields.io/crates/v/onemoney-protocol.svg)](https://crates.io/crates/onemoney-protocol)
[![Documentation](https://docs.rs/onemoney-protocol/badge.svg)](https://docs.rs/onemoney-protocol)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## Features

- **Async/await support** - Built with Tokio for high-performance async operations
- **Cryptographic utilities** - ECDSA signing, address derivation, and message verification
- **Type-safe API** - Strongly typed requests and responses with comprehensive error handling
- **Modular design** - Organized by functionality (accounts, tokens, transactions, etc.)
- **Network support** - Mainnet and testnet configurations
- **Extensible** - Hook system for middleware and custom logging
- **Well documented** - Comprehensive API documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
onemoney-protocol = "0.1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use onemoney_protocol::{Client, ClientBuilder, Network, OneMoneyAddress, TokenAmount};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create clients for different networks
    let mainnet_client = Client::mainnet();          // Mainnet
    let testnet_client = Client::testnet();          // Testnet
    let local_client = Client::local();              // Local development

    // Or use the builder pattern
    let client = ClientBuilder::new()
        .network(Network::Testnet)
        .build()?;

    // Get account nonce
    let address = OneMoneyAddress::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")?;
    let nonce = client.get_account_nonce(address).await?;
    println!("Account nonce: {}", nonce.nonce);

    // Get latest blockchain state
    let state = client.get_latest_epoch_checkpoint().await?;
    println!("Current epoch: {}, checkpoint: {}", state.epoch, state.checkpoint);

    Ok(())
}
```

## Core Functionality

### Client Configuration

```rust
use onemoney_protocol::{Client, ClientBuilder};
use std::time::Duration;

// Basic clients
let client = Client::mainnet();     // Mainnet
let client = Client::testnet();     // Testnet

// Custom configuration
let client = ClientBuilder::new()
    .base_url("https://custom.api.endpoint.com")
    .timeout(Duration::from_secs(30))
    .build()?;
```

### Account Operations

```rust
// Get account nonce
let nonce = client.get_account_nonce(address).await?;

// Get token account balance
let token_account = client.get_token_account(owner, mint_address).await?;
println!("Balance: {}", token_account.amount);

// List all token accounts for an address
let (accounts, total) = client.list_token_accounts(owner, Some(10), Some(0)).await?;

// Derive token account address
let token_account_addr = client.derive_token_account_address(wallet, mint);
```

### Token Operations

```rust
use onemoney_protocol::{TokenMintPayload, Authority};

// Mint tokens
let mint_payload = TokenMintPayload {
    recent_epoch: state.epoch,
    recent_checkpoint: state.checkpoint,
    chain_id: 1212101,
    nonce: 1,
    token: token_address,
    recipient: recipient_address,
    value: TokenAmount::from(1000000000000000000u64), // 1 token
};

let result = client.mint_token(mint_payload, private_key).await?;
```

### Transaction Operations

```rust
use onemoney_protocol::PaymentPayload;

// Send a payment
let payment = PaymentPayload {
    recent_epoch: state.epoch,
    recent_checkpoint: state.checkpoint,
    chain_id: 1212101,
    nonce: 2,
    recipient: recipient_address,
    value: TokenAmount::from(500000000000000000u64), // 0.5 tokens
    token: token_address,
};

let result = client.send_payment(payment, private_key).await?;
println!("Payment sent: {}", result.hash);

// Get transaction details
let tx = client.get_transaction_by_hash(&result.hash).await?;
println!("Transaction status: {:?}", tx.status);

// Wait for confirmation
let confirmed_tx = client.wait_for_transaction(
    &result.transaction_hash,
    30, // max attempts
    Duration::from_secs(2) // polling interval
).await?;
```

### Blockchain State

```rust
// Get latest state (for transaction construction)
let state = client.get_latest_epoch_checkpoint().await?;

// Get chain information
let chain = client.get_chain_info().await?;
println!("Chain ID: {}", chain.chain_id);

// Get network statistics
let stats = client.get_network_stats().await?;
println!("Total transactions: {}", stats.total_transactions);
```

### Cryptographic Utilities

```rust
use onemoney_protocol::crypto;

// Derive address from private key
let address = crypto::private_key_to_address(private_key)?;

// Sign a message
let signature = crypto::sign_message(&payload, private_key)?;

// Verify signature
let is_valid = crypto::verify_signature(&payload, &signature, signer_address)?;
```

## Error Handling

The SDK provides comprehensive error handling:

```rust
use onemoney_protocol::{Error, Result};

match client.get_account_nonce(address).await {
    Ok(nonce) => println!("Nonce: {}", nonce.nonce),
    Err(Error::Api { status_code: 404, .. }) => {
        println!("Account not found");
    },
    Err(Error::Http(e)) => {
        println!("Network error: {}", e);
    },
    Err(Error::Json(e)) => {
        println!("JSON parsing error: {}", e);
    },
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

## Examples

Run the included examples:

```bash
# Transaction management example
cargo run --example transactions_example

# Network configuration examples
cargo run --example network_examples

# Token operations example
cargo run --example tokens_example

# Account management example
cargo run --example accounts_example
```

## API Reference

The SDK is organized into several modules:

- **`accounts`** - Account queries, nonce management, token account operations
- **`tokens`** - Token minting, burning, authority management, pause/blacklist/whitelist controls
- **`transactions`** - Payment sending, transaction queries, fee estimation
- **`chains`** - Chain information and network details
- **`checkpoints`** - Checkpoint queries and blockchain state
- **`states`** - Latest state queries and network statistics
- **`crypto`** - Cryptographic utilities for signing and verification

## Configuration

### Environment Variables

- `ONEMONEY_API_URL` - Override the default API endpoint
- `ONEMONEY_TIMEOUT` - Set request timeout in seconds
- `ONEMONEY_LOG_LEVEL` - Set logging level (trace, debug, info, warn, error)

### Network Endpoints

- **Mainnet**: `https://api.1money.network`
- **Testnet**: `https://api.testnet.1money.network`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run the test suite: `cargo test`
6. Submit a pull request

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test accounts::tests
```

## License

This project is licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) for details or visit http://www.apache.org/licenses/LICENSE-2.0.

## Support

- 📖 [Documentation](https://docs.rs/onemoney-protocol)
- 🐛 [Issue Tracker](https://github.com/1Money-Co/onemoney-rust-sdk/issues)
- 💬 [Discussions](https://github.com/1Money-Co/onemoney-rust-sdk/discussions)

---

Built by the OneMoney team.
