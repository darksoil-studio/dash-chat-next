mod author_operation;
mod stream_processing;

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{anyhow, Context, Result};
use p2panda_auth::Access;
use p2panda_core::{Body, Hash, Header, PrivateKey, PruneFlag, PublicKey};
use p2panda_discovery::mdns::LocalDiscovery;
use p2panda_discovery::Discovery;
use p2panda_encryption::Rng;
use p2panda_net::config::GossipConfig;
use p2panda_net::{FromNetwork, Network, NetworkBuilder, SyncConfiguration, ToNetwork, TopicId};
use p2panda_store::{LogStore, MemoryStore, OperationStore};
use p2panda_stream::operation::{ingest_operation, IngestResult};
use p2panda_stream::{DecodeExt, IngestExt};
use p2panda_sync::log_sync::LogSyncProtocol;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, RwLock};
use tokio::task;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tracing::{debug, error, warn};

use crate::chat::{AuthorStore, ChatId, LogId};
use crate::forge::DashForge;
use crate::operation::{decode_gossip_message, encode_gossip_message, Extensions, Payload};
use crate::spaces::{DashGroup, DashManager, DashSpace};

// const RELAY_ENDPOINT: &str = "https://wasser.liebechaos.org";

const NETWORK_ID: [u8; 32] = [88; 32];

const DEFAULT_TOPIC: &str = "peers-for-peers";

const MAX_MESSAGE_SIZE: usize = 1000 * 10; // 10kb max. UDP payload size

#[derive(Clone, Debug)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    op_store: MemoryStore<LogId, Extensions>,
    network: Network<ChatId>,
    chats: Arc<RwLock<HashMap<ChatId, ChatNetwork>>>,
    author_store: AuthorStore,
    config: Config,
    private_key: PrivateKey,
}

#[derive(Clone, Debug)]
pub struct ChatNetwork {
    pub(crate) sender: mpsc::Sender<ToNetwork>,
    pub(crate) manager: DashManager,
}

impl Node {
    pub async fn new(private_key: PrivateKey, config: Config) -> Result<Self> {
        let mdns = LocalDiscovery::new();

        let op_store = MemoryStore::<LogId, Extensions>::new();
        let author_store = AuthorStore::new();

        // let relay_url = RELAY_ENDPOINT.parse()?;

        let mut network_builder = NetworkBuilder::new(NETWORK_ID)
            .private_key(private_key.clone())
            .discovery(mdns)
            .gossip(GossipConfig {
                max_message_size: MAX_MESSAGE_SIZE,
            });
        // .relay(relay_url, false, 0);

        let sync_protocol = LogSyncProtocol::new(author_store.clone(), op_store.clone());
        let sync_config = SyncConfiguration::new(sync_protocol);
        network_builder = network_builder.sync(sync_config);

        // if config.bootstrap {
        //     network_builder = network_builder.bootstrap();
        // }

        // if let Some(bootstrap) = config.use_bootstrap {
        //     network_builder = network_builder.direct_address(bootstrap, vec![], None);
        // }

        let network = network_builder.build().await.context("spawn p2p network")?;
        let chats = Arc::new(RwLock::new(HashMap::new()));

        // TODO: locally store list of groups and initialize them when the node starts

        Ok(Self {
            op_store,
            author_store,
            network,
            chats,
            config,
            private_key,
        })
    }

    /// Create a new chat Space, and subscribe to the Topic for this chat.
    pub async fn create_chat(&self) -> anyhow::Result<(ChatId, ChatNetwork)> {
        let chat_id = ChatId::random();
        let chat = self.initialize_chat(chat_id).await?;

        let (_, msgs) = chat
            .manager
            .create_space(
                chat_id,
                &[(self.private_key.public_key().into(), Access::manage())],
            )
            .await?;

        for msg in msgs {
            self.author_operation(&chat_id, Payload::Control(msg))
                .await?;
        }

        Ok((chat_id, chat))
    }

    pub async fn add_member(&self, chat_id: ChatId, public_key: PublicKey) -> anyhow::Result<()> {
        let chat = self
            .chats
            .read()
            .await
            .get(&chat_id)
            .cloned()
            .ok_or(anyhow!("Chat not found"))?;
        chat.manager
            .space(chat_id)
            .await?
            .unwrap()
            // TODO: we need an access level for only adding but not removing members
            .add(public_key.into(), Access::manage())
            .await?;

        Ok(())
    }

    /// "Joining" a chat means subscribing to messages for that chat.
    /// This needs to be accompanied by being added as a member of the chat Space by an existing member
    /// -- you're not fully a member until someone adds you.
    pub async fn join_chat(&self, chat_id: ChatId) -> anyhow::Result<ChatNetwork> {
        let chat = self.initialize_chat(chat_id).await?;

        Ok(chat)
    }

    pub async fn get_messages(&self, chat_id: ChatId) -> anyhow::Result<Vec<String>> {
        let Some(authors) = self.author_store.authors(&chat_id).await else {
            return Ok(vec![]);
        };

        let mut messages = vec![];

        for author in authors {
            let mut m = self.get_messages_from(chat_id.clone(), author).await?;
            messages.append(&mut m);
        }

        Ok(messages)
    }

    pub async fn get_messages_from(
        &self,
        chat_id: ChatId,
        public_key: PublicKey,
    ) -> anyhow::Result<Vec<String>> {
        let log_id = (chat_id, public_key);
        let log = self.op_store.get_log(&public_key, &log_id, None).await?;

        let Some(log) = log else {
            return Ok(vec![]);
        };

        let messages = log
            .into_iter()
            .map(|(_h, body)| {
                let Some(body) = body else {
                    return Err(anyhow!("No body?"));
                };

                Ok(String::from_utf8(body.to_bytes())?)
            })
            .collect::<anyhow::Result<Vec<String>>>()?;

        Ok(messages)
    }

    pub async fn send_message(&self, chat_id: ChatId, message: String) -> anyhow::Result<()> {
        let body = Body::new(message.as_bytes());
        let public_key = self.private_key.public_key();

        let Some(chat_network) = self.chats.read().await.get(&chat_id).cloned() else {
            return Err(anyhow!("Chat not found"));
        };

        let log_id = (chat_id, public_key);

        // TODO: this is not atomic, see https://github.com/p2panda/p2panda/issues/798
        let latest_operation = self.op_store.latest_operation(&public_key, &log_id).await?;

        let (seq_num, backlink) = match latest_operation {
            Some((header, _)) => (header.seq_num + 1, Some(header.hash())),
            None => (0, None),
        };

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time from operation system")
            .as_secs();

        let extensions = Extensions {
            log_id: log_id.clone(),
            control_message: None,
        };

        let mut header = Header {
            version: 1,
            public_key,
            signature: None,
            payload_size: body.size(),
            payload_hash: Some(body.hash()),
            timestamp,
            seq_num,
            backlink,
            previous: vec![],
            extensions: Some(extensions),
        };
        header.sign(&self.private_key);

        p2panda_stream::operation::ingest_operation(
            &mut self.op_store.clone(),
            header.clone(),
            Some(body.clone()),
            header.to_bytes(),
            &log_id,
            false,
        )
        .await?;

        chat_network
            .sender
            .send(ToNetwork::Message {
                bytes: encode_gossip_message(&header, Some(&body))?,
            })
            .await?;

        Ok(())
    }
}
