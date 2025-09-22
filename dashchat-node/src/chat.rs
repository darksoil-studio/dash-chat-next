use std::{convert::Infallible, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, derive_more::Deref)]
#[serde(into = "String", try_from = "String")]
pub struct ChatId([u8; 32]);

impl ChatId {
    pub fn new(topic_id: [u8; 32]) -> Self {
        Self(topic_id)
    }

    pub fn random() -> Self {
        Self(rand::random())
    }
}

impl From<ChatId> for String {
    fn from(chat_id: ChatId) -> Self {
        chat_id.to_string()
    }
}

impl TryFrom<String> for ChatId {
    type Error = Infallible;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(ChatId::from_str(&value).unwrap())
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl std::fmt::Debug for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut k = self.to_string();
        k.truncate(8);
        write!(f, "ChatId|{k}")
    }
}

impl FromStr for ChatId {
    type Err = anyhow::Error;

    fn from_str(topic: &str) -> Result<Self, Self::Err> {
        // maybe base64?
        Ok(Self(
            hex::decode(topic)?
                .try_into()
                .map_err(|e| anyhow::anyhow!("Invalid ChatId: {e:?}"))?,
        ))
    }
}
