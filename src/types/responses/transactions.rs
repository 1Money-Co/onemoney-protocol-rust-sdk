//! Transaction-related API response types.

use alloy_primitives::{Address, B256, Bytes};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::{accounts::Nonce, tokens::TokenMetadata};
use crate::Signature;

/// Chain ID type from L1 primitives
pub type ChainId = u64;

/// Fee estimation result.
/// Matches L1 server's EstimateFee structure: { "fee": String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    /// Estimated fee amount as string.
    pub fee: String,
}

impl Display for FeeEstimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Fee Estimate: {}", self.fee)
    }
}

/// Represents a transaction hash returned by the API.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Hash {
    pub hash: B256,
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Transaction Hash: {}", self.hash)
    }
}

/// Represents a transaction hash and the token that created by the transaction.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HashWithToken {
    /// The hash of the transaction.
    pub hash: B256,
    /// The token that created by the transaction, only works for issuing new
    /// tokens.
    pub token: Address,
}

impl Display for HashWithToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Transaction Hash: {} (Token: {})", self.hash, self.token)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Hash
    pub hash: B256,

    /// Checkpoint hash
    #[serde(default)]
    pub checkpoint_hash: Option<B256>,
    /// Checkpoint number
    #[serde(default)]
    pub checkpoint_number: Option<u64>,
    /// Transaction Index
    #[serde(default)]
    pub transaction_index: Option<u64>,

    /// Epoch
    pub recent_epoch: u64,

    /// Checkpoint
    pub recent_checkpoint: u64,

    /// The chain id of the transaction, if any.
    pub chain_id: ChainId,
    /// Sender
    pub from: Address,
    /// Nonce
    pub nonce: Nonce,

    #[serde(flatten)]
    pub data: TxPayload,

    /// All _flattened_ fields of the transaction signature.
    /// Note: this is an option so special transaction types without a signature (e.g. <https://github.com/ethereum-optimism/optimism/blob/0bf643c4147b43cd6f25a759d331ef3a2a61a2a3/specs/deposits.md#the-deposited-transaction-type>) can be supported.
    pub signature: Signature,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Transaction {}: from {} at recent epoch {} recent checkpoint {} (nonce: {})",
            self.hash, self.from, self.recent_epoch, self.recent_checkpoint, self.nonce
        )?;
        if let Some(checkpoint_hash) = &self.checkpoint_hash {
            write!(f, " in checkpoint {}", checkpoint_hash)?;
        }
        Ok(())
    }
}

/// Transaction receipt response.
/// Matches L1 server's TransactionReceipt structure with proper types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// If transaction is executed successfully.
    pub success: bool,
    /// Transaction Hash.
    pub transaction_hash: B256,
    /// Index within the block.
    pub transaction_index: Option<u64>,
    /// Hash of the checkpoint this transaction was included within.
    pub checkpoint_hash: Option<B256>,
    /// Number of the checkpoint this transaction was included within.
    pub checkpoint_number: Option<u64>,
    /// Fee used.
    pub fee_used: u128,
    /// Address of the sender.
    pub from: Address,
    /// Address of the receiver. None when its a contract creation transaction.
    pub to: Option<Address>,
    /// Token address created, or None if not a deployment.
    pub token_address: Option<Address>,
}

impl Display for TransactionReceipt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Transaction Receipt:")?;
        writeln!(f, "  Success: {}", self.success)?;
        writeln!(f, "  Transaction Hash: {}", self.transaction_hash)?;
        writeln!(f, "  Fee Used: {}", self.fee_used)?;
        if let Some(idx) = self.transaction_index {
            writeln!(f, "  Transaction Index: {}", idx)?;
        }
        if let Some(hash) = &self.checkpoint_hash {
            writeln!(f, "  Checkpoint Hash: {}", hash)?;
        }
        if let Some(num) = self.checkpoint_number {
            writeln!(f, "  Checkpoint Number: {}", num)?;
        }
        writeln!(f, "  From: {}", self.from)?;
        if let Some(to) = &self.to {
            writeln!(f, "  To: {}", to)?;
        }
        if let Some(token) = &self.token_address {
            write!(f, "  Token Address: {}", token)?;
        }
        Ok(())
    }
}

/// Instructions supported by mint token
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "transaction_type", content = "data")]
pub enum TxPayload {
    /// Create a new mint token. After the token is created, the
    /// `master_authority` of the token is initialized with the signer of the
    /// message.
    ///
    /// Refer to `TokenInstruction::CreateNewToken`.
    TokenCreate {
        /// The symbol of the token to create.
        symbol: String,

        /// Number of base 10 digits to the right of the decimal place.
        decimals: u8,

        /// The master authority of the token.
        master_authority: Address,

        /// `true` if this token is private and only whitelisted addresses can
        /// operate with the tokens
        is_private: bool,

        /// The name of the token to create.
        name: String,
    },

