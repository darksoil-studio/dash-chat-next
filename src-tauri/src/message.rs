use std::str::FromStr;

use p2panda_core::{
    cbor::{decode_cbor, encode_cbor, DecodeError},
    PublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub content: String,
    // #[serde(with = "public_key_serde")]
    pub author: PublicKey, // Current user's key
    pub timestamp: u64,
}

impl ChatMessage {
    pub fn as_bytes(&self) -> Vec<u8> {
        encode_cbor(&self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        decode_cbor(bytes)
    }
}

mod public_key_serde {
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&key.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(PublicKey::from_str(&s).map_err(serde::de::Error::custom)?)
    }
}
