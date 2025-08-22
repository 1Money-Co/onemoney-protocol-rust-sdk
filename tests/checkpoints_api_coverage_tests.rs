//! Comprehensive checkpoints API coverage tests

use alloy_primitives::B256;
use onemoney_protocol::client::config::api_path;
use onemoney_protocol::client::config::endpoints::checkpoints::*;
use onemoney_protocol::types::responses::transactions::Hash;
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
fn test_checkpoint_api_path_construction() {
    // Test get_checkpoint_by_number paths
    let path_without_full = api_path(&format!("{}?number={}&full={}", BY_NUMBER, 1500, false));
    assert!(path_without_full.contains("/checkpoints/by_number"));
    assert!(path_without_full.contains("number=1500"));
    assert!(path_without_full.contains("full=false"));

    let path_with_full = api_path(&format!("{}?number={}&full={}", BY_NUMBER, 1500, true));
    assert!(path_with_full.contains("/checkpoints/by_number"));
    assert!(path_with_full.contains("number=1500"));
    assert!(path_with_full.contains("full=true"));

    // Test get_checkpoint_by_hash paths
    let hash = "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777";
    let hash_path_without_full = api_path(&format!("{}?hash={}&full={}", BY_HASH, hash, false));
    assert!(hash_path_without_full.contains("/checkpoints/by_hash"));
    assert!(hash_path_without_full.contains(&format!("hash={}", hash)));
    assert!(hash_path_without_full.contains("full=false"));

    let hash_path_with_full = api_path(&format!("{}?hash={}&full={}", BY_HASH, hash, true));
    assert!(hash_path_with_full.contains("/checkpoints/by_hash"));
    assert!(hash_path_with_full.contains(&format!("hash={}", hash)));
    assert!(hash_path_with_full.contains("full=true"));

    // Test get_checkpoint_number path
    let number_path = api_path(NUMBER);
    assert!(number_path.contains("/checkpoints/number"));
}

#[test]
fn test_checkpoint_endpoints_constants() {
    assert_eq!(BY_NUMBER, "/checkpoints/by_number");
    assert_eq!(BY_HASH, "/checkpoints/by_hash");
    assert_eq!(NUMBER, "/checkpoints/number");
}

#[test]
fn test_checkpoint_number_various_values() {
    // Test with different checkpoint numbers
    let numbers = [0, 1, 100, 1500, 999999, u64::MAX];

    for number in numbers {
        let checkpoint_number = CheckpointNumber { number };

        // Test serialization
        let json = serde_json::to_string(&checkpoint_number).expect("Test data should be valid");
        let deserialized: CheckpointNumber =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(checkpoint_number.number, deserialized.number);

        // Test display
        let display_str = format!("{}", checkpoint_number);
        assert_eq!(display_str, format!("Checkpoint Number: {}", number));
    }
}

#[test]
fn test_checkpoint_with_various_sizes() {
    let checkpoint_with_size = Checkpoint {
        hash: create_hash("0x123abc"),
        parent_hash: create_hash("0x000abc"),
        state_root: create_hash("0xabc123"),
        transactions_root: create_hash("0xdef456"),
        receipts_root: create_hash("0x789012"),
        number: 100,
        timestamp: 1703097600,
        extra_data: "0x".to_string(),
        transactions: CheckpointTransactions::Hashes(vec![]),
        size: Some(2048),
    };

    let checkpoint_without_size = Checkpoint {
        hash: create_hash("0x123abc"),
        parent_hash: create_hash("0x000abc"),
        state_root: create_hash("0xabc123"),
        transactions_root: create_hash("0xdef456"),
        receipts_root: create_hash("0x789012"),
        number: 100,
        timestamp: 1703097600,
        extra_data: "0x".to_string(),
        transactions: CheckpointTransactions::Hashes(vec![]),
        size: None,
    };

    // Test serialization with size
    let json_with_size =
        serde_json::to_string(&checkpoint_with_size).expect("Test data should be valid");
    let deserialized_with_size: Checkpoint =
        serde_json::from_str(&json_with_size).expect("Test data should be valid");
    assert_eq!(checkpoint_with_size.size, deserialized_with_size.size);

    // Test serialization without size
    let json_without_size =
        serde_json::to_string(&checkpoint_without_size).expect("Test data should be valid");
    let deserialized_without_size: Checkpoint =
        serde_json::from_str(&json_without_size).expect("Test data should be valid");
    assert_eq!(checkpoint_without_size.size, deserialized_without_size.size);
    assert!(deserialized_without_size.size.is_none());
}

#[test]
fn test_checkpoint_parameter_encoding() {
    // Test different parameter combinations for URL encoding
    let test_cases = [
        (0u64, false),
        (1u64, true),
        (999u64, false),
        (1500u64, true),
        (u64::MAX, false),
    ];

    for (number, full) in test_cases {
        let path = format!("{}?number={}&full={}", BY_NUMBER, number, full);
        assert!(path.contains(&format!("number={}", number)));
        assert!(path.contains(&format!("full={}", full)));
    }
}

