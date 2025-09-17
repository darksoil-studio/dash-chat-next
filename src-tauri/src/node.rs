use std::net::{Ipv4Addr, SocketAddr};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{anyhow, Context, Result};
use p2panda_core::{Body, Hash, Header, PrivateKey, PruneFlag, PublicKey};
use p2panda_discovery::mdns::LocalDiscovery;
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

use crate::chat::{AuthorStore, Chat, LogId};
use crate::operation::{
    create_operation, decode_gossip_message, encode_gossip_message, Extensions,
};

// const RELAY_ENDPOINT: &str = "https://wasser.liebechaos.org";

const NETWORK_ID: &str = "p2panda-tauri-chat";

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
    op_store: Arc<RwLock<MemoryStore<LogId, Extensions>>>,
    network: Network<Chat>,
    tx: mpsc::Sender<ToNetwork>,
    author_store: Arc<RwLock<AuthorStore>>,
    config: Config,
    private_key: PrivateKey,
}

impl Node {
    pub fn chat() -> Chat {
        Chat::new([11; 32])
    }

    pub async fn new(private_key: PrivateKey, config: Config) -> Result<Self> {
        // Launch an p2p network.
        let network_id = Hash::new([88; 32]);

        let mdns = LocalDiscovery::new();

        let operation_store = MemoryStore::<LogId, Extensions>::new();
        let mut author_store = AuthorStore::new();

        author_store
            .add_author(Self::chat(), private_key.public_key())
            .await;

        // let relay_url = RELAY_ENDPOINT.parse()?;

        let mut network_builder = NetworkBuilder::new(network_id.into())
            .private_key(private_key.clone())
            .discovery(mdns)
            .gossip(GossipConfig {
                max_message_size: MAX_MESSAGE_SIZE,
            });
        // .relay(relay_url, false, 0);

        let sync_protocol = LogSyncProtocol::new(author_store.clone(), operation_store.clone());
        let sync_config = SyncConfiguration::new(sync_protocol);
        network_builder = network_builder.sync(sync_config);

        // if config.bootstrap {
        //     network_builder = network_builder.bootstrap();
        // }

        // if let Some(bootstrap) = config.use_bootstrap {
        //     network_builder = network_builder.direct_address(bootstrap, vec![], None);
        // }

        let network = network_builder.build().await.context("spawn p2p network")?;

        // let topic = config.topic.clone();
        let (network_tx, network_rx, gossip_ready) = network.subscribe(Self::chat()).await?;

        task::spawn(async move {
            if gossip_ready.await.is_ok() {
                debug!("joined gossip overlay");
            }
        });

        let stream = ReceiverStream::new(network_rx);
        let stream = stream.filter_map(|event| match event {
            FromNetwork::GossipMessage { bytes, .. } => match decode_gossip_message(&bytes) {
                Ok(result) => Some(result),
                Err(err) => {
                    warn!("could not decode gossip message: {err}");
                    None
                }
            },
            FromNetwork::SyncMessage {
                header, payload, ..
            } => Some((header, payload)),
        });

        // Decode and ingest the p2panda operations.
        let mut stream = stream
            .decode()
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    warn!("decode operation error: {err}");
                    None
                }
            })
            .ingest(operation_store.clone(), 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    warn!("ingest operation error: {err}");
                    None
                }
            });

        {
            let mut author_store = author_store.clone();

            task::spawn(async move {
                while let Some(operation) = stream.next().await {
                    // let log_id: Option<LogId> = operation.header.extension();
                    let topic = Self::chat();
                    author_store
                        .add_author(topic, operation.header.public_key)
                        .await;

                    let body_len = operation.body.as_ref().map_or(0, |body| body.size());
                    debug!(
                        seq_num = operation.header.seq_num,
                        len = body_len,
                        hash = %operation.hash,
                        "received operation"
                    );
                }
            });
        }

        Ok(Self {
            tx: network_tx,
            op_store: Arc::new(RwLock::new(operation_store)),
            author_store: Arc::new(RwLock::new(author_store)),
            network,
            config,
            private_key,
        })
    }

    pub async fn get_messages(&self) -> anyhow::Result<Vec<String>> {
        let author = self.author_store.clone().read_owned().await;
        let Some(authors) = author.authors(&Self::chat()).await else {
            return Ok(vec![]);
        };

        let mut messages = vec![];

        for author in authors {
            let mut m = self.get_messages_from(author).await?;
            messages.append(&mut m);
        }

        Ok(messages)
    }

    pub async fn get_messages_from(&self, public_key: PublicKey) -> anyhow::Result<Vec<String>> {
        let store = self.op_store.clone().read_owned().await;

        let log_id = Self::chat().log_id(public_key);
        let log = store.get_log(&public_key, &log_id, None).await?;

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

    pub async fn send_message(&self, message: String) -> anyhow::Result<()> {
        let body = Body::new(message.as_bytes());
        let public_key = self.private_key.public_key();

        let log_id = Self::chat().log_id(public_key);

        let store = self.op_store.clone().write_owned().await;

        let latest_operation = store.latest_operation(&public_key, &log_id).await?;

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
            prune_flag: PruneFlag::new(false),
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
            &mut store.clone(),
            header.clone(),
            Some(body.clone()),
            header.to_bytes(),
            &log_id,
            false,
        )
        .await?;

        self.tx
            .send(ToNetwork::Message {
                bytes: encode_gossip_message(&header, Some(&body))?,
            })
            .await?;

        Ok(())
    }
}
