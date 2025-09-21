mod author_operation;
mod stream_processing;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{anyhow, Context, Result};
use p2panda_auth::Access;
use p2panda_core::cbor::{decode_cbor, encode_cbor, DecodeError};
use p2panda_core::{Body, Header, Operation, PrivateKey, PublicKey};
use p2panda_discovery::mdns::LocalDiscovery;
use p2panda_discovery::Discovery;
use p2panda_encryption::Rng;
use p2panda_net::config::GossipConfig;
use p2panda_net::{
    FromNetwork, Network, NetworkBuilder, ResyncConfiguration, SyncConfiguration, ToNetwork,
};
use p2panda_spaces::member::Member;
use p2panda_store::{LogStore, MemoryStore};
use p2panda_stream::{DecodeExt, IngestExt};
use p2panda_sync::log_sync::LogSyncProtocol;
use tokio::sync::{mpsc, RwLock};
use tokio::task;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tracing::{debug, warn};

use crate::chat::ChatId;
use crate::forge::DashForge;
use crate::friend::Friend;
use crate::message::ChatMessage;
use crate::network::{AuthorStore, LogId, Topic};
use crate::node::author_operation::create_operation;
use crate::operation::{
    decode_gossip_message, encode_gossip_message, Extensions, HeaderData, InvitationMessage,
    Payload,
};
use crate::spaces::{DashManager, SpacesStore};

const RELAY_ENDPOINT: &str = "https://wasser.liebechaos.org";

const NETWORK_ID: [u8; 32] = [88; 32];

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
    network: Network<Topic>,
    chats: Arc<RwLock<HashMap<ChatId, ChatNetwork>>>,
    author_store: AuthorStore<Topic>,
    spaces_store: SpacesStore,
    manager: DashManager,
    config: Config,
    private_key: PrivateKey,
    friends: Arc<RwLock<HashMap<PublicKey, Friend>>>,
}

#[derive(Clone, Debug)]
pub struct ChatNetwork {
    pub(crate) sender: mpsc::Sender<ToNetwork>,
}