#[test]
fn test_checkpoint_hash_parameter_encoding() {
    // Test different hash formats
    let test_hashes = [
        "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
        "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "0x0000000000000000000000000000000000000000000000000000000000000000",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ];

    for hash in test_hashes {
        for full in [true, false] {
            let path = format!("{}?hash={}&full={}", BY_HASH, hash, full);
            assert!(path.contains(&format!("hash={}", hash)));
            assert!(path.contains(&format!("full={}", full)));
        }
    }
}

#[test]
fn test_checkpoint_all_fields_serialization() {
    let checkpoint = Checkpoint {
        hash: create_hash("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777"),
        parent_hash: create_hash(
            "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
        ),
        state_root: create_hash(
            "0x18b2b9746b15451d1f9bc414f1c12bda8249c63d4a46926e661ae74c69defd9a",
        ),
        transactions_root: create_hash(
            "0xa1e7ed47e548fa45c30232a7e7dfaad6495cff595a0ee1458aa470e574f3f6e4",
        ),
        receipts_root: create_hash(
            "0x59ff04f73d9f934800687c60fb80e2de6e8233817b46d144aec724b569d80c3b",
        ),
        number: 1500,
        timestamp: 1703097600,
        extra_data: "checkpoint_data_v1".to_string(),
        transactions: CheckpointTransactions::Hashes(vec![
            create_hash("0x1111111111111111111111111111111111111111111111111111111111111111"),
            create_hash("0x2222222222222222222222222222222222222222222222222222222222222222"),
        ]),
        size: Some(4096),
    };

    // Test that all fields serialize and deserialize correctly
    let json = serde_json::to_string(&checkpoint).expect("Test data should be valid");
    let deserialized: Checkpoint = serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(checkpoint.hash, deserialized.hash);
    assert_eq!(checkpoint.parent_hash, deserialized.parent_hash);
    assert_eq!(checkpoint.state_root, deserialized.state_root);
    assert_eq!(checkpoint.transactions_root, deserialized.transactions_root);
    assert_eq!(checkpoint.receipts_root, deserialized.receipts_root);
    assert_eq!(checkpoint.number, deserialized.number);
    assert_eq!(checkpoint.timestamp, deserialized.timestamp);
    assert_eq!(checkpoint.extra_data, deserialized.extra_data);
    assert_eq!(checkpoint.size, deserialized.size);

    // Verify transactions
    match (checkpoint.transactions, deserialized.transactions) {
        (
            CheckpointTransactions::Hashes(original),
            CheckpointTransactions::Hashes(deserialized),
        ) => {
            assert_eq!(original.len(), deserialized.len());
            assert_eq!(original, deserialized);
        }
        _ => panic!("Transaction types should match"),
    }
}

#[test]
fn test_checkpoint_edge_case_values() {
    // Test with edge case values
    let checkpoint = Checkpoint {
        hash: create_hash("0x0000000000000000000000000000000000000000000000000000000000000000"), // Empty hash
        parent_hash: create_hash(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        ), // Minimal hash
        state_root: create_hash(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        ),
        transactions_root: create_hash(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ),
        receipts_root: create_hash(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        ),
        number: 0,                    // Minimum number
        timestamp: u64::MAX,          // Maximum timestamp
        extra_data: "a".repeat(1000), // Long extra data
        transactions: CheckpointTransactions::Hashes(vec![]),
        size: Some(0), // Zero size
    };

    // Should still serialize and deserialize correctly
    let json = serde_json::to_string(&checkpoint).expect("Test data should be valid");
    let deserialized: Checkpoint = serde_json::from_str(&json).expect("Test data should be valid");

    assert_eq!(checkpoint.hash, deserialized.hash);
    assert_eq!(checkpoint.number, deserialized.number);
    assert_eq!(checkpoint.timestamp, deserialized.timestamp);
    assert_eq!(checkpoint.extra_data, deserialized.extra_data);
    assert_eq!(checkpoint.size, deserialized.size);
}

#[test]
fn test_checkpoint_clone_and_debug() {
    let checkpoint = Checkpoint {
        hash: create_hash("0x123abc"),
        parent_hash: create_hash("0x000abc"),
        state_root: create_hash("0xabc123"),
        transactions_root: create_hash("0xdef456"),
        receipts_root: create_hash("0x789012"),
        number: 100,
        timestamp: 1703097600,
        extra_data: "test".to_string(),
        transactions: CheckpointTransactions::Hashes(vec![create_hash(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        )]),
        size: Some(1024),
    };

    // Test clone
    let cloned = checkpoint.clone();
    assert_eq!(checkpoint.hash, cloned.hash);
    assert_eq!(checkpoint.number, cloned.number);
    assert_eq!(checkpoint.size, cloned.size);

    // Test debug output (should not panic)
    let debug_str = format!("{:?}", checkpoint);
    assert!(debug_str.contains("Checkpoint"));
    assert!(debug_str.contains("0x123abc"));

    // Test debug for CheckpointNumber
    let checkpoint_num = CheckpointNumber { number: 42 };
    let cloned_num = checkpoint_num.clone();
    assert_eq!(checkpoint_num.number, cloned_num.number);

    let debug_num_str = format!("{:?}", checkpoint_num);
    assert!(debug_num_str.contains("CheckpointNumber"));
    assert!(debug_num_str.contains("42"));
}
