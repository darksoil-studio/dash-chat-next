use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;

use async_trait::async_trait;
use p2panda_sync::log_sync::TopicLogMap;

use crate::chat::ChatId;
use p2panda_core::PublicKey;
use p2panda_net::TopicId;
use p2panda_sync::TopicQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, derive_more::From)]
pub enum Topic {
    Chat(ChatId),
    Inbox(PublicKey),
}

impl TopicQuery for Topic {}

impl TopicId for Topic {
    fn id(&self) -> [u8; 32] {
        match self {
            Topic::Chat(chat_id) => **chat_id,
            Topic::Inbox(public_key) => *public_key.as_bytes(),
        }
    }
}

pub type LogId = (Topic, PublicKey);

#[derive(Clone, Debug)]
pub struct AuthorStore(Arc<RwLock<HashMap<Topic, HashSet<PublicKey>>>>);

impl AuthorStore {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn add_author(&mut self, chat: Topic, public_key: PublicKey) {
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

    pub async fn authors(&self, chat: &Topic) -> Option<HashSet<PublicKey>> {
        let authors = self.0.read().await;
        authors.get(chat).cloned()
    }
}

#[async_trait]
impl TopicLogMap<Topic, LogId> for AuthorStore {
    /// During sync other peers are interested in all our append-only logs for a certain topic.
    /// This method tells the sync protocol which logs we have available from which author for that
    /// given topic.
    async fn get(&self, chat: &Topic) -> Option<HashMap<PublicKey, Vec<LogId>>> {
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