impl Node {
    pub async fn new(private_key: PrivateKey, config: Config) -> Result<Self> {
        let public_key = private_key.public_key();

        let mdns = LocalDiscovery::new();

        let op_store = MemoryStore::<LogId, Extensions>::new();
        let mut author_store = AuthorStore::new();

        {
            let op_store = op_store.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    for (pk, seq) in op_store
                        .get_log_heights(&Topic::Inbox(public_key.clone()))
                        .await
                        .unwrap_or_default()
                    {
                        println!("*** log height: {pk} -> {seq}");
                    }
                }
            });
        }

        author_store
            .add_author(Topic::Inbox(public_key.clone()), public_key)
            .await;

        let relay_url = RELAY_ENDPOINT.parse()?;

        let sync_protocol = LogSyncProtocol::new(author_store.clone(), op_store.clone());
        let sync_config = SyncConfiguration::new(sync_protocol)
            .resync(ResyncConfiguration::new().interval(10).poll_interval(1));

        let mut new_peers = mdns.subscribe(NETWORK_ID).unwrap();

        {
            let mut author_store = author_store.clone();
            tokio::spawn(async move {
                while let Some(Ok(peer)) = new_peers.next().await {
                    let pubkey = PublicKey::from_bytes(peer.node_addr.node_id.as_bytes()).unwrap();
                    if author_store
                        .authors(&Topic::Inbox(pubkey.clone()))
                        .await
                        .map(|authors| !authors.contains(&pubkey))
                        .unwrap_or(true)
                    {
                        println!("*** new peer: {}", pubkey);
                        author_store
                            .add_author(Topic::Inbox(pubkey.clone()), pubkey)
                            .await;
                    }
                }
                println!("*** new peers stream ended");
            });
        }

        let network_builder = NetworkBuilder::new(NETWORK_ID)
            .private_key(private_key.clone())
            .discovery(mdns)
            .gossip(GossipConfig {
                max_message_size: MAX_MESSAGE_SIZE,
            })
            .relay(relay_url, false, 0)
            .sync(sync_config);

        // if config.bootstrap {
        //     network_builder = network_builder.bootstrap();
        // }

        // if let Some(bootstrap) = config.use_bootstrap {
        //     network_builder = network_builder.direct_address(bootstrap, vec![], None);
        // }

        let network = network_builder.build().await.context("spawn p2p network")?;
        let chats = Arc::new(RwLock::new(HashMap::new()));

        // TODO: we probably shouldn't be storing the store across spaces, right?
        //       this is only for ease of testing
        let spaces_store: SpacesStore =
            crate::spaces::create_test_store(private_key.clone()).into();

        let rng = Rng::default();

        let forge = DashForge {
            private_key: private_key.clone(),
        };

        let manager = DashManager::new(spaces_store.clone(), forge, rng).unwrap();

        let node = Self {
            op_store,
            author_store,
            spaces_store,
            network,
            chats,
            manager,
            config,
            private_key,
            friends: Arc::new(RwLock::new(HashMap::new())),
        };

        node.initialize_inbox(public_key).await?;

        // TODO: remove, this is jsut a test
        node.author_operation(
            public_key.into(),
            Payload::Invitation(InvitationMessage::Test),
        )
        .await?;

        // TODO: locally store list of groups and initialize them when the node starts

        Ok(node)
    }

    pub async fn me(&self) -> anyhow::Result<p2panda_spaces::member::Member> {
        let long_term_key_bundle = self.spaces_store.long_term_key_bundle().await?;
        Ok(p2panda_spaces::member::Member::new(
            self.private_key.public_key().into(),
            long_term_key_bundle,
        ))
    }

    /// Create a new chat Space, and subscribe to the Topic for this chat.
    pub async fn create_group(&self) -> anyhow::Result<(ChatId, ChatNetwork)> {
        let chat_id = ChatId::random();
        let chat = self.initialize_group(chat_id).await?;

        let (_, msgs) = self
            .manager
            .create_space(
                chat_id,
                &[(self.private_key.public_key().into(), Access::manage())],
            )
            .await
            .expect("TODO ?");

        for msg in msgs {
            self.author_operation(chat_id.into(), Payload::SpaceControl(msg))
                .await?;
        }

        Ok((chat_id, chat))
    }

    pub async fn get_groups(&self) -> anyhow::Result<Vec<ChatId>> {
        let groups = self.chats.read().await.keys().cloned().collect();
        Ok(groups)
    }

    pub async fn add_member(&self, chat_id: ChatId, pubkey: PublicKey) -> anyhow::Result<()> {
        let ms = self
            .manager
            .space(chat_id)
            .await
            .expect("TODO ?")
            .unwrap()
            // TODO: we need an access level for only adding but not removing members
            .add(pubkey.into(), Access::manage())
            .await
            .expect("TODO ?");

        self.author_operation(
            pubkey.into(),
            Payload::Invitation(InvitationMessage::JoinGroup(chat_id)),
        )
        .await?;

        for msg in ms {
            self.author_operation(chat_id.into(), Payload::SpaceControl(msg))
                .await?;
        }

        Ok(())
    }

    pub async fn get_members(
        &self,
        chat_id: ChatId,
    ) -> anyhow::Result<Vec<(p2panda_spaces::ActorId, Access)>> {
        let members = self
            .manager
            .space(chat_id)
            .await
            .expect("TODO ?")
            .unwrap()
            .members()
            .await
            .expect("TODO ?");

        Ok(members)
    }

    /// "Joining" a chat means subscribing to messages for that chat.
    /// This needs to be accompanied by being added as a member of the chat Space by an existing member
    /// -- you're not fully a member until someone adds you.
    pub async fn join_group(&self, chat_id: ChatId) -> anyhow::Result<ChatNetwork> {
        let chat = self.initialize_group(chat_id).await?;

        Ok(chat)
    }

    pub async fn get_messages(&self, chat_id: ChatId) -> anyhow::Result<Vec<ChatMessage>> {
        let Some(authors) = self.author_store.authors(&chat_id.into()).await else {
            println!("*** get_messages: no authors found for {chat_id}");
            return Ok(vec![]);
        };

        println!(
            "*** get_messages: authors found for {chat_id}: {:?}",
            authors
        );

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
    ) -> anyhow::Result<Vec<ChatMessage>> {
        let log_id = chat_id.into();
        let log = self.op_store.get_log(&public_key, &log_id, None).await?;

        let Some(log) = log else {
            println!("*** get_messages_from: no log found for {public_key} in {chat_id}");
            return Ok(vec![]);
        };

        println!("*** get_messages_from num messages: {}", log.len());

        let messages = log
            .into_iter()
            .filter_map(|(_h, body)| {
                if let Some(body) = body {
                    Some(ChatMessage::from_bytes(body.to_bytes().as_slice()))
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>, DecodeError>>()?;

        Ok(messages)
    }

    pub async fn send_message(&self, chat_id: ChatId, message: ChatMessage) -> anyhow::Result<()> {
        let topic = chat_id.into();

        self.author_operation(topic, Payload::ChatMessage(message.clone()))
            .await?;

        Ok(())
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    // Friend management methods
    pub async fn add_friend(&self, member: Member) -> anyhow::Result<PublicKey> {
        println!("*** adding friend: {:?}", member);
        let public_key = PublicKey::try_from(member.id()).expect("actor id is public key");

        // Register the member in the spaces manager
        self.manager
            .register_member(&member)
            .await
            .map_err(|e| anyhow!("Failed to register friend: {e:?}"))?;

        let network_tx = self
            .initialize_inbox(PublicKey::try_from(member.id()).expect("actor id is public key"))
            .await?;

        // Store the friend
        self.friends.write().await.insert(
            public_key.clone(),
            Friend {
                member: member.clone(),
                network_tx,
            },
        );

        self.author_operation(
            public_key.clone().into(),
            Payload::Invitation(InvitationMessage::Friend),
        )
        .await?;

        Ok(public_key)
    }

    pub async fn get_friends(&self) -> anyhow::Result<Vec<PublicKey>> {
        let friends = self.friends.read().await;
        Ok(friends.keys().cloned().collect())
    }

    pub async fn remove_friend(&self, public_key: PublicKey) -> anyhow::Result<()> {
        // TODO: shutdown inbox task, etc.
        self.friends.write().await.remove(&public_key);
        Ok(())
    }
}
