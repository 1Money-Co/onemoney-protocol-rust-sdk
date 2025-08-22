//! State-related API response types.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Response type for latest state endpoint
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LatestStateResponse {
    /// Current epoch number.
    pub epoch: u64,
    /// Current checkpoint number.
    pub checkpoint: u64,
}

impl Display for LatestStateResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Latest State: epoch={}, checkpoint={}",
            self.epoch, self.checkpoint
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latest_state_response_structure() {
        let state = LatestStateResponse {
            epoch: 100,
            checkpoint: 1500,
        };

        // Test serialization
        let json = serde_json::to_string(&state).expect("Should serialize");
        assert!(json.contains("100"));
        assert!(json.contains("1500"));

        // Test deserialization
        let deserialized: LatestStateResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.epoch, 100);
        assert_eq!(deserialized.checkpoint, 1500);

        // Test display
        let display_str = format!("{}", state);
        assert_eq!(display_str, "Latest State: epoch=100, checkpoint=1500");

        // Test debug
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("LatestStateResponse"));
        assert!(debug_str.contains("epoch: 100"));
        assert!(debug_str.contains("checkpoint: 1500"));
    }

    #[test]
    fn test_latest_state_response_different_values() {
        let test_cases = [
            (0u64, 0u64),
            (1, 1),
            (100, 200),
            (999999, 123456),
            (u64::MAX, u64::MAX),
        ];

        for (epoch_val, checkpoint_val) in test_cases {
            let state = LatestStateResponse {
                epoch: epoch_val,
                checkpoint: checkpoint_val,
            };

            // Test serialization round-trip
            let json = serde_json::to_string(&state).expect("Should serialize");
            let deserialized: LatestStateResponse =
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(state.epoch, deserialized.epoch);
            assert_eq!(state.checkpoint, deserialized.checkpoint);

            // Test display
            let display_str = format!("{}", state);
            assert_eq!(
                display_str,
                format!(
                    "Latest State: epoch={}, checkpoint={}",
                    epoch_val, checkpoint_val
                )
            );
        }
    }

    #[test]
    fn test_latest_state_response_default() {
        let default_state = LatestStateResponse::default();

        assert_eq!(default_state.epoch, 0);
        assert_eq!(default_state.checkpoint, 0);

        // Test that default can be serialized
        let json = serde_json::to_string(&default_state).expect("Should serialize");
        let deserialized: LatestStateResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_state, deserialized);

        // Test display of default
        let display_str = format!("{}", default_state);
        assert_eq!(display_str, "Latest State: epoch=0, checkpoint=0");
    }

    #[test]
    fn test_latest_state_response_equality_and_hashing() {
        let state1 = LatestStateResponse {
            epoch: 100,
            checkpoint: 200,
        };
        let state2 = LatestStateResponse {
            epoch: 100,
            checkpoint: 200,
        };
        let state3 = LatestStateResponse {
            epoch: 101,
            checkpoint: 200,
        };

        // Test equality
        assert_eq!(state1, state2);
        assert_ne!(state1, state3);

        // Test that Hash trait is implemented and works
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(state1.clone());
        set.insert(state2.clone());
        set.insert(state3.clone());

        // Should have 2 unique states (state1 == state2)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&state1));
        assert!(set.contains(&state3));
    }

    #[test]
    fn test_latest_state_response_clone() {
        let state = LatestStateResponse {
            epoch: 42,
            checkpoint: 84,
        };
        let cloned = state.clone();

        assert_eq!(state.epoch, cloned.epoch);
        assert_eq!(state.checkpoint, cloned.checkpoint);
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_json_format_compatibility() {
        // Test that our structure matches expected JSON format from L1 API

        // LatestStateResponse should serialize with both fields
        let state = LatestStateResponse {
            epoch: 123,
            checkpoint: 456,
        };
        let json = serde_json::to_string(&state).expect("Should serialize");
        assert_eq!(json, r#"{"epoch":123,"checkpoint":456}"#);

        // Test deserialization from expected L1 format
        let l1_json = r#"{"epoch":789,"checkpoint":1011}"#;
        let deserialized: LatestStateResponse =
            serde_json::from_str(l1_json).expect("Should deserialize");
        assert_eq!(deserialized.epoch, 789);
        assert_eq!(deserialized.checkpoint, 1011);
    }

    #[test]
    fn test_latest_state_response_edge_cases() {
        // Test with zero values
        let zero_state = LatestStateResponse {
            epoch: 0,
            checkpoint: 0,
        };
        let json = serde_json::to_string(&zero_state).expect("Should serialize");
        let deserialized: LatestStateResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(zero_state, deserialized);

        // Test with maximum values
        let max_state = LatestStateResponse {
            epoch: u64::MAX,
            checkpoint: u64::MAX,
        };
        let json = serde_json::to_string(&max_state).expect("Should serialize");
        let deserialized: LatestStateResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(max_state, deserialized);
    }

    #[test]
    fn test_typical_blockchain_state_values() {
        // Test realistic blockchain state values
        let current_state = LatestStateResponse {
            epoch: 1024,
            checkpoint: 15384,
        };

        // Test display formatting
        let display_str = format!("{}", current_state);
        assert_eq!(display_str, "Latest State: epoch=1024, checkpoint=15384");

        // Test JSON serialization of realistic values
        let json = serde_json::to_string(&current_state).expect("Should serialize");
        assert!(json.contains("1024"));
        assert!(json.contains("15384"));

        let deserialized: LatestStateResponse =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.epoch, 1024);
        assert_eq!(deserialized.checkpoint, 15384);
    }

    #[test]
    fn test_state_progression_ordering() {
        // Test that states can be compared logically (though PartialOrd is not implemented)
        let earlier_state = LatestStateResponse {
            epoch: 10,
            checkpoint: 100,
        };

        let later_state = LatestStateResponse {
            epoch: 10,
            checkpoint: 101,
        };

        let much_later_state = LatestStateResponse {
            epoch: 11,
            checkpoint: 50,
        };

        // These should be different
        assert_ne!(earlier_state, later_state);
        assert_ne!(earlier_state, much_later_state);
        assert_ne!(later_state, much_later_state);

        // Test that individual fields can be compared
        assert!(earlier_state.checkpoint < later_state.checkpoint);
        assert!(earlier_state.epoch < much_later_state.epoch);
    }

    #[test]
    fn test_field_access() {
        let state = LatestStateResponse {
            epoch: 42,
            checkpoint: 84,
        };

        // Test direct field access
        assert_eq!(state.epoch, 42);
        assert_eq!(state.checkpoint, 84);

        // Test field modification (through mutable instance)
        let mut mutable_state = state.clone();
        mutable_state.epoch = 43;
        mutable_state.checkpoint = 85;

        assert_eq!(mutable_state.epoch, 43);
        assert_eq!(mutable_state.checkpoint, 85);
        assert_ne!(state, mutable_state);
    }
}
