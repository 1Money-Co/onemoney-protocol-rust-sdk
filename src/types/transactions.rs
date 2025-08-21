//! Transaction-related type definitions.

use super::common::{OneMoneyAddress, Signature, TokenAmount};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Transaction information matching L1 server response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash.
    pub hash: String,
    /// Checkpoint hash (optional).
    pub checkpoint_hash: Option<String>,
    /// Checkpoint number (optional).
    pub checkpoint_number: Option<u64>,
    /// Transaction index (optional).
    pub transaction_index: Option<u64>,
    /// Epoch number.
    pub epoch: u64,
    /// Checkpoint.
    pub checkpoint: u64,
    /// Chain ID.
    pub chain_id: u64,
    /// From address.
    pub from: OneMoneyAddress,
    /// Nonce.
    pub nonce: u64,
    /// Transaction data.
    #[serde(flatten)]
    pub data: TransactionData,
    /// Transaction signature.
    pub signature: Signature,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Transaction Details:")?;
        writeln!(f, "  Hash: {}", self.hash)?;
        writeln!(f, "  From: {}", self.from)?;
        writeln!(f, "  Chain ID: {}", self.chain_id)?;
        writeln!(f, "  Nonce: {}", self.nonce)?;
        writeln!(f, "  Epoch: {}", self.epoch)?;
        writeln!(f, "  Checkpoint: {}", self.checkpoint)?;

        if let Some(checkpoint_hash) = &self.checkpoint_hash {
            writeln!(f, "  Checkpoint Hash: {}", checkpoint_hash)?;
        } else {
            writeln!(f, "  Checkpoint Hash: (pending)")?;
        }

        if let Some(checkpoint_number) = self.checkpoint_number {
            writeln!(f, "  Checkpoint Number: {}", checkpoint_number)?;
        } else {
            writeln!(f, "  Checkpoint Number: (pending)")?;
        }

        if let Some(transaction_index) = self.transaction_index {
            writeln!(f, "  Transaction Index: {}", transaction_index)?;
        } else {
            writeln!(f, "  Transaction Index: (pending)")?;
        }

        writeln!(f, "  Transaction Type: {}", self.data)?;
        writeln!(f, "  Signature:")?;
        writeln!(f, "    R: {:#x}", self.signature.r)?;
        writeln!(f, "    S: {:#x}", self.signature.s)?;
        write!(f, "    V: {}", self.signature.v)?;

        Ok(())
    }
}