    /// Transfer tokens from one account to another. The signer of message must
    /// be the owner of the source account. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::Transfer`.
    TokenTransfer {
        /// The amount of tokens to transfer.
        value: String,

        /// The real recipient address.
        recipient: Address,

        /// The token address, if it's native token, token address is `None`.
        token: Option<Address>,
    },

    /// Grant authority to another account. The signer of message must be the
    /// Mint's `master_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::GrantAuthority`.
    TokenGrantAuthority {
        /// The type of authority to update.
        authority_type: String,
        /// The new authority
        authority_address: Address,
        /// The amount of tokens to mint.
        value: Option<String>,

        /// The token address
        token: Address,
    },

    /// Revoke authority to another account. The signer of message must be the
    /// Mint's `master_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::RevokeAuthority`.
    TokenRevokeAuthority {
        /// The type of authority to update.
        authority_type: String,
        /// The new authority
        authority_address: Address,
        /// The amount of tokens to mint.
        value: Option<String>,

        /// The token address
        token: Address,
    },

    /// Add the account to the blacklisted accounts. The signer of message must
    /// be the Mint's `blacklist_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::BlacklistAccount`.
    TokenBlacklistAccount {
        /// The account to blacklist
        address: Address,

        /// The token address
        token: Address,
    },

    /// Whitelist the a previously blacklisted account. The signer of message
    /// must be the Mint's `blacklist_authority`. Otherwise the transaction may
    /// fail.
    ///
    /// Refer to `TokenInstruction::WhitelistAccount`.
    TokenWhitelistAccount {
        /// The account to whitelist
        address: Address,

        /// The token address
        token: Address,
    },

    /// Mints new tokens to an account. The signer of the message must be Mint's
    /// `mint_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::MintTo`.
    TokenMint {
        /// The amount of new tokens to mint.
        value: String,
        /// The address to mint the tokens to.
        recipient: Address,

        /// The token address
        token: Address,
    },

    /// Burns tokens by removing them from an account. The signer of the message
    /// must be Mint's `mint_burn` authority. Otherwise the transaction may
    /// fail.
    ///
    /// Refer to `TokenInstruction::BurnFromAccount`.
    TokenBurn {
        /// The amount of tokens to burn.
        value: String,
        /// The address to burn the tokens from.
        recipient: Address,

        /// The token address
        token: Address,
    },

    /// Close an account. Note that an account can be closed only if the token
    /// balance is zero.
    ///
    /// Refer to `TokenInstruction::CloseAccount`.
    TokenCloseAccount {
        /// The token address
        token: Address,
    },

    /// Pause all transactions associated with the Mint. The signer of the
    /// message must be the Mint's `pause_authority`. Otherwise the
    /// transaction may fail.
    ///
    /// Refer to `TokenInstruction::Pause`.
    TokenPause {
        /// The token address
        token: Address,
    },

    /// Unpause transactions for the Mint. The signer of the message must be the
    /// Mint's `pause_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::Unpause`.
    TokenUnpause {
        /// The token address
        token: Address,
    },

    /// Update token metadata. The signer of the message must be the Mint's
    /// `metadata_update_authority`. Otherwise the transaction may fail.
    ///
    /// Refer to `TokenInstruction::UpdateMetadata`.
    TokenUpdateMetadata {
        /// The metadata to update
        metadata: TokenMetadata,

        /// The token address
        token: Address,
    },

    /// Raw transaction data, all unsupported instructions are encoded as raw
    /// data.
    ///
    /// This variant is used for all instructions that are not supported by the
    /// current version of the RPC. Just for compatibility.
    Raw {
        /// The input data of the transaction.
        input: Bytes,
        /// The token address
        token: Address,
    },

    // *FIXLATER*: for governance, we don't support them for now.
    Governance,
}

impl TxPayload {
    pub fn is_raw(&self) -> bool {
        matches!(self, TxPayload::Raw { .. })
    }
}

