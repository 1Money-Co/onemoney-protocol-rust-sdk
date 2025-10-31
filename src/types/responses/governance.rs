//! Governance and epoch-related API response types.
//!
//! Certificates may be returned either as structured JSON or as hex-encoded BCS.
//! The SDK keeps the representation generic and lets consumers decide how to
//! decode the payload.

use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Certificate payload returned by the governance epoch API.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CertificateData {
    /// Hex-encoded BCS certificate payload (prefixed with `0x`).
    Bcs { certificate: String },
    /// JSON-encoded certificate payload.
    Json { certificate: Value },
}

impl CertificateData {
    /// Returns the certificate as JSON when available.
    pub fn as_json(&self) -> Option<&Value> {
        match self {
            CertificateData::Json { certificate } => Some(certificate),
            CertificateData::Bcs { .. } => None,
        }
    }

    /// Returns the certificate hex string for BCS-encoded payloads.
    pub fn as_bcs_hex(&self) -> Option<&str> {
        match self {
            CertificateData::Json { .. } => None,
            CertificateData::Bcs { certificate } => Some(certificate),
        }
    }
}

impl Display for CertificateData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CertificateData::Bcs { certificate } => {
                write!(f, "Certificate (BCS hex): {}", certificate)
            }
            CertificateData::Json { certificate } => {
                write!(f, "Certificate (JSON): {}", certificate)
            }
        }
    }
}

/// Epoch information returned by the governance endpoints.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochResponse {
    /// Epoch identifier.
    pub epoch_id: u64,
    /// Certificate hash.
    pub certificate_hash: B256,
    /// Certificate payload in either JSON or BCS representation.
    #[serde(flatten)]
    pub certificate_data: CertificateData,
}

impl EpochResponse {
    /// Returns the certificate as JSON when available.
    pub fn certificate_json(&self) -> Option<&Value> {
        self.certificate_data.as_json()
    }

    /// Returns the certificate as a BCS hex string when available.
    pub fn certificate_bcs_hex(&self) -> Option<&str> {
        self.certificate_data.as_bcs_hex()
    }
}

impl Display for EpochResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Epoch {}:", self.epoch_id)?;
        writeln!(f, "  Certificate Hash: {}", self.certificate_hash)?;
        write!(f, "  {}", self.certificate_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_certificate_helpers() {
        let certificate_value = json!({
            "type": "Epoch",
            "proposal": { "epoch": 42 }
        });
        let response = EpochResponse {
            epoch_id: 42,
            certificate_hash: B256::from([1u8; 32]),
            certificate_data: CertificateData::Json {
                certificate: certificate_value.clone(),
            },
        };

        assert_eq!(response.certificate_json(), Some(&certificate_value));
        assert!(response.certificate_bcs_hex().is_none());
    }

    #[test]
    fn test_bcs_certificate_helpers() {
        let response = EpochResponse {
            epoch_id: 7,
            certificate_hash: B256::from([2u8; 32]),
            certificate_data: CertificateData::Bcs {
                certificate: "0xdeadbeef".to_string(),
            },
        };

        assert_eq!(response.certificate_bcs_hex(), Some("0xdeadbeef"));
        assert!(response.certificate_json().is_none());
    }

    #[test]
    fn test_deserialize_json_certificate_payload() {
        let json_payload = r#"{
            "epoch_id": 99,
            "certificate_hash": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "certificate": {
                "type": "Epoch",
                "proposal": {
                    "epoch": 99
                }
            }
        }"#;

        let response: EpochResponse =
            serde_json::from_str(json_payload).expect("EpochResponse should deserialize from JSON");

        assert_eq!(response.epoch_id, 99);
        assert!(response.certificate_json().is_some());
        assert!(response.certificate_bcs_hex().is_none());
    }

    #[test]
    fn test_deserialize_bcs_certificate_payload() {
        let json_payload = r#"{
            "epoch_id": 45,
            "certificate_hash": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "certificate": "0xfeedface"
        }"#;

        let response: EpochResponse =
            serde_json::from_str(json_payload).expect("EpochResponse should deserialize from JSON");

        assert_eq!(response.epoch_id, 45);
        assert_eq!(response.certificate_bcs_hex(), Some("0xfeedface"));
        assert!(response.certificate_json().is_none());
    }

    #[test]
    fn test_serialization_roundtrip_json() {
        let original = EpochResponse {
            epoch_id: 9,
            certificate_hash: B256::from([3u8; 32]),
            certificate_data: CertificateData::Json {
                certificate: json!({ "type": "Epoch", "proposal": { "epoch": 9 } }),
            },
        };

        let json = serde_json::to_string(&original).expect("serialize to json");
        let reconstructed: EpochResponse =
            serde_json::from_str(&json).expect("deserialize from json");

        assert_eq!(reconstructed.epoch_id, original.epoch_id);
        assert_eq!(reconstructed.certificate_hash, original.certificate_hash);
        assert_eq!(
            reconstructed.certificate_json(),
            original.certificate_json()
        );
    }

    #[test]
    fn test_serialization_roundtrip_bcs() {
        let original = EpochResponse {
            epoch_id: 21,
            certificate_hash: B256::from([4u8; 32]),
            certificate_data: CertificateData::Bcs {
                certificate: "0x0011".to_string(),
            },
        };

        let json = serde_json::to_string(&original).expect("serialize to json");
        let reconstructed: EpochResponse =
            serde_json::from_str(&json).expect("deserialize from json");

        assert_eq!(reconstructed.certificate_bcs_hex(), Some("0x0011"));
        assert_eq!(reconstructed.certificate_json(), None);
    }

    #[test]
    fn test_display_formats() {
        let json_variant = CertificateData::Json {
            certificate: json!({"type": "Epoch"}),
        };
        let bcs_variant = CertificateData::Bcs {
            certificate: "0xaaaa".to_string(),
        };

        assert!(format!("{}", json_variant).contains("Certificate (JSON)"));
        assert!(format!("{}", bcs_variant).contains("Certificate (BCS hex)"));

        let response = EpochResponse {
            epoch_id: 5,
            certificate_hash: B256::from([7u8; 32]),
            certificate_data: json_variant,
        };

        let display_output = format!("{}", response);
        assert!(display_output.contains("Epoch 5"));
        assert!(display_output.contains("Certificate Hash"));
    }
}
