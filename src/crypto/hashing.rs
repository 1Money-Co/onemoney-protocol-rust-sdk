//! Hashing utilities and traits.

use alloy_primitives::B256;

/// Trait for types that can be cryptographically signed.
pub trait Signable {
    /// Calculate the signature hash for this payload.
    fn signature_hash(&self) -> B256;
}
