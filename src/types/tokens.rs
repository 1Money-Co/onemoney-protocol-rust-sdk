//! Token-related type definitions.

use alloy_primitives::Address;
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Token authority action types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorityAction {
    /// Grant authority to an address.
    Grant,
    /// Revoke authority from an address.
    Revoke,
}

/// Token authority types matching L1 server implementation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Authority {
    /// Master mint/burn authority - full control over token.
    MasterMintBurn,
    /// Mint/burn authority with limited allowance.
    MintBurnTokens,
    /// Pause/unpause authority.
    Pause,
    /// Manage blacklist/whitelist authority.
    ManageList,
    /// Update metadata authority.
    UpdateMetadata,
}

impl Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let authority_name = match self {
            Authority::MasterMintBurn => "Master Mint/Burn Authority",
            Authority::MintBurnTokens => "Mint/Burn Tokens Authority",
            Authority::Pause => "Pause Authority",
            Authority::ManageList => "Manage List Authority",
            Authority::UpdateMetadata => "Update Metadata Authority",
        };
        write!(f, "{}", authority_name)
    }
}

/// Token metadata (part of MintInfo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// The longer name of the token.
    pub name: String,
    /// The URI pointing to richer metadata.
    pub uri: String,
    /// Additional key-value metadata pairs.
    pub additional_metadata: Vec<MetadataKVPair>,
}

/// Additional key-value properties for token metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataKVPair {
    /// Metadata key.
    pub key: String,
    /// Metadata value.
    pub value: String,
}

impl Encodable for MetadataKVPair {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.key);
        s.append(&self.value);
    }
}

/// Minter allowance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinterAllowance {
    /// Minter address.
    pub minter: Address,
    /// Allowance amount as string.
    pub allowance: String,
}

/// Complete token mint information from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintInfo {
    /// Token symbol.
    pub symbol: String,
    /// Master authority address.
    pub master_authority: Address,
    /// Master mint/burn authority address.
    pub master_mint_burn_authority: Address,
    /// Mint/burn authorities with allowances.
    pub mint_burn_authorities: Vec<MinterAllowance>,
    /// Pause authorities.
    pub pause_authorities: Vec<Address>,
    /// List management authorities.
    pub list_authorities: Vec<Address>,
    /// Blacklisted addresses.
    pub black_list: Vec<Address>,
    /// Whitelisted addresses.
    pub white_list: Vec<Address>,
    /// Metadata update authorities.
    pub metadata_update_authorities: Vec<Address>,
    /// Total supply as string.
    pub supply: String,
    /// Number of decimal places.
    pub decimals: u8,
    /// Whether token transactions are paused.
    pub is_paused: bool,
    /// Whether token is private.
    pub is_private: bool,
    /// Optional token metadata.
    pub meta: Option<TokenMetadata>,
}

impl Display for TokenMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({})", self.name, self.uri)
    }
}

impl Display for MintInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let privacy = if self.is_private { "private" } else { "public" };
        let paused = if self.is_paused { "paused" } else { "active" };

        writeln!(f, "Token Information:")?;
        writeln!(f, "  Symbol: {}", self.symbol)?;
        writeln!(f, "  Type: {} {}", privacy, paused)?;
        writeln!(f, "  Decimals: {}", self.decimals)?;
        writeln!(f, "  Supply: {}", self.supply)?;
        writeln!(f, "  Master Authority: {}", self.master_authority)?;
        writeln!(
            f,
            "  Master Mint/Burn Authority: {}",
            self.master_mint_burn_authority
        )?;

        if !self.mint_burn_authorities.is_empty() {
            writeln!(
                f,
                "  Mint/Burn Authorities: {} authorities",
                self.mint_burn_authorities.len()
            )?;
            for (i, auth) in self.mint_burn_authorities.iter().enumerate() {
                writeln!(
                    f,
                    "    {}: {} (allowance: {})",
                    i + 1,
                    auth.minter,
                    auth.allowance
                )?;
            }
        }

        if !self.pause_authorities.is_empty() {
            writeln!(f, "  Pause Authorities: {}", self.pause_authorities.len())?;
            for (i, auth) in self.pause_authorities.iter().enumerate() {
                writeln!(f, "    {}: {}", i + 1, auth)?;
            }
        }

        if !self.list_authorities.is_empty() {
            writeln!(
                f,
                "  List Management Authorities: {}",
                self.list_authorities.len()
            )?;
            for (i, auth) in self.list_authorities.iter().enumerate() {
                writeln!(f, "    {}: {}", i + 1, auth)?;
            }
        }

        if !self.metadata_update_authorities.is_empty() {
            writeln!(
                f,
                "  Metadata Update Authorities: {}",
                self.metadata_update_authorities.len()
            )?;
            for (i, auth) in self.metadata_update_authorities.iter().enumerate() {
                writeln!(f, "    {}: {}", i + 1, auth)?;
            }
        }

        if !self.black_list.is_empty() {
            writeln!(f, "  Blacklist: {} addresses", self.black_list.len())?;
            for (i, addr) in self.black_list.iter().enumerate() {
                writeln!(f, "    {}: {}", i + 1, addr)?;
            }
        }

        if !self.white_list.is_empty() {
            writeln!(f, "  Whitelist: {} addresses", self.white_list.len())?;
            for (i, addr) in self.white_list.iter().enumerate() {
                writeln!(f, "    {}: {}", i + 1, addr)?;
            }
        }

        if let Some(metadata) = &self.meta {
            writeln!(f, "  Metadata:")?;
            writeln!(f, "    Name: {}", metadata.name)?;
            writeln!(f, "    URI: {}", metadata.uri)?;
            if !metadata.additional_metadata.is_empty() {
                writeln!(f, "    Additional Properties:")?;
                for prop in &metadata.additional_metadata {
                    writeln!(f, "      {}: {}", prop.key, prop.value)?;
                }
            }
        }

        Ok(())
    }
}

/// Associated token account information from API response.
/// Matches the L1 server's AssociatedTokenAccount structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedTokenAccount {
    /// The derived token account address.
    pub token_account_address: Address,
    /// Token balance as string.
    pub balance: String,
    /// Owner account nonce.
    pub nonce: u64,
}

impl Display for AssociatedTokenAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Associated Token Account {}: balance = {}, owner nonce = {}",
            self.token_account_address, self.balance, self.nonce
        )
    }
}
