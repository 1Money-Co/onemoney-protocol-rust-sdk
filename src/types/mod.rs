//! Type definitions for the OneMoney SDK.

// Original SDK types
pub mod common;

// New organized API types
pub mod requests;
pub mod responses;

// Re-export commonly used types from original SDK
pub use common::*;
// Note: accounts, checkpoints, transactions types are now in responses/

// Re-export authority types (avoid conflicts with API types)
pub use requests::authorities::{Authority, AuthorityAction};

// Re-export action types from requests module
pub use requests::{BlacklistAction, PauseAction, WhitelistAction};

// Re-export request types
pub use requests::tokens::*;
pub use requests::transactions::*;

// Re-export response types
pub use responses::accounts::*;
pub use responses::chains::*;
pub use responses::checkpoints::*;
pub use responses::tokens::*;
pub use responses::transactions::*;

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn test_types_module_organization() {
        // Test that all core types are accessible through the main types module

        // Test common types
        let _signature = Signature::default();
        let _action_type = ActionType::Payment;

        // Test authority types
        let _authority = Authority::MintBurnTokens;
        let _auth_action = AuthorityAction::Grant;

        // Test action types
        let _blacklist_action = BlacklistAction::Add;
        let _pause_action = PauseAction::Pause;
        let _whitelist_action = WhitelistAction::Add;

        // All types are accessible through re-exports if compilation succeeds
    }

    #[test]
    fn test_request_types_accessibility() {
        // Test that all request types can be created and used
        let token_address =
            Address::from_str("0x1234567890abcdef1234567890abcdef12345678").expect("Valid address");
        let recipient =
            Address::from_str("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd").expect("Valid address");

        // Test token request types
        let _mint_payload = TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token_address,
            recipient,
            value: U256::from(1000u64),
        };

        let _burn_payload = TokenBurnPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token_address,
            recipient,
            value: U256::from(1000u64),
        };

        let _authority_payload = TokenAuthorityPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            action: AuthorityAction::Grant,
            authority_type: Authority::MintBurnTokens,
            authority_address: recipient,
            token: token_address,
            value: U256::from(1000u64),
        };

        // Test transaction request types
        let _payment_payload = PaymentPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            recipient,
            value: U256::from(1000u64),
            token: token_address,
        };

        // All request types are constructible if compilation succeeds
    }

    #[test]
    fn test_response_types_accessibility() {
        // Test that all response types can be created and used
        let test_address =
            Address::from_str("0x1234567890abcdef1234567890abcdef12345678").expect("Valid address");

        // Test account response types
        let _nonce = AccountNonce { nonce: 100 };
        let _token_account = AssociatedTokenAccount {
            token_account_address: test_address,
            balance: "1000".to_string(),
            nonce: 42,
        };

        // Test chain response types
        let _chain_id = ChainIdResponse { chain_id: 1 };

        // Test transaction response types
        use crate::responses::TransactionResponse;
        use alloy_primitives::B256;
        let _tx_response = TransactionResponse { hash: B256::ZERO };

        // All response types are constructible if compilation succeeds
    }

    #[test]
    fn test_enum_completeness() {
        // Test that all enum variants are accessible

        // Authority enum
        let authorities = [
            Authority::MasterMintBurn,
            Authority::MintBurnTokens,
            Authority::Pause,
            Authority::ManageList,
            Authority::UpdateMetadata,
        ];

        for authority in authorities {
            assert_ne!(
                format!("{:?}", authority),
                "",
                "Authority should have debug output"
            );
        }

        // AuthorityAction enum
        let auth_actions = [AuthorityAction::Grant, AuthorityAction::Revoke];
        for action in auth_actions {
            assert_ne!(
                format!("{:?}", action),
                "",
                "AuthorityAction should have debug output"
            );
        }

        // Action type enums
        let blacklist_actions = [BlacklistAction::Add, BlacklistAction::Remove];
        let pause_actions = [PauseAction::Pause, PauseAction::Unpause];
        let whitelist_actions = [WhitelistAction::Add, WhitelistAction::Remove];

        for action in blacklist_actions {
            assert_ne!(
                format!("{:?}", action),
                "",
                "BlacklistAction should have debug output"
            );
        }

        for action in pause_actions {
            assert_ne!(
                format!("{:?}", action),
                "",
                "PauseAction should have debug output"
            );
        }

        for action in whitelist_actions {
            assert_ne!(
                format!("{:?}", action),
                "",
                "WhitelistAction should have debug output"
            );
        }
    }

    #[test]
    fn test_types_serialization_compatibility() {
        // Test that types can be serialized/deserialized
        let token_address = Address::ZERO;
        let recipient = Address::from([0xFF; 20]);

        let mint_payload = TokenMintPayload {
            recent_checkpoint: 200,
            chain_id: 1,
            nonce: 1,
            token: token_address,
            recipient,
            value: U256::from(1000u64),
        };

        // Should be able to serialize
        let json = serde_json::to_string(&mint_payload);
        assert!(json.is_ok(), "Types should be serializable");

        // Should be able to deserialize
        let restored: Result<TokenMintPayload, _> = serde_json::from_str(&json.unwrap());
        assert!(restored.is_ok(), "Types should be deserializable");

        let restored_payload = restored.unwrap();
        assert_eq!(restored_payload.token, mint_payload.token);
        assert_eq!(restored_payload.value, mint_payload.value);
    }

    #[test]
    fn test_module_boundaries() {
        // Test that module organization makes logical sense

        // Request types should be in requests module
        use requests::tokens::TokenMintPayload as RequestMintPayload;

        // Response types should be in responses module
        use responses::chains::ChainIdResponse as ResponseChainId;

        // Both should be accessible and distinct
        let _request_mint = RequestMintPayload {
            recent_checkpoint: 1,
            chain_id: 1,
            nonce: 1,
            token: Address::ZERO,
            recipient: Address::ZERO,
            value: U256::ZERO,
        };

        let _response_chain = ResponseChainId { chain_id: 1 };

        // Module boundaries are clear and logical if compilation succeeds
    }
}
