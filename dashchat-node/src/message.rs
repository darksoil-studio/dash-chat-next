use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::PK;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub content: String,
    // #[serde(with = "public_key_serde")]
    pub author: PK, // Current user's key
    pub timestamp: u64,
}

mod public_key_serde {
    use p2panda_core::PublicKey;
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
