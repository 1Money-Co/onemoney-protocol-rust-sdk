//! Additional tests to improve code coverage

use alloy_primitives::{Address, B256, U256};
use onemoney_protocol::types::responses::tokens::MinterAllowance;
use onemoney_protocol::*;
use std::str::FromStr;

#[test]
fn test_client_config_api_paths() {
    use onemoney_protocol::client::config::*;

    // Test API path construction
    let path = api_path("/test");
    assert!(path.contains("/v1/test"));

    // Test endpoints constants
    assert_eq!(endpoints::accounts::NONCE, "/accounts/nonce");
    assert_eq!(
        endpoints::accounts::TOKEN_ACCOUNT,
        "/accounts/token_account"
    );
    assert_eq!(endpoints::chains::CHAIN_ID, "/chains/chain_id");
    assert_eq!(
        endpoints::states::LATEST_EPOCH_CHECKPOINT,
        "/states/latest_epoch_checkpoint"
    );
    assert_eq!(endpoints::checkpoints::NUMBER, "/checkpoints/number");
    assert_eq!(endpoints::checkpoints::BY_NUMBER, "/checkpoints/by_number");
    assert_eq!(endpoints::checkpoints::BY_HASH, "/checkpoints/by_hash");
    assert_eq!(endpoints::transactions::PAYMENT, "/transactions/payment");
    assert_eq!(endpoints::transactions::BY_HASH, "/transactions/by_hash");
    assert_eq!(
        endpoints::transactions::RECEIPT_BY_HASH,
        "/transactions/receipt/by_hash"
    );
    assert_eq!(
        endpoints::transactions::ESTIMATE_FEE,
        "/transactions/estimate_fee"
    );
    assert_eq!(endpoints::tokens::TOKEN_METADATA, "/tokens/token_metadata");
    assert_eq!(endpoints::tokens::MINT, "/tokens/mint");
    assert_eq!(endpoints::tokens::BURN, "/tokens/burn");
    assert_eq!(
        endpoints::tokens::GRANT_AUTHORITY,
        "/tokens/grant_authority"
    );
    assert_eq!(endpoints::tokens::PAUSE, "/tokens/pause");
    assert_eq!(
        endpoints::tokens::MANAGE_BLACKLIST,
        "/tokens/manage_blacklist"
    );
    assert_eq!(
        endpoints::tokens::MANAGE_WHITELIST,
        "/tokens/manage_whitelist"
    );
    assert_eq!(
        endpoints::tokens::UPDATE_METADATA,
        "/tokens/update_metadata"
    );
}

#[test]
fn test_request_structures_serialization() {
    let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");

    // Test PaymentPayload
    let payment_payload = PaymentPayload {
        recent_epoch: 100,
        recent_checkpoint: 200,
        chain_id: 1212101,
        nonce: 5,
        recipient: address,
        value: U256::from(1000000000000000000u64),
        token,
    };

    let json = serde_json::to_string(&payment_payload).expect("Test data should be valid");
    let deserialized: PaymentPayload =
        serde_json::from_str(&json).expect("Test data should be valid");
    assert_eq!(payment_payload.recent_epoch, deserialized.recent_epoch);
    assert_eq!(payment_payload.recipient, deserialized.recipient);

    // Test signature hash calculation
    let hash = payment_payload.signature_hash();
    assert_ne!(hash, B256::default());

    // Test RLP encoding
    let encoded = rlp::encode(&payment_payload);
    assert!(!encoded.is_empty());

    // Test display
    let display_str = format!("{}", payment_payload);
    assert!(display_str.contains("Payment to"));
    assert!(display_str.contains(&format!("{}", address)));
}

#[test]
fn test_response_structures_display() {
    let address = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");

    // Test LatestStateResponse
    let state = LatestStateResponse {
        epoch: 100,
        checkpoint: 200,
    };
    let display_str = format!("{}", state);
    assert!(display_str.contains("Latest State"));
    assert!(display_str.contains("epoch=100"));
    assert!(display_str.contains("checkpoint=200"));

    // Test ChainIdResponse
    let chain_id = ChainIdResponse { chain_id: 1212101 };
    let display_str = format!("{}", chain_id);
    assert!(display_str.contains("Chain ID: 1212101"));

    // Test MintInfo
    let mint_info = MintInfo {
        symbol: "TEST".to_string(),
        master_authority: address,
        master_mint_burn_authority: address,
        mint_burn_authorities: vec![MinterAllowance {
            minter: address,
            allowance: "1000000000000000000000".to_string(),
        }],
        pause_authorities: vec![address],
        list_authorities: vec![address],
        black_list: vec![],
        white_list: vec![],
        metadata_update_authorities: vec![address],
        supply: "1000000000000000000000".to_string(),
        decimals: 18,
        is_paused: false,
        is_private: false,
        meta: None,
    };
    let display_str = format!("{}", mint_info);
    assert!(display_str.contains("TEST"));
    assert!(display_str.contains("Decimals: 18"));
}

#[test]
fn test_error_handling() {
    // Test custom error
    let custom_error = Error::custom("Test error message".to_string());
    assert!(format!("{}", custom_error).contains("Test error message"));

    // Test address error
    let address_error = Error::address("invalid address");
    assert!(format!("{}", address_error).contains("invalid address"));

    // Test validation error
    let validation_error = Error::validation("field", "message");
    assert!(format!("{}", validation_error).contains("field"));
}

