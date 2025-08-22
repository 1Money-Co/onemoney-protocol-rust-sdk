//! Chain-related API response types.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Response type for chain ID endpoint
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainIdResponse {
    pub chain_id: u64,
}

impl Display for ChainIdResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Chain ID: {}", self.chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_response_structure() {
        let chain_id = ChainIdResponse { chain_id: 1212101 };

        // Test serialization
        let json = serde_json::to_string(&chain_id).expect("Should serialize");
        assert!(json.contains("1212101"));

        // Test deserialization
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.chain_id, 1212101);

        // Test display
        let display_str = format!("{}", chain_id);
        assert_eq!(display_str, "Chain ID: 1212101");

        // Test debug
        let debug_str = format!("{:?}", chain_id);
        assert!(debug_str.contains("ChainIdResponse"));
        assert!(debug_str.contains("1212101"));
    }

    #[test]
    fn test_chain_id_response_different_values() {
        let test_cases = [0u64, 1, 1212101, 999999, u64::MAX];

        for chain_id_value in test_cases {
            let chain_id = ChainIdResponse {
                chain_id: chain_id_value,
            };

            // Test serialization round-trip
            let json = serde_json::to_string(&chain_id).expect("Should serialize");
            let deserialized: ChainIdResponse =
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(chain_id.chain_id, deserialized.chain_id);

            // Test display
            let display_str = format!("{}", chain_id);
            assert_eq!(display_str, format!("Chain ID: {}", chain_id_value));
        }
    }

    #[test]
    fn test_chain_id_response_default() {
        let default_chain_id = ChainIdResponse::default();

        assert_eq!(default_chain_id.chain_id, 0);

        // Test that default can be serialized
        let json = serde_json::to_string(&default_chain_id).expect("Should serialize");
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_chain_id, deserialized);
    }

    #[test]
    fn test_chain_id_response_equality_and_hashing() {
        let chain_id1 = ChainIdResponse { chain_id: 1 };
        let chain_id2 = ChainIdResponse { chain_id: 1 };
        let chain_id3 = ChainIdResponse { chain_id: 2 };

        // Test equality
        assert_eq!(chain_id1, chain_id2);
        assert_ne!(chain_id1, chain_id3);

        // Test that Hash trait is implemented and works
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(chain_id1.clone());
        set.insert(chain_id2.clone());
        set.insert(chain_id3.clone());

        // Should have 2 unique chain IDs (chain_id1 == chain_id2)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&chain_id1));
        assert!(set.contains(&chain_id3));
    }

    #[test]
    fn test_chain_id_response_clone() {
        let chain_id = ChainIdResponse { chain_id: 1212101 };
        let cloned = chain_id.clone();

        assert_eq!(chain_id.chain_id, cloned.chain_id);
        assert_eq!(chain_id, cloned);
    }

    #[test]
    fn test_json_format_compatibility() {
        // Test that our structure matches expected JSON format from L1 API

        // ChainIdResponse should serialize as simple object with chain_id field
        let chain_id = ChainIdResponse { chain_id: 123 };
        let json = serde_json::to_string(&chain_id).expect("Should serialize");
        assert_eq!(json, r#"{"chain_id":123}"#);

        // Test deserialization from expected L1 format
        let l1_json = r#"{"chain_id":456}"#;
        let deserialized: ChainIdResponse =
            serde_json::from_str(l1_json).expect("Should deserialize");
        assert_eq!(deserialized.chain_id, 456);
    }

    #[test]
    fn test_chain_id_response_edge_cases() {
        // Test with zero value
        let zero_chain_id = ChainIdResponse { chain_id: 0 };
        let json = serde_json::to_string(&zero_chain_id).expect("Should serialize");
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(zero_chain_id, deserialized);

        // Test with maximum value
        let max_chain_id = ChainIdResponse { chain_id: u64::MAX };
        let json = serde_json::to_string(&max_chain_id).expect("Should serialize");
        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(max_chain_id, deserialized);
    }

    #[test]
    fn test_common_chain_id_values() {
        // Test known chain IDs
        let mainnet = ChainIdResponse { chain_id: 1 };
        assert_eq!(format!("{}", mainnet), "Chain ID: 1");

        let onemoney_chain = ChainIdResponse { chain_id: 1212101 };
        assert_eq!(format!("{}", onemoney_chain), "Chain ID: 1212101");

        // Test serialization of common values
        let json = serde_json::to_string(&onemoney_chain).expect("Should serialize");
        assert!(json.contains("1212101"));

        let deserialized: ChainIdResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.chain_id, 1212101);
    }
}
