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
    /// JSON-encoded certificate payload.
    Json { certificate: Value },
    /// Hex-encoded BCS certificate payload (prefixed with `0x`).
    Bcs { certificate: String },
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
            CertificateData::Json { certificate } => {
                write!(f, "Certificate (JSON): {}", certificate)
            }
            CertificateData::Bcs { certificate } => {
                write!(f, "Certificate (BCS hex): {}", certificate)
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
}
