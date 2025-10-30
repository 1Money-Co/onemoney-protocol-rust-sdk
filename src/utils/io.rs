use std::{
    io::{self, Read, Write},
    string::FromUtf8Error,
};

pub type Result<T> = core::result::Result<T, IOError>;

#[derive(Debug, thiserror::Error)]
pub enum IOError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Hex encoding/decoding error: {0}")]
    Hex(#[from] faster_hex::Error),

    #[error("K256 error: {0}")]
    K256(String),

    #[error("Alloy primitives error: {0}")]
    AlloyPrimitives(String),

    #[error("{0}")]
    Other(String),
}

impl From<&str> for IOError {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<k256::elliptic_curve::Error> for IOError {
    fn from(err: k256::elliptic_curve::Error) -> Self {
        Self::K256(err.to_string())
    }
}

impl From<alloy_primitives::SignatureError> for IOError {
    fn from(err: alloy_primitives::SignatureError) -> Self {
        Self::AlloyPrimitives(err.to_string())
    }
}

pub trait Import<const IS_HUMAN_READABLE: bool>: Sized {
    /// Import from a reader.
    fn import<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Export<const IS_HUMAN_READABLE: bool> {
    /// Export to a writer.
    fn export<W: Write>(&self, writer: &mut W) -> Result<()>;
}

/// Encode `T` to hex string with '0x' prefix.
pub fn prefix_hex_string<T: Export<true>>(src: &T) -> Result<String> {
    let mut buf = Vec::new();
    src.export(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

use faster_hex::{hex_decode, hex_encode};
const HEX_PREFIX: &[u8; 2] = b"0x";

fn hex_encode_with_prefix<W: Write>(bytes: &[u8], writer: &mut W) -> Result<()> {
    let mut buf = vec![0; bytes.len() * 2];
    hex_encode(bytes, &mut buf)?;
    writer.write_all(HEX_PREFIX)?;
    writer.write_all(&buf)?;
    Ok(())
}

fn hex_decode_with_prefix<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let hex_without_prefix = buf
        .strip_prefix(HEX_PREFIX)
        .ok_or("Must have '0x' prefix")?;
    let mut dst = vec![0; hex_without_prefix.len() / 2];
    hex_decode(hex_without_prefix, &mut dst)?;
    Ok(dst)
}

impl Export<true> for Box<[u8]> {
    fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        hex_encode_with_prefix(self, writer)
    }
}

impl Import<true> for Box<[u8]> {
    fn import<R: Read>(reader: &mut R) -> Result<Self> {
        hex_decode_with_prefix(reader).map(|bytes| bytes.into_boxed_slice())
    }
}

impl Export<true> for Vec<u8> {
    fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        hex_encode_with_prefix(self, writer)
    }
}

impl Import<true> for Vec<u8> {
    fn import<R: Read>(reader: &mut R) -> Result<Self> {
        hex_decode_with_prefix(reader)
    }
}

impl Export<true> for k256::PublicKey {
    fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.to_sec1_bytes().export(writer)
    }
}

impl Import<true> for k256::PublicKey {
    fn import<R: Read>(reader: &mut R) -> Result<Self> {
        let public_bytes = Vec::<u8>::import(reader)?;
        Ok(Self::from_sec1_bytes(&public_bytes)?)
    }
}

impl Export<true> for alloy_primitives::Signature {
    fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes: Vec<u8> = self.into();
        bytes.export(writer)
    }
}

impl Import<true> for alloy_primitives::Signature {
    fn import<R: Read>(reader: &mut R) -> Result<Self> {
        let secret_bytes = Vec::<u8>::import(reader)?;
        Ok(Self::try_from(secret_bytes.as_slice())?)
    }
}