impl Default for TxPayload {
    fn default() -> Self {
        Self::TokenTransfer {
            value: String::default(),
            recipient: Address::default(),
            token: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, B256};
    use std::str::FromStr;

    #[test]
    fn test_fee_estimate_serialization() {
        let fee_estimate = FeeEstimate {
            fee: "1000000000000000000".to_string(),
        };

        let json = serde_json::to_string(&fee_estimate).expect("Test data should be valid");
        let deserialized: FeeEstimate =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(fee_estimate.fee, deserialized.fee);
    }

    #[test]
    fn test_fee_estimate_display() {
        let fee_estimate = FeeEstimate {
            fee: "1000000000000000000".to_string(),
        };

        let display_str = format!("{}", fee_estimate);
        assert_eq!(display_str, "Fee Estimate: 1000000000000000000");
    }

    #[test]
    fn test_hash_serialization() {
        let hash = Hash {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&hash).expect("Test data should be valid");
        let deserialized: Hash = serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(hash.hash, deserialized.hash);
    }

    #[test]
    fn test_hash_display() {
        let hash = Hash {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
        };

        let display_str = format!("{}", hash);
        assert!(display_str.contains("Transaction Hash"));
        assert!(
            display_str
                .contains("0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777")
        );
    }

    #[test]
    fn test_hash_with_token_serialization() {
        let hash_with_token = HashWithToken {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let json = serde_json::to_string(&hash_with_token).expect("Test data should be valid");
        let deserialized: HashWithToken =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(hash_with_token.hash, deserialized.hash);
        assert_eq!(hash_with_token.token, deserialized.token);
    }

    #[test]
    fn test_transaction_serialization() {
        let transaction = Transaction {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            checkpoint_hash: Some(
                B256::from_str(
                    "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                )
                .expect("Test data should be valid"),
            ),
            checkpoint_number: Some(1500),
            transaction_index: Some(0),
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            nonce: 5,
            data: TxPayload::default(),
            signature: Signature::default(),
        };

        let json = serde_json::to_string(&transaction).expect("Test data should be valid");
        let deserialized: Transaction =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(transaction.hash, deserialized.hash);
        assert_eq!(transaction.recent_epoch, deserialized.recent_epoch);
        assert_eq!(
            transaction.recent_checkpoint,
            deserialized.recent_checkpoint
        );
    }

    #[test]
    fn test_transaction_receipt_serialization() {
        let receipt = TransactionReceipt {
            success: true,
            transaction_hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            transaction_index: Some(0),
            checkpoint_hash: Some(
                B256::from_str(
                    "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                )
                .expect("Test data should be valid"),
            ),
            checkpoint_number: Some(1500),
            fee_used: 1000000,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            to: Some(
                Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                    .expect("Test data should be valid"),
            ),
            token_address: None,
        };

        let json = serde_json::to_string(&receipt).expect("Test data should be valid");
        let deserialized: TransactionReceipt =
            serde_json::from_str(&json).expect("Test data should be valid");

        assert_eq!(receipt.success, deserialized.success);
        assert_eq!(receipt.transaction_hash, deserialized.transaction_hash);
        assert_eq!(receipt.fee_used, deserialized.fee_used);
    }

    #[test]
    fn test_tx_payload_token_create_serialization() {
        let payload = TxPayload::TokenCreate {
            symbol: "TEST".to_string(),
            decimals: 18,
            master_authority: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            is_private: false,
            name: "Test Token".to_string(),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");
        let deserialized: TxPayload =
            serde_json::from_str(&json).expect("Test data should be valid");

        match deserialized {
            TxPayload::TokenCreate {
                symbol,
                decimals,
                master_authority,
                is_private,
                name,
            } => {
                assert_eq!(symbol, "TEST");
                assert_eq!(decimals, 18);
                assert_eq!(
                    master_authority,
                    Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                        .expect("Test data should be valid")
                );
                assert!(!is_private);
                assert_eq!(name, "Test Token");
            }
            _ => panic!("Wrong payload type"),
        }
    }

    #[test]
    fn test_tx_payload_token_transfer_serialization() {
        let payload = TxPayload::TokenTransfer {
            value: "1000000000000000000".to_string(),
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            token: Some(
                Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                    .expect("Test data should be valid"),
            ),
        };

        let json = serde_json::to_string(&payload).expect("Test data should be valid");
        let deserialized: TxPayload =
            serde_json::from_str(&json).expect("Test data should be valid");

        match deserialized {
            TxPayload::TokenTransfer {
                value,
                recipient: to,
                token,
            } => {
                assert_eq!(value, "1000000000000000000");
                assert_eq!(
                    to,
                    Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                        .expect("Test data should be valid")
                );
                assert_eq!(
                    token,
                    Some(
                        Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                            .expect("Test data should be valid")
                    )
                );
            }
            _ => panic!("Wrong payload type"),
        }
    }

    #[test]
    fn test_tx_payload_is_raw() {
        let raw_payload = TxPayload::Raw {
            input: Bytes::from(vec![1, 2, 3, 4]),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let transfer_payload = TxPayload::TokenTransfer {
            value: "1000000000000000000".to_string(),
            recipient: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            token: None,
        };

        assert!(raw_payload.is_raw());
        assert!(!transfer_payload.is_raw());
    }

    #[test]
    fn test_tx_payload_default() {
        let default_payload = TxPayload::default();

        match default_payload {
            TxPayload::TokenTransfer {
                value,
                recipient: to,
                token,
            } => {
                assert_eq!(value, String::default());
                assert_eq!(to, Address::default());
                assert_eq!(token, None);
            }
            _ => panic!("Default payload should be TokenTransfer"),
        }
    }

    #[test]
    fn test_hash_with_token_display() {
        let hash_with_token = HashWithToken {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            token: Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                .expect("Test data should be valid"),
        };

        let display_str = format!("{}", hash_with_token);
        assert_eq!(
            display_str,
            "Transaction Hash: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777 (Token: 0x1234567890AbcdEF1234567890aBcdef12345678)"
        );
    }

    #[test]
    fn test_transaction_display_with_checkpoint_hash() {
        let transaction = Transaction {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            checkpoint_hash: Some(
                B256::from_str(
                    "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                )
                .expect("Test data should be valid"),
            ),
            checkpoint_number: Some(200),
            transaction_index: Some(1),
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            nonce: 5,
            data: TxPayload::default(),
            signature: Signature::default(),
        };

        let display_str = format!("{}", transaction);
        assert_eq!(
            display_str,
            "Transaction 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777: from 0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0 at epoch 100 checkpoint 200 (nonce: 5) in checkpoint 0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f"
        );
    }

    #[test]
    fn test_transaction_display_without_checkpoint_hash() {
        let transaction = Transaction {
            hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            checkpoint_hash: None, // This tests the None branch
            checkpoint_number: None,
            transaction_index: None,
            recent_epoch: 100,
            recent_checkpoint: 200,
            chain_id: 1212101,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            nonce: 5,
            data: TxPayload::default(),
            signature: Signature::default(),
        };

        let display_str = format!("{}", transaction);
        assert_eq!(
            display_str,
            "Transaction 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777: from 0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0 at epoch 100 checkpoint 200 (nonce: 5)"
        );
    }

    #[test]
    fn test_transaction_receipt_display() {
        let receipt = TransactionReceipt {
            success: true,
            transaction_hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            transaction_index: Some(5),
            checkpoint_hash: Some(
                B256::from_str(
                    "0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f",
                )
                .expect("Test data should be valid"),
            ),
            checkpoint_number: Some(200),
            fee_used: 1000000000000000000u128,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            to: Some(
                Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
                    .expect("Test data should be valid"),
            ),
            token_address: Some(
                Address::from_str("0xabcdef1234567890abcdef1234567890abcdef12")
                    .expect("Test data should be valid"),
            ),
        };

        let display_str = format!("{}", receipt);
        let expected = "Transaction Receipt:\n  Success: true\n  Transaction Hash: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777\n  Fee Used: 1000000000000000000\n  Transaction Index: 5\n  Checkpoint Hash: 0x20e081da293ae3b81e30f864f38f6911663d7f2cf98337fca38db3cf5bbe7a8f\n  Checkpoint Number: 200\n  From: 0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0\n  To: 0x1234567890AbcdEF1234567890aBcdef12345678\n  Token Address: 0xabCDEF1234567890ABcDEF1234567890aBCDeF12";
        assert_eq!(display_str, expected);
    }

    #[test]
    fn test_transaction_receipt_display_minimal() {
        let receipt = TransactionReceipt {
            success: false,
            transaction_hash: B256::from_str(
                "0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777",
            )
            .expect("Test data should be valid"),
            transaction_index: None, // Test None branch
            checkpoint_hash: None,   // Test None branch
            checkpoint_number: None, // Test None branch
            fee_used: 500000000000000000u128,
            from: Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0")
                .expect("Test data should be valid"),
            to: None,            // Test None branch
            token_address: None, // Test None branch
        };

        let display_str = format!("{}", receipt);
        let expected = "Transaction Receipt:\n  Success: false\n  Transaction Hash: 0x902006665c369834a0cf52eea2780f934a90b3c86a3918fb57371ac1fbbd7777\n  Fee Used: 500000000000000000\n  From: 0x742d35Cc6634c0532925a3b8D91D6f4a81B8cbc0\n";
        assert_eq!(display_str, expected);
    }
}