/// Transaction data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "transaction_type", content = "data")]
pub enum TransactionData {
    /// Token creation.
    TokenCreate {
        /// Token symbol.
        symbol: String,
        /// Token decimals.
        decimals: u8,
        /// Master authority.
        master_authority: OneMoneyAddress,
        /// Whether token is private.
        is_private: bool,
        /// Token name.
        name: String,
    },
    /// Token transfer.
    TokenTransfer {
        /// Transfer value.
        value: String,
        /// Recipient address.
        to: OneMoneyAddress,
        /// Token address (None for native token).
        token: Option<OneMoneyAddress>,
    },
    /// Grant authority.
    TokenGrantAuthority {
        /// Authority type.
        authority_type: String,
        /// New authority address.
        new_authority: OneMoneyAddress,
        /// Amount of tokens to mint (optional).
        mint_tokens: Option<String>,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Revoke authority.
    TokenRevokeAuthority {
        /// Authority type.
        authority_type: String,
        /// Authority address to revoke.
        new_authority: OneMoneyAddress,
        /// Amount of tokens to mint (optional).
        mint_tokens: Option<String>,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Blacklist account.
    TokenBlacklistAccount {
        /// Address to blacklist.
        address: OneMoneyAddress,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Whitelist account.
    TokenWhitelistAccount {
        /// Address to whitelist.
        address: OneMoneyAddress,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Mint tokens.
    TokenMint {
        /// Amount to mint.
        value: String,
        /// Address to mint to.
        address: OneMoneyAddress,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Burn tokens.
    TokenBurn {
        /// Amount to burn.
        value: String,
        /// Address to burn from.
        address: OneMoneyAddress,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Close account.
    TokenCloseAccount {
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Pause token.
    TokenPause {
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Unpause token.
    TokenUnpause {
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Update metadata.
    TokenUpdateMetadata {
        /// New metadata.
        metadata: serde_json::Value, // Using Value for flexibility
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Raw transaction data.
    Raw {
        /// Input data.
        input: String,
        /// Token address.
        token: OneMoneyAddress,
    },
    /// Governance transaction.
    Governance,
}

impl Display for TransactionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TransactionData::TokenCreate {
                symbol,
                decimals,
                master_authority,
                is_private,
                name,
            } => {
                writeln!(f, "Token Creation:")?;
                writeln!(f, "    Name: {}", name)?;
                writeln!(f, "    Symbol: {}", symbol)?;
                writeln!(f, "    Decimals: {}", decimals)?;
                writeln!(f, "    Master Authority: {}", master_authority)?;
                write!(f, "    Is Private: {}", is_private)
            }
            TransactionData::TokenTransfer { value, to, token } => {
                writeln!(f, "Token Transfer:")?;
                writeln!(f, "    Amount: {}", value)?;
                writeln!(f, "    To: {}", to)?;
                if let Some(token_addr) = token {
                    write!(f, "    Token: {}", token_addr)
                } else {
                    write!(f, "    Token: Native Token")
                }
            }
            TransactionData::TokenGrantAuthority {
                authority_type,
                new_authority,
                mint_tokens,
                token,
            } => {
                writeln!(f, "Grant Authority:")?;
                writeln!(f, "    Authority Type: {}", authority_type)?;
                writeln!(f, "    New Authority: {}", new_authority)?;
                if let Some(mint_amount) = mint_tokens {
                    writeln!(f, "    Mint Tokens: {}", mint_amount)?;
                } else {
                    writeln!(f, "    Mint Tokens: None")?;
                }
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenRevokeAuthority {
                authority_type,
                new_authority,
                mint_tokens,
                token,
            } => {
                writeln!(f, "Revoke Authority:")?;
                writeln!(f, "    Authority Type: {}", authority_type)?;
                writeln!(f, "    Authority: {}", new_authority)?;
                if let Some(mint_amount) = mint_tokens {
                    writeln!(f, "    Mint Tokens: {}", mint_amount)?;
                } else {
                    writeln!(f, "    Mint Tokens: None")?;
                }
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenMint {
                value,
                address,
                token,
            } => {
                writeln!(f, "Token Mint:")?;
                writeln!(f, "    Amount: {}", value)?;
                writeln!(f, "    To Address: {}", address)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenBurn {
                value,
                address,
                token,
            } => {
                writeln!(f, "Token Burn:")?;
                writeln!(f, "    Amount: {}", value)?;
                writeln!(f, "    From Address: {}", address)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenBlacklistAccount { address, token } => {
                writeln!(f, "Blacklist Account:")?;
                writeln!(f, "    Address: {}", address)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenWhitelistAccount { address, token } => {
                writeln!(f, "Whitelist Account:")?;
                writeln!(f, "    Address: {}", address)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenCloseAccount { token } => {
                writeln!(f, "Close Account:")?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenPause { token } => {
                writeln!(f, "Pause Token:")?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenUnpause { token } => {
                writeln!(f, "Unpause Token:")?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::TokenUpdateMetadata { metadata, token } => {
                writeln!(f, "Update Metadata:")?;
                writeln!(f, "    Metadata: {}", metadata)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::Raw { input, token } => {
                writeln!(f, "Raw Transaction:")?;
                writeln!(f, "    Input Data: {}", input)?;
                write!(f, "    Token: {}", token)
            }
            TransactionData::Governance => {
                write!(f, "Governance Transaction")
            }
        }
    }
}

/// Transaction status (for compatibility).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    /// Transaction succeeded.
    Success,
    /// Transaction failed.
    Failed,
    /// Transaction is pending.
    Pending,
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let status_str = match self {
            TransactionStatus::Success => "Success",
            TransactionStatus::Failed => "Failed",
            TransactionStatus::Pending => "Pending",
        };
        write!(f, "{}", status_str)
    }
}

/// Fee estimation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    /// Estimated gas limit.
    pub gas_limit: u64,
    /// Estimated gas price.
    pub gas_price: TokenAmount,
    /// Total estimated fee.
    pub total_fee: TokenAmount,
}

impl Display for FeeEstimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Fee Estimate: {} gas @ {} per unit = {} total",
            self.gas_limit, self.gas_price, self.total_fee
        )
    }
}

/// Transaction hash response (matching REST API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hash {
    /// Transaction hash.
    pub hash: String,
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Transaction hash: {}", self.hash)
    }
}

/// Transaction hash with token address response (matching REST API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashWithToken {
    /// Transaction hash.
    pub hash: String,
    /// Token address created by the transaction.
    pub token: OneMoneyAddress,
}

impl Display for HashWithToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Transaction hash: {}, Token address: {}",
            self.hash, self.token
        )
    }
}
