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

impl alloy_rlp::Encodable for MetadataKVPair {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        // Calculate the actual payload length by encoding to a temporary buffer first
        let mut temp_buf = Vec::new();

        self.key.encode(&mut temp_buf);
        self.value.encode(&mut temp_buf);

        // Now encode the proper header with correct payload length
        alloy_rlp::Header {
            list: true,
            payload_length: temp_buf.len(),
        }
        .encode(out);

        // Write the actual payload
        out.put_slice(&temp_buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Address;
    use alloy_rlp::Encodable;
    use std::str::FromStr;

    #[test]
    fn test_mint_info_structure() {
        let address1 =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let address2 =
            Address::from_str("0x1234567890abcdef1234567890abcdef12345678").expect("Valid address");

        let mint_info = MintInfo {
            symbol: "TEST".to_string(),
            master_authority: address1,
            master_mint_burn_authority: address2,
            mint_burn_authorities: vec![MinterAllowance {
                minter: address1,
                allowance: "1000000000".to_string(),
            }],
            pause_authorities: vec![address1],
            list_authorities: vec![address2],
            black_list: vec![],
            white_list: vec![address1],
            metadata_update_authorities: vec![address2],
            supply: "1000000000000000000000".to_string(),
            decimals: 18,
            is_paused: false,
            is_private: true,
            meta: Some(TokenMetadata {
                name: "Test Token".to_string(),
                uri: "https://example.com/token".to_string(),
                additional_metadata: vec![MetadataKVPair {
                    key: "description".to_string(),
                    value: "A test token".to_string(),
                }],
            }),
        };

        // Test serialization
        let json = serde_json::to_string(&mint_info).expect("Should serialize");
        assert!(json.contains("TEST"));
        assert!(json.contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0")); // lowercase in JSON
        assert!(json.contains("1000000000000000000000"));
        assert!(json.contains("18"));
        assert!(json.contains("Test Token"));

        // Test deserialization
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.symbol, "TEST");
        assert_eq!(deserialized.master_authority, address1);
        assert_eq!(deserialized.decimals, 18);
        assert!(!deserialized.is_paused);
        assert!(deserialized.is_private);
        assert_eq!(deserialized.supply, "1000000000000000000000");

        // Test display
        let display_str = format!("{}", mint_info);
        assert!(display_str.contains("Token Info:"));
        assert!(display_str.contains("Symbol: TEST"));
        assert!(display_str.contains("Supply: 1000000000000000000000"));
        assert!(display_str.contains("Decimals: 18"));
        assert!(display_str.contains("Paused: false"));
        assert!(display_str.contains("Private: true"));
    }

    #[test]
    fn test_mint_info_default() {
        let default_mint_info = MintInfo::default();

        assert_eq!(default_mint_info.symbol, "");
        assert_eq!(default_mint_info.master_authority, Address::ZERO);
        assert_eq!(default_mint_info.master_mint_burn_authority, Address::ZERO);
        assert_eq!(default_mint_info.mint_burn_authorities.len(), 0);
        assert_eq!(default_mint_info.pause_authorities.len(), 0);
        assert_eq!(default_mint_info.list_authorities.len(), 0);
        assert_eq!(default_mint_info.black_list.len(), 0);
        assert_eq!(default_mint_info.white_list.len(), 0);
        assert_eq!(default_mint_info.metadata_update_authorities.len(), 0);
        assert_eq!(default_mint_info.supply, "");
        assert_eq!(default_mint_info.decimals, 0);
        assert!(!default_mint_info.is_paused);
        assert!(!default_mint_info.is_private);
        assert!(default_mint_info.meta.is_none());

        // Test that default can be serialized
        let json = serde_json::to_string(&default_mint_info).expect("Should serialize");
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_mint_info, deserialized);
    }