#[test]
fn test_signature_structure() {
    let signature = Signature {
        r: U256::from(12345u64),
        s: U256::from(67890u64),
        v: 27,
    };

    // Test serialization
    let json = serde_json::to_string(&signature).expect("Test data should be valid");
    let deserialized: Signature = serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(signature.r, deserialized.r);
    assert_eq!(signature.s, deserialized.s);
    assert_eq!(signature.v, deserialized.v);

    // Test default
    let default_sig = Signature::default();
    assert_eq!(default_sig.r, U256::ZERO);
    assert_eq!(default_sig.s, U256::ZERO);
    assert_eq!(default_sig.v, 0);
}

#[test]
fn test_token_metadata_structure() {
    let metadata = TokenMetadata {
        name: "Test Token".to_string(),
        uri: "https://example.com/token.json".to_string(),
        additional_metadata: vec![
            MetadataKVPair {
                key: "version".to_string(),
                value: "1.0".to_string(),
            },
            MetadataKVPair {
                key: "category".to_string(),
                value: "utility".to_string(),
            },
        ],
    };

    // Test serialization
    let json = serde_json::to_string(&metadata).expect("Test data should be valid");
    let deserialized: TokenMetadata =
        serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(metadata.name, deserialized.name);
    assert_eq!(metadata.uri, deserialized.uri);
    assert_eq!(
        metadata.additional_metadata.len(),
        deserialized.additional_metadata.len()
    );

    // Test display
    let display_str = format!("{}", metadata);
    assert!(display_str.contains("Test Token"));
    assert!(display_str.contains("https://example.com/token.json"));
}

#[test]
fn test_checkpoint_transactions_display() {
    // Test hashes display
    let hashes = CheckpointTransactions::Hashes(vec![
        "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777".to_string(),
        "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f".to_string(),
    ]);

    let display_str = format!("{}", hashes);
    assert!(display_str.contains("Checkpoint with 2 transaction hashes"));

    // Test full transactions display
    let transactions = CheckpointTransactions::Full(vec![]);
    let display_str = format!("{}", transactions);
    assert!(display_str.contains("Checkpoint with 0 full transactions"));
}

#[test]
fn test_transport_retry_logic() {
    use onemoney_protocol::transport::retry::*;
    use std::time::Duration;

    // Test retry config
    let config = RetryConfig::new()
        .max_attempts(5)
        .initial_delay(Duration::from_millis(1000))
        .max_delay(Duration::from_millis(30000));

    assert_eq!(config.max_attempts, 5);
    assert_eq!(config.initial_delay, Duration::from_millis(1000));
    assert_eq!(config.max_delay, Duration::from_millis(30000));

    // Test default config
    let default_config = RetryConfig::default();
    assert_eq!(default_config.max_attempts, 3);
    assert_eq!(default_config.initial_delay, Duration::from_millis(100));
    assert_eq!(default_config.max_delay, Duration::from_secs(60));

    // Test delay calculation
    let delay1 = config.delay_for_attempt(1);
    let delay2 = config.delay_for_attempt(2);
    let delay3 = config.delay_for_attempt(3);

    assert!(delay1 >= Duration::from_millis(1000));
    assert!(delay2 >= delay1);
    assert!(delay3 >= delay2);
    assert!(delay3 <= Duration::from_millis(30000));

    // Test status codes
    assert!(is_retryable_status(500));
    assert!(is_retryable_status(502));
    assert!(is_retryable_status(503));
    assert!(is_retryable_status(429));
    assert!(!is_retryable_status(400));
    assert!(!is_retryable_status(401));
    assert!(!is_retryable_status(404));

    // Test should retry
    assert!(config.should_retry(1));
    assert!(config.should_retry(2));
    assert!(!config.should_retry(5)); // Max attempts reached
    assert!(config.should_retry(0));
}

#[test]
fn test_crypto_functions() {
    use onemoney_protocol::crypto::*;

    // Test private key to address conversion
    let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let address_result = private_key_to_address(private_key).expect("Test data should be valid");
    assert_ne!(address_result, String::default());
    assert!(address_result.starts_with("0x"));
    assert_eq!(address_result.len(), 42); // 0x + 40 hex chars

    // Test derive token account address
    let owner = Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
        .expect("Test data should be valid");
    let token = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Test data should be valid");
    let token_account = derive_token_account_address(owner, token);
    assert_ne!(token_account, Address::default());
    assert_ne!(token_account, owner);
    assert_ne!(token_account, token);

    // Test deterministic behavior
    let token_account2 = derive_token_account_address(owner, token);
    assert_eq!(token_account, token_account2);
}

#[test]
fn test_utils_functions() {
    use onemoney_protocol::utils::*;

    // Test address validation
    assert!(address::is_valid_address_format(
        "0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"
    ));
    assert!(address::is_valid_address_format(
        "0x1234567890abcdef1234567890abcdef12345678"
    ));
    assert!(!address::is_valid_address_format("invalid"));
    assert!(!address::is_valid_address_format("0x123")); // Too short
    // Note: addresses without 0x prefix are actually valid - the function strips 0x and validates hex
    assert!(address::is_valid_address_format(
        "742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0"
    )); // Valid without 0x

    // Test wallet generation
    let wallet = wallet::EvmWallet::generate_random().expect("Test data should be valid");
    assert!(!wallet.private_key.is_empty());
    assert!(wallet.private_key.starts_with("0x"));
    assert_ne!(wallet.address, Address::default());

    // Test private key format
    assert!(wallet.private_key.len() == 66); // 0x + 64 hex chars
}
