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

pub type LogId = (ChatId, PublicKey);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChatId([u8; 32]);

impl ChatId {
    pub fn new(topic_id: [u8; 32]) -> Self {
        Self(topic_id)
    }

    pub fn random() -> Self {
        Self(rand::random())
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for ChatId {
    type Err = Infallible;

    fn from_str(topic: &str) -> Result<Self, Self::Err> {
        // maybe base64?
        Ok(Self(Hash::new(topic.as_bytes()).into()))
    }
}

impl TopicQuery for ChatId {}

impl TopicId for ChatId {
    fn id(&self) -> [u8; 32] {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct AuthorStore(Arc<RwLock<HashMap<ChatId, HashSet<PublicKey>>>>);

impl AuthorStore {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn add_author(&mut self, chat: ChatId, public_key: PublicKey) {
        let mut authors = self.0.write().await;
        authors
            .entry(chat)
            .and_modify(|public_keys| {
                public_keys.insert(public_key);
            })
            .or_insert({
                let mut public_keys = HashSet::new();
                public_keys.insert(public_key);
                public_keys
            });
    }

    pub async fn authors(&self, chat: &ChatId) -> Option<HashSet<PublicKey>> {
        let authors = self.0.read().await;
        authors.get(chat).cloned()
    }
}

#[async_trait]
impl TopicLogMap<ChatId, LogId> for AuthorStore {
    /// During sync other peers are interested in all our append-only logs for a certain topic.
    /// This method tells the sync protocol which logs we have available from which author for that
    /// given topic.
    async fn get(&self, chat: &ChatId) -> Option<HashMap<PublicKey, Vec<LogId>>> {
        let authors = self.authors(chat).await;
        let map = match authors {
            Some(authors) => {
                let mut map = HashMap::with_capacity(authors.len());
                for author in authors {
                    // We write all data of one author into one log for now.
                    map.insert(author, vec![(chat.clone(), author)]);
                }
                map
            }
            None => HashMap::new(),
        };
        Some(map)
    }
}
