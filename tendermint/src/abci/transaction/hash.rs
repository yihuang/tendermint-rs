//! Transaction hashes

use crate::error::{Error, ErrorKind};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};
use subtle::{self, ConstantTimeEq};
use subtle_encoding::hex;

/// Size of a transaction hash in bytes
pub const LENGTH: usize = 32;

/// Trannsaction hashes
#[derive(Copy, Clone, Hash)]
pub struct Hash([u8; LENGTH]);

impl Hash {
    /// Create a new transaction hash from raw bytes
    pub fn new(bytes: [u8; LENGTH]) -> Hash {
        Hash(bytes)
    }

    /// Borrow the transaction hash as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl Default for Hash {
    fn default() -> Hash {
        // hash of empty data
        let mut bytes = [0u8; LENGTH];
        bytes.copy_from_slice(&Sha256::digest(&[])[..LENGTH]);
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ConstantTimeEq for Hash {
    #[inline]
    fn ct_eq(&self, other: &Hash) -> subtle::Choice {
        self.as_bytes().ct_eq(other.as_bytes())
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transaction::Hash({})", self)
    }
}

/// Decode transaction hash from hex
impl FromStr for Hash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept either upper or lower case hex
        let bytes = hex::decode_upper(s)
            .or_else(|_| hex::decode(s))
            .map_err(|_| ErrorKind::Parse)?;

        if bytes.len() != LENGTH {
            return Err(ErrorKind::Parse.into());
        }

        let mut result_bytes = [0u8; LENGTH];
        result_bytes.copy_from_slice(&bytes);
        Ok(Hash(result_bytes))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            de::Error::custom(format!(
                "expected {}-character hex string, got {:?}",
                LENGTH * 2,
                s
            ))
        })
    }
}

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