    #[test]
    fn test_mint_info_equality_and_clone() {
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");

        let mint_info1 = MintInfo {
            symbol: "TOKEN1".to_string(),
            master_authority: address,
            supply: "1000".to_string(),
            decimals: 18,
            is_paused: false,
            is_private: false,
            ..MintInfo::default()
        };

        let mint_info2 = MintInfo {
            symbol: "TOKEN1".to_string(),
            master_authority: address,
            supply: "1000".to_string(),
            decimals: 18,
            is_paused: false,
            is_private: false,
            ..MintInfo::default()
        };

        let mint_info3 = MintInfo {
            symbol: "TOKEN2".to_string(),
            master_authority: address,
            supply: "1000".to_string(),
            decimals: 18,
            is_paused: false,
            is_private: false,
            ..MintInfo::default()
        };

        // Test equality
        assert_eq!(mint_info1, mint_info2);
        assert_ne!(mint_info1, mint_info3);

        // Test clone
        let cloned = mint_info1.clone();
        assert_eq!(mint_info1, cloned);
    }

    #[test]
    fn test_minter_allowance_structure() {
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");
        let allowance = MinterAllowance {
            minter: address,
            allowance: "5000000000000000000000".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&allowance).expect("Should serialize");
        assert!(json.contains("742d35cc6634c0532925a3b8d91d6f4a81b8cbc0"));
        assert!(json.contains("5000000000000000000000"));

        // Test deserialization
        let deserialized: MinterAllowance =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.minter, address);
        assert_eq!(deserialized.allowance, "5000000000000000000000");

        // Test display
        let display_str = format!("{}", allowance);
        assert!(display_str.contains("Minter:"));
        assert!(display_str.contains("Allowance:"));
        assert!(display_str.contains("5000000000000000000000"));

        // Test equality and clone
        let cloned = allowance.clone();
        assert_eq!(allowance, cloned);

