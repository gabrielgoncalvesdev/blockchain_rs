use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {

    pub const ZERO: Hash = Hash([0u8; 32]);

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Hash(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn has_leading_zeros(&self, count: usize) -> bool {
        let full_bytes = count / 2;
        if self.0[..full_bytes].iter().any(|&b| b != 0){
            return false;
        }
        if count % 2 == 1 {
            return self.0[full_bytes] >> 4 == 0;
        }
        true
    }

    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out  = String::with_capacity(64);
        for &byte in &self.0 {
            out.push(HEX[(byte >> 4 ) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    pub fn from_hex(text: &str) -> Result<Self, HashParseError> {
        let raw = text.as_bytes();
        if raw.len() != 64 {
            return Err(HashParseError::Length(raw.len()));
        }
        let mut bytes =  [0u8; 32];
        for (i, slot) in bytes.iter_mut().enumerate() {
            let high = hex_digit(raw[i * 2])?;
            let low = hex_digit(raw[i * 2 + 1])?;
            *slot = (high << 4) | low;
        }
        Ok(Hash(bytes))
    }
}

        fn hex_digit(c: u8) -> Result<u8, HashParseError> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(HashParseError::InvalidChar(c as char)),
        }
    }

        impl fmt::Display for Hash {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.to_hex())
        }
    }

        impl Serialize for Hash {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(&self.to_hex())
        }
    }

        impl<'de> Deserialize<'de> for Hash {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let text = String::deserialize(d)?;
            Hash::from_hex(&text).map_err(serde::de::Error::custom)
        }
    }

    #[derive(Debug, Error, PartialEq)]
    pub enum HashParseError {
        #[error("hash must have 64 char hex, received {0}")]
        Length(usize),
        #[error("hex char invalid: {0:?}")]
        InvalidChar(char),
    }    