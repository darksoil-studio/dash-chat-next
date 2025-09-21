use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
    sync::Arc,
};

use async_trait::async_trait;
use p2panda_core::{Hash, PublicKey};
use p2panda_net::TopicId;
use p2panda_sync::{log_sync::TopicLogMap, TopicQuery};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::Deref)]
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
