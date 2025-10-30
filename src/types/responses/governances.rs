use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use alloy_primitives::{Address, B256};
use k256::PublicKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use serde::{Deserialize, Serialize};

use crate::ChainId;
use crate::hex_serde;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpochResponse {
    pub epoch_id: u64,
    pub certificate_hash: B256,
    pub certificate: Certificate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Certificate {
    Genesis { proposal: GenesisProposal },
    Epoch { proposal: GovernanceProposal },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenesisProposal {
    pub message: GovernanceMessage,
    #[serde(with = "hex_serde")]
    pub signature: alloy_primitives::Signature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub message: GovernanceMessage,
    #[serde(with = "hex_serde")]
    pub operator_signature: alloy_primitives::Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceMessage {
    pub epoch: Epoch,
    pub chain: ChainId,

    /// Public key of the network operator, used for signature verification
    #[serde(with = "hex_serde")]
    pub operator_public_key: PublicKey,
    /// The account address for this epoch’s mint account, also known as the
    /// operator address
    pub operator_address: Address,

    pub validator_set: ValidatorSet,
    // protocol_parameters: ProtocolParameters,
    pub timestamp: u64,
    pub version: u64,
}

/// `EpochId` is the ID of an epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Epoch {
    pub epoch_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorSet {
    /// Ordered set of unique validator identities
    pub members: BTreeSet<ValidatorIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ValidatorIdentity {
    /// Public key used for consensus operations and signing
    pub consensus_public_key: String,

    /// Ethereum address derived from the consensus public key
    pub address: Address,

    /// Peer ID derived from the network public key for libp2p networking
    pub peer_id: String,

    /// Whether the validator self declares to be an archive node
    #[serde(default)]
    pub archive: bool,
}

impl Display for EpochResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Epoch Response:")?;
        writeln!(f, "  Epoch ID: {}", self.epoch_id)?;
        writeln!(f, "  Certificate Hash: {}", self.certificate_hash)?;
        match &self.certificate {
            Certificate::Genesis { proposal } => {
                writeln!(f, "  Certificate Type: Genesis")?;
                write!(f, "{}", proposal)?;
            }
            Certificate::Epoch { proposal } => {
                writeln!(f, "  Certificate Type: Epoch")?;
                write!(f, "{}", proposal)?;
            }
        }
        Ok(())
    }
}

impl Display for GenesisProposal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "  Genesis Proposal:")?;
        write!(f, "{}", self.message)?;
        writeln!(f, "    Signature: {}", self.signature)?;
        Ok(())
    }
}

impl Display for GovernanceProposal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "  Governance Proposal:")?;
        write!(f, "{}", self.message)?;
        writeln!(f, "    Operator Signature: {}", self.operator_signature)?;
        Ok(())
    }
}

impl Display for GovernanceMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "    Epoch: {}", self.epoch.epoch_id)?;
        writeln!(f, "    Chain ID: {}", self.chain)?;
        writeln!(f, "    Operator Address: {}", self.operator_address)?;
        writeln!(
            f,
            "    Operator Public Key: 0x{}",
            hex::encode(self.operator_public_key.to_encoded_point(false).as_bytes())
        )?;
        writeln!(f, "    Timestamp: {}", self.timestamp)?;
        writeln!(f, "    Version: {}", self.version)?;
        writeln!(f, "    Validator Set:")?;
        writeln!(
            f,
            "      Total Validators: {}",
            self.validator_set.members.len()
        )?;
        for (idx, validator) in self.validator_set.members.iter().enumerate() {
            writeln!(f, "      Validator {}:", idx + 1)?;
            writeln!(f, "        Address: {}", validator.address)?;
            writeln!(
                f,
                "        Consensus Public Key: {}",
                validator.consensus_public_key
            )?;
            writeln!(f, "        Peer ID: {}", validator.peer_id)?;
            writeln!(f, "        Archive Node: {}", validator.archive)?;
        }
        Ok(())
    }
}
