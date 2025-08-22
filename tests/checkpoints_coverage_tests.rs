//! Comprehensive checkpoints coverage tests

use alloy_primitives::{Address, B256, U256};
use onemoney_protocol::types::responses::checkpoints::*;
use onemoney_protocol::types::responses::transactions::*;
use onemoney_protocol::*;
use std::str::FromStr;

/// Helper function to create Hash from hex string
fn create_hash(hex_str: &str) -> Hash {
    // Pad short hex strings to 32 bytes (64 hex chars + 0x prefix)
    let padded_hex = if hex_str.len() < 66 {
        let without_prefix = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        format!("0x{:0<64}", without_prefix)
    } else {
        hex_str.to_string()
    };

    Hash {
        hash: B256::from_str(&padded_hex).expect("Test data should be valid"),
    }
}

#[test]
fn test_checkpoint_transactions_full() {
    // Create a transaction for testing
    let transaction = Transaction {
        hash: B256::from_str("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
            .expect("Test data should be valid"),
        checkpoint_hash: Some(
            B256::from_str("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f")
                .expect("Test data should be valid"),
        ),
        checkpoint_number: Some(1500),
        transaction_index: Some(0),
        epoch: 100,
        checkpoint: 200,
        chain_id: 1212101,
        from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        nonce: 5,
        data: TxPayload::TokenTransfer {
            value: "1000000000000000000".to_string(),
            to: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            token: None,
        },
        signature: Signature {
            r: U256::from(12345u64),
            s: U256::from(67890u64),
            v: 27,
        },
    };

    let checkpoint_transactions = CheckpointTransactions::Full(vec![transaction.clone()]);

    // Test serialization
    let json = serde_json::to_string(&checkpoint_transactions).expect("Test data should be valid");
    let deserialized: CheckpointTransactions =
        serde_json::from_str(&json).expect("Test data should be valid");

    match deserialized {
        CheckpointTransactions::Full(transactions) => {
            assert_eq!(transactions.len(), 1);
            assert_eq!(transactions[0].hash, transaction.hash);
            assert_eq!(transactions[0].from, transaction.from);
        }
        _ => panic!("Should be Full variant"),
    }

    // Test display
    let display_str = format!("{}", checkpoint_transactions);
    assert!(display_str.contains("Checkpoint with 1 full transactions"));
}

#[test]
fn test_checkpoint_transactions_hashes() {
    let hashes = vec![
        create_hash("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"),
        create_hash("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"),
        create_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"),
    ];

    // Test individual Hash serialization is transparent (not an object)
    let single_hash_json =
        serde_json::to_string(&hashes[0]).expect("Hash serialization should work");
    assert!(single_hash_json.starts_with("\"0x"));
    assert!(single_hash_json.ends_with("\""));
    assert!(!single_hash_json.contains("{"));
    assert!(!single_hash_json.contains("hash"));

    let checkpoint_transactions = CheckpointTransactions::Hashes(hashes.clone());

    // Test serialization
    let json = serde_json::to_string(&checkpoint_transactions).expect("Test data should be valid");
    let deserialized: CheckpointTransactions =
        serde_json::from_str(&json).expect("Test data should be valid");

    match deserialized {
        CheckpointTransactions::Hashes(deserialized_hashes) => {
            assert_eq!(deserialized_hashes.len(), 3);
            assert_eq!(deserialized_hashes, hashes);
        }
        _ => panic!("Should be Hashes variant"),
    }

    // Test display
    let display_str = format!("{}", checkpoint_transactions);
    assert!(display_str.contains("Checkpoint with 3 transaction hashes"));
}

#[test]
fn test_checkpoint_full_structure() {
    let transaction = Transaction {
        hash: B256::from_str("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
            .expect("Test data should be valid"),
        checkpoint_hash: Some(
            B256::from_str("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f")
                .expect("Test data should be valid"),
        ),
        checkpoint_number: Some(1500),
        transaction_index: Some(0),
        epoch: 100,
        checkpoint: 200,
        chain_id: 1212101,
        from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        nonce: 5,
        data: TxPayload::TokenMint {
            value: "1000000000000000000".to_string(),
            address: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            token: Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                .expect("Test data should be valid"),
        },
        signature: Signature {
            r: U256::from(12345u64),
            s: U256::from(67890u64),
            v: 27,
        },
    };

    let checkpoint = Checkpoint {
        hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
        parent_hash: create_hash(
            "0x000abc456def789012345678901234567890123456789012345678901234567e",
        ),
        state_root: create_hash(
            "0xabc123456def789012345678901234567890123456789012345678901234567e",
        ),
        transactions_root: create_hash(
            "0xdef456789012345678901234567890123456789012345678901234567890abc1",
        ),
        receipts_root: create_hash(
            "0x789012345678901234567890123456789012345678901234567890abc123def4",
        ),
        number: 1500,
        timestamp: 1703097600, // 2023-12-20 16:00:00 UTC
        extra_data: "0x".to_string(),
        transactions: CheckpointTransactions::Full(vec![transaction]),
        size: Some(1024),
    };

    // Test serialization
    let json = serde_json::to_string(&checkpoint).expect("Test data should be valid");
    let deserialized: Checkpoint = serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(checkpoint.hash, deserialized.hash);
    assert_eq!(checkpoint.parent_hash, deserialized.parent_hash);
    assert_eq!(checkpoint.number, deserialized.number);
    assert_eq!(checkpoint.timestamp, deserialized.timestamp);
    assert_eq!(checkpoint.size, deserialized.size);

    // Test display with all fields
    let display_str = format!("{}", checkpoint);
    assert!(display_str.contains("Checkpoint #1500:"));
    assert!(display_str.contains(&checkpoint.hash.hash.to_string()));
    assert!(display_str.contains(&checkpoint.parent_hash.hash.to_string()));
    assert!(display_str.contains(&checkpoint.state_root.hash.to_string()));
    assert!(display_str.contains(&checkpoint.transactions_root.hash.to_string()));
    assert!(display_str.contains(&checkpoint.receipts_root.hash.to_string()));
    assert!(display_str.contains("1703097600"));
    assert!(display_str.contains("Size: 1024 bytes"));
    assert!(display_str.contains("full transaction details"));
    assert!(display_str.contains("Transaction 1:"));
    assert!(display_str.contains("Signature:"));
    assert!(display_str.contains("Payload:"));
}

#[test]
fn test_checkpoint_with_hashes_only() {
    let checkpoint = Checkpoint {
        hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
        parent_hash: create_hash(
            "0x000abc456def789012345678901234567890123456789012345678901234567e",
        ),
        state_root: create_hash(
            "0xabc123456def789012345678901234567890123456789012345678901234567e",
        ),
        transactions_root: create_hash(
            "0xdef456789012345678901234567890123456789012345678901234567890abc1",
        ),
        receipts_root: create_hash(
            "0x789012345678901234567890123456789012345678901234567890abc123def4",
        ),
        number: 1500,
        timestamp: 1703097600,
        extra_data: "checkpoint_data".to_string(),
        transactions: CheckpointTransactions::Hashes(vec![
            create_hash("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"),
            create_hash("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"),
        ]),
        size: None, // Test without size
    };

    // Test display with hashes
    let display_str = format!("{}", checkpoint);
    assert!(display_str.contains("Checkpoint #1500:"));
    assert!(display_str.contains("checkpoint_data"));
    assert!(!display_str.contains("Size:")); // Should not contain size when None
    assert!(display_str.contains("hashes only"));
    assert!(
        display_str
            .contains("1: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
    );
    assert!(
        display_str
            .contains("2: 0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f")
    );
}

#[test]
fn test_checkpoint_header_structure() {
    let header = CheckpointHeader {
        hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
        parent_hash: create_hash(
            "0x000abc456def789012345678901234567890123456789012345678901234567e",
        ),
        state_root: create_hash(
            "0xabc123456def789012345678901234567890123456789012345678901234567e",
        ),
        transactions_root: create_hash(
            "0xdef456789012345678901234567890123456789012345678901234567890abc1",
        ),
        receipts_root: create_hash(
            "0x789012345678901234567890123456789012345678901234567890abc123def4",
        ),
        number: 999,
        timestamp: 1703097600,
        extra_data: "0x1234".to_string(),
    };

    // Test serialization
    let json = serde_json::to_string(&header).expect("Test data should be valid");
    let deserialized: CheckpointHeader =
        serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(header.hash, deserialized.hash);
    assert_eq!(header.parent_hash, deserialized.parent_hash);
    assert_eq!(header.state_root, deserialized.state_root);
    assert_eq!(header.transactions_root, deserialized.transactions_root);
    assert_eq!(header.receipts_root, deserialized.receipts_root);
    assert_eq!(header.number, deserialized.number);
    assert_eq!(header.timestamp, deserialized.timestamp);
    assert_eq!(header.extra_data, deserialized.extra_data);

    // Test display
    let display_str = format!("{}", header);
    assert!(display_str.contains("Checkpoint #999:"));
    assert!(display_str.contains(&header.hash.hash.to_string()));
    assert!(display_str.contains(&header.parent_hash.hash.to_string()));
    assert!(display_str.contains("1703097600"));
}

#[test]
fn test_checkpoint_number_structure() {
    let checkpoint_num = CheckpointNumber { number: 12345 };

    // Test serialization
    let json = serde_json::to_string(&checkpoint_num).expect("Test data should be valid");
    let deserialized: CheckpointNumber =
        serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(checkpoint_num.number, deserialized.number);

    // Test display
    let display_str = format!("{}", checkpoint_num);
    assert_eq!(display_str, "Checkpoint Number: 12345");
}

#[test]
fn test_checkpoint_transactions_empty() {
    // Test empty transactions
    let empty_full = CheckpointTransactions::Full(vec![]);
    let display_str = format!("{}", empty_full);
    assert!(display_str.contains("Checkpoint with 0 full transactions"));

    let empty_hashes = CheckpointTransactions::Hashes(vec![]);
    let display_str = format!("{}", empty_hashes);
    assert!(display_str.contains("Checkpoint with 0 transaction hashes"));
}

#[test]
fn test_checkpoint_multiple_transactions() {
    let tx1 = Transaction {
        hash: B256::from_str("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
            .expect("Test data should be valid"),
        checkpoint_hash: None,   // Test without checkpoint hash
        checkpoint_number: None, // Test without checkpoint number
        transaction_index: None, // Test without transaction index
        epoch: 100,
        checkpoint: 200,
        chain_id: 1212101,
        from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
            .expect("Test data should be valid"),
        nonce: 5,
        data: TxPayload::TokenTransfer {
            value: "1000000000000000000".to_string(),
            to: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
            token: None,
        },
        signature: Signature::default(),
    };

    let tx2 = Transaction {
        hash: B256::from_str("0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f")
            .expect("Test data should be valid"),
        checkpoint_hash: Some(
            B256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .expect("Test data should be valid"),
        ),
        checkpoint_number: Some(1500),
        transaction_index: Some(1),
        epoch: 101,
        checkpoint: 201,
        chain_id: 1212101,
        from: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
            .expect("Test data should be valid"),
        nonce: 10,
        data: TxPayload::TokenBurn {
            value: "500000000000000000".to_string(),
            address: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            token: Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                .expect("Test data should be valid"),
        },
        signature: Signature {
            r: U256::from(54321u64),
            s: U256::from(98765u64),
            v: 28,
        },
    };

    let checkpoint_transactions = CheckpointTransactions::Full(vec![tx1.clone(), tx2.clone()]);

    // Test CheckpointTransactions display
    let display_str = format!("{}", checkpoint_transactions);
    assert!(display_str.contains("Checkpoint with 2 full transactions"));

    // Create a full checkpoint to test detailed transaction display
    let checkpoint = Checkpoint {
        hash: create_hash("0x123abc456def789012345678901234567890123456789012345678901234567e"),
        parent_hash: create_hash(
            "0x000abc456def789012345678901234567890123456789012345678901234567e",
        ),
        state_root: create_hash(
            "0xabc123456def789012345678901234567890123456789012345678901234567e",
        ),
        transactions_root: create_hash(
            "0xdef456789012345678901234567890123456789012345678901234567890abc1",
        ),
        receipts_root: create_hash(
            "0x789012345678901234567890123456789012345678901234567890abc123def4",
        ),
        number: 1500,
        timestamp: 1703097600,
        extra_data: "0x".to_string(),
        transactions: checkpoint_transactions,
        size: Some(2048),
    };

    // Test checkpoint display with multiple transactions
    let checkpoint_display_str = format!("{}", checkpoint);
    assert!(checkpoint_display_str.contains("Transaction 1:"));
    assert!(checkpoint_display_str.contains("Transaction 2:"));
    assert!(checkpoint_display_str.contains(&format!("{}", tx1.hash)));
    assert!(checkpoint_display_str.contains(&format!("{}", tx2.hash)));

    // Test that optional fields are handled correctly
    assert!(checkpoint_display_str.contains(&format!(
        "{}",
        tx2.checkpoint_hash.expect("Test data should be valid")
    )));
}

#[test]
fn test_serde_untagged_enum() {
    // Test that the untagged enum properly deserializes different JSON structures

    // JSON with array of hashes (should deserialize as Hashes)
    let hashes_json = r#"["0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777", "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"]"#;
    let parsed: CheckpointTransactions =
        serde_json::from_str(hashes_json).expect("Test data should be valid");

    match parsed {
        CheckpointTransactions::Hashes(hashes) => {
            assert_eq!(hashes.len(), 2);
        }
        _ => panic!("Should parse as Hashes variant"),
    }

    // Note: Testing full transaction parsing would require a complete transaction JSON structure
    // with all required fields. For now, we've tested the Hashes variant which works correctly.
    // The untagged enum will automatically choose the correct variant based on the JSON structure.
}