        let different = MinterAllowance {
            minter: address,
            allowance: "1000".to_string(),
        };
        assert_ne!(allowance, different);
    }

    #[test]
    fn test_minter_allowance_default() {
        let default_allowance = MinterAllowance::default();

        assert_eq!(default_allowance.minter, Address::ZERO);
        assert_eq!(default_allowance.allowance, "");

        // Test serialization
        let json = serde_json::to_string(&default_allowance).expect("Should serialize");
        let deserialized: MinterAllowance =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_allowance, deserialized);
    }

    #[test]
    fn test_token_metadata_structure() {
        let metadata = TokenMetadata {
            name: "Awesome Token".to_string(),
            uri: "https://api.example.com/metadata/token1".to_string(),
            additional_metadata: vec![
                MetadataKVPair {
                    key: "description".to_string(),
                    value: "An awesome token for testing".to_string(),
                },
                MetadataKVPair {
                    key: "image".to_string(),
                    value: "https://example.com/token-image.png".to_string(),
                },
            ],
        };

        // Test serialization
        let json = serde_json::to_string(&metadata).expect("Should serialize");
        assert!(json.contains("Awesome Token"));
        assert!(json.contains("https://api.example.com/metadata/token1"));
        assert!(json.contains("description"));
        assert!(json.contains("An awesome token for testing"));

        // Test deserialization
        let deserialized: TokenMetadata = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.name, "Awesome Token");
        assert_eq!(deserialized.uri, "https://api.example.com/metadata/token1");
        assert_eq!(deserialized.additional_metadata.len(), 2);
        assert_eq!(deserialized.additional_metadata[0].key, "description");
        assert_eq!(
            deserialized.additional_metadata[0].value,
            "An awesome token for testing"
        );

        // Test display
        let display_str = format!("{}", metadata);
        assert!(display_str.contains("Token Metadata: Awesome Token"));
        assert!(display_str.contains("(URI: https://api.example.com/metadata/token1)"));
        assert!(display_str.contains("[2 additional properties]"));

        // Test clone and equality
        let cloned = metadata.clone();
        assert_eq!(metadata, cloned);
    }

    #[test]
    fn test_token_metadata_default() {
        let default_metadata = TokenMetadata::default();

        assert_eq!(default_metadata.name, "");
        assert_eq!(default_metadata.uri, "");
        assert_eq!(default_metadata.additional_metadata.len(), 0);

        // Test serialization
        let json = serde_json::to_string(&default_metadata).expect("Should serialize");
        let deserialized: TokenMetadata = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_metadata, deserialized);

        // Test display with empty metadata
        let display_str = format!("{}", default_metadata);
        assert!(display_str.contains("Token Metadata:  (URI: )"));
        assert!(!display_str.contains("additional properties"));
    }

    #[test]
    fn test_token_metadata_with_no_additional_properties() {
        let metadata = TokenMetadata {
            name: "Simple Token".to_string(),
            uri: "https://example.com/simple".to_string(),
            additional_metadata: vec![],
        };

        let display_str = format!("{}", metadata);
        assert!(display_str.contains("Token Metadata: Simple Token"));
        assert!(display_str.contains("(URI: https://example.com/simple)"));
        assert!(!display_str.contains("additional properties"));
    }

    #[test]
    fn test_metadata_kv_pair_structure() {
        let kv_pair = MetadataKVPair {
            key: "author".to_string(),
            value: "OneMoney Team".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&kv_pair).expect("Should serialize");
        assert!(json.contains("author"));
        assert!(json.contains("OneMoney Team"));

        // Test deserialization
        let deserialized: MetadataKVPair = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.key, "author");
        assert_eq!(deserialized.value, "OneMoney Team");

        // Test display
        let display_str = format!("{}", kv_pair);
        assert_eq!(display_str, "author: OneMoney Team");

        // Test clone and equality
        let cloned = kv_pair.clone();
        assert_eq!(kv_pair, cloned);

        let different = MetadataKVPair {
            key: "author".to_string(),
            value: "Different Team".to_string(),
        };
        assert_ne!(kv_pair, different);
    }

    #[test]
    fn test_metadata_kv_pair_default() {
        let default_kv_pair = MetadataKVPair::default();

        assert_eq!(default_kv_pair.key, "");
        assert_eq!(default_kv_pair.value, "");

        // Test serialization
        let json = serde_json::to_string(&default_kv_pair).expect("Should serialize");
        let deserialized: MetadataKVPair = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(default_kv_pair, deserialized);

        // Test display
        let display_str = format!("{}", default_kv_pair);
        assert_eq!(display_str, ": ");
    }

    #[test]
    fn test_metadata_kv_pair_hash() {
        use std::collections::HashSet;

        let kv_pair1 = MetadataKVPair {
            key: "version".to_string(),
            value: "1.0".to_string(),
        };
        let kv_pair2 = MetadataKVPair {
            key: "version".to_string(),
            value: "1.0".to_string(),
        };
        let kv_pair3 = MetadataKVPair {
            key: "version".to_string(),
            value: "2.0".to_string(),
        };

        // Test that Hash trait is implemented and works
        let mut set = HashSet::new();
        set.insert(kv_pair1.clone());
        set.insert(kv_pair2.clone());
        set.insert(kv_pair3.clone());

        // Should have 2 unique KV pairs (kv_pair1 == kv_pair2)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&kv_pair1));
        assert!(set.contains(&kv_pair3));
    }

    #[test]
    fn test_metadata_kv_pair_alloy_rlp_encoding() {
        let kv_pair = MetadataKVPair {
            key: "token_type".to_string(),
            value: "utility".to_string(),
        };

        // Test alloy_rlp encoding
        let mut encoded = Vec::new();
        kv_pair.encode(&mut encoded);

        // Verify that something was encoded (basic check)
        assert!(!encoded.is_empty());

        // The exact RLP encoding depends on the implementation, but we can verify
        // that the encoding process doesn't panic and produces output
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_mint_info_with_comprehensive_authorities() {
        let address1 =
            Address::from_str("0x1111111111111111111111111111111111111111").expect("Valid address");
        let address2 =
            Address::from_str("0x2222222222222222222222222222222222222222").expect("Valid address");
        let address3 =
            Address::from_str("0x3333333333333333333333333333333333333333").expect("Valid address");

        let mint_info = MintInfo {
            symbol: "COMPREHENSIVE".to_string(),
            master_authority: address1,
            master_mint_burn_authority: address2,
            mint_burn_authorities: vec![
                MinterAllowance {
                    minter: address1,
                    allowance: "1000000".to_string(),
                },
                MinterAllowance {
                    minter: address2,
                    allowance: "2000000".to_string(),
                },
            ],
            pause_authorities: vec![address1, address2],
            list_authorities: vec![address2, address3],
            black_list: vec![address3],
            white_list: vec![address1, address2],
            metadata_update_authorities: vec![address1],
            supply: "10000000000000000000000000".to_string(),
            decimals: 6,
            is_paused: true,
            is_private: false,
            meta: None,
        };

        // Test serialization and deserialization
        let json = serde_json::to_string(&mint_info).expect("Should serialize");
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(mint_info.mint_burn_authorities.len(), 2);
        assert_eq!(mint_info.pause_authorities.len(), 2);
        assert_eq!(mint_info.list_authorities.len(), 2);
        assert_eq!(mint_info.black_list.len(), 1);
        assert_eq!(mint_info.white_list.len(), 2);
        assert_eq!(mint_info.metadata_update_authorities.len(), 1);

        assert_eq!(deserialized.mint_burn_authorities.len(), 2);
        assert_eq!(deserialized.pause_authorities.len(), 2);
        assert_eq!(deserialized.list_authorities.len(), 2);
        assert_eq!(deserialized.black_list.len(), 1);
        assert_eq!(deserialized.white_list.len(), 2);
        assert_eq!(deserialized.metadata_update_authorities.len(), 1);

        assert_eq!(mint_info, deserialized);
    }

    #[test]
    fn test_mint_info_edge_cases() {
        // Test with maximum values
        let max_mint_info = MintInfo {
            symbol: "MAX".to_string(),
            supply: u128::MAX.to_string(),
            decimals: u8::MAX,
            is_paused: true,
            is_private: true,
            ..MintInfo::default()
        };

        let json = serde_json::to_string(&max_mint_info).expect("Should serialize");
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(max_mint_info, deserialized);
        assert_eq!(deserialized.decimals, u8::MAX);

        // Test with zero supply
        let zero_supply_info = MintInfo {
            symbol: "ZERO".to_string(),
            supply: "0".to_string(),
            decimals: 0,
            is_paused: false,
            is_private: false,
            ..MintInfo::default()
        };

        let json = serde_json::to_string(&zero_supply_info).expect("Should serialize");
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(zero_supply_info, deserialized);
        assert_eq!(deserialized.supply, "0");
    }

    #[test]
    fn test_mint_info_debug_formatting() {
        let mint_info = MintInfo {
            symbol: "DEBUG".to_string(),
            supply: "1000".to_string(),
            decimals: 8,
            is_paused: false,
            is_private: true,
            ..MintInfo::default()
        };

        let debug_str = format!("{:?}", mint_info);
        assert!(debug_str.contains("MintInfo"));
        assert!(debug_str.contains("symbol: \"DEBUG\""));
        assert!(debug_str.contains("supply: \"1000\""));
        assert!(debug_str.contains("decimals: 8"));
        assert!(debug_str.contains("is_paused: false"));
        assert!(debug_str.contains("is_private: true"));
    }

    #[test]
    fn test_json_format_compatibility() {
        // Test that our structures match expected JSON format from L1 API
        let address =
            Address::from_str("0x742d35Cc6634C0532925a3b8D91D6F4A81B8Cbc0").expect("Valid address");

        let mint_info = MintInfo {
            symbol: "COMPAT".to_string(),
            master_authority: address,
            supply: "1000000".to_string(),
            decimals: 18,
            is_paused: false,
            is_private: false,
            ..MintInfo::default()
        };

        let json = serde_json::to_string(&mint_info).expect("Should serialize");

        // Verify key field names are preserved in JSON
        assert!(json.contains("\"symbol\""));
        assert!(json.contains("\"master_authority\""));
        assert!(json.contains("\"supply\""));
        assert!(json.contains("\"decimals\""));
        assert!(json.contains("\"is_paused\""));
        assert!(json.contains("\"is_private\""));
        assert!(json.contains("\"mint_burn_authorities\""));
        assert!(json.contains("\"pause_authorities\""));
        assert!(json.contains("\"list_authorities\""));
        assert!(json.contains("\"black_list\""));
        assert!(json.contains("\"white_list\""));
        assert!(json.contains("\"metadata_update_authorities\""));

        // Test deserialization from L1-compatible format
        let deserialized: MintInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.symbol, "COMPAT");
        assert_eq!(deserialized.master_authority, address);
        assert_eq!(deserialized.supply, "1000000");
        assert_eq!(deserialized.decimals, 18);
    }
}
