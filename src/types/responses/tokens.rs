//! Token-related API response types.

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// `MintInfo` is the struct for token contract. One mint account represents one
/// token.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintInfo {
    /// The symbol of the token. Created during token creation and cannot be
    /// changed.
    pub symbol: String,

    /// `master_authority` used to create new tokens. The `master_authority` can
    /// be provided if and only if during token creation. If `master_authority`
    /// is `EMPTY_ADDRESS`, the token hasn't been initialized. When the token is
    /// initialized with `master_authority`, only the `master_authority` can
    /// grant other role authorities to others. And the `master_authority`
    /// serves as the identifier address for the token. All token account's mint
    /// field will be associated with the mint's `master_authority`.
    pub master_authority: Address,

    /// The authority that can grant individual `mint_burn_authorities`. The
    /// `master_mint_burn_authority` is created by `master_authority`, which
    /// delegate the mint authority to other accounts.
    pub master_mint_burn_authority: Address,

    /// The collection of authorities to mint and burn tokens with a given
    /// allowance. If the allowance is used up, the authority is not able to
    /// mint any more tokens until the allowance is updated.
    ///
    /// The allowance to burn is unlimited. Maximum of 20 authorities.
    pub mint_burn_authorities: Vec<MinterAllowance>,

    /// The authorities to pause/unpause token transactions. Maximum of 5
    /// authorities.
    pub pause_authorities: Vec<Address>,

    /// The authorities to blacklist/whitelist malicious accounts
    pub list_authorities: Vec<Address>,

    /// A blacklist of token accounts
    pub black_list: Vec<Address>,

    /// A whitelist of token accounts. Only used if the token is private
    pub white_list: Vec<Address>,

    /// The authorities for updating the metadata. Maximum of 5 authorities.
    pub metadata_update_authorities: Vec<Address>,

    /// Total supply of tokens.
    pub supply: String,

    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,

    /// `true` if all transactions for this token are paused
    pub is_paused: bool,

    /// `true` if this token is private and only whitelisted addresses can
    /// operate with the tokens
    pub is_private: bool,

    /// Metadata of the token
    pub meta: Option<TokenMetadata>,
}

impl Display for MintInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Token Info:\n  Symbol: {}\n  Master Authority: {}\n  Supply: {}\n  Decimals: {}\n  Paused: {}\n  Private: {}",
            self.symbol,
            self.master_authority,
            self.supply,
            self.decimals,
            self.is_paused,
            self.is_private
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinterAllowance {
    pub minter: Address,
    pub allowance: String,
}

impl Display for MinterAllowance {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Minter: {} (Allowance: {})", self.minter, self.allowance)
    }
}

/// Token metadata for one token.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// The longer name of the token
    pub name: String,

    /// The URI pointing to richer metadata
    pub uri: String,

    /// must avoid storing the same key twice
    pub additional_metadata: Vec<MetadataKVPair>,
}

impl Display for TokenMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Token Metadata: {} (URI: {})", self.name, self.uri)?;
        if !self.additional_metadata.is_empty() {
            write!(
                f,
                " [{} additional properties]",
                self.additional_metadata.len()
            )?;
        }
        Ok(())
    }
}

/// The additional key-value properties for one token.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MetadataKVPair {
    pub key: String,
    pub value: String,
}

impl Display for MetadataKVPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl rlp::Encodable for MetadataKVPair {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(2);
        s.append(&self.key);
        s.append(&self.value);
    }
}
