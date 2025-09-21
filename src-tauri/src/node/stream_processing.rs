use p2panda_core::Operation;
use tokio_stream::Stream;

use crate::{operation::InvitationMessage, spaces::SpacesArgs};

use super::*;

impl Node {
    pub(super) async fn initialize_inbox(
        &self,
        pubkey: PublicKey,
    ) -> anyhow::Result<tokio::sync::mpsc::Sender<ToNetwork>> {
        if let Some(friend) = self.friends.read().await.get(&pubkey) {
            return Ok(friend.network_tx.clone());
        }

        let network_tx = self.initialize_topic(pubkey.into()).await?;
        Ok(network_tx)
    }

    pub(super) async fn initialize_group(&self, chat_id: ChatId) -> anyhow::Result<ChatNetwork> {
        if let Some(chat_network) = self.chats.read().await.get(&chat_id) {
            return Ok(chat_network.clone());
        }

        self.author_store
            .add_author(chat_id.into(), self.public_key())
            .await;

        let network_tx = self.initialize_topic(chat_id.into()).await?;

        let chat_network = ChatNetwork { sender: network_tx };
        self.chats
            .write()
            .await
            .insert(chat_id, chat_network.clone());

        Ok(chat_network)
    }

    /// Internal function to start the necessary tasks for processing group chat
    /// network activity.
    ///
    /// This must be called:
    /// - when created a new group chat
    /// - when initializing the node, for each existing group chat
    pub(super) async fn initialize_topic(
        &self,
        topic: Topic,
    ) -> anyhow::Result<mpsc::Sender<ToNetwork>> {
        println!("*** initializing topic: {:?}", topic);
        let (network_tx, network_rx, _gossip_ready) = self.network.subscribe(topic.clone()).await?;

        let stream = ReceiverStream::new(network_rx);
        let stream = stream.filter_map(|event| match event {
            FromNetwork::GossipMessage { bytes, .. } => match decode_gossip_message(&bytes) {
                Ok(result) => Some(result),
                Err(err) => {
                    println!("*** decode gossip message error: {err}");
                    warn!("could not decode gossip message: {err}");
                    None
                }
            },
            FromNetwork::SyncMessage {
                header, payload, ..
            } => Some((header, payload)),
        });

        // Decode and ingest the p2panda operations.
        let stream = stream
            .decode()
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    println!("*** decode operation error: {err}");
                    warn!("decode operation error: {err}");
                    None
                }
            })
            .ingest(self.op_store.clone(), 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    println!("*** ingest operation error: {err}");
                    warn!("ingest operation error: {err}");
                    None
                }
            });

        let author_store = self.author_store.clone();
        self.spawn_stream_process_loop(stream, author_store, topic);

        Ok(network_tx)
    }

    fn spawn_stream_process_loop(
        &self,
        stream: impl Stream<Item = Operation<Extensions>> + Send + 'static,
        mut author_store: AuthorStore<Topic>,
        topic: Topic,
    ) {
        let node = self.clone();
        let mut stream = Box::pin(stream);
        task::spawn(async move {
            println!("*** stream process loop started for topic: {:?}", topic);
            while let Some(operation) = stream.next().await {
                // let log_id: Option<LogId> = operation.header.extension();

                let Operation { header, body, hash } = operation;

                author_store
                    .add_author(topic.clone(), header.public_key)
                    .await;

                println!("*** adding author for topic {:?}", topic);

                let Some(extensions) = header.extensions else {
                    println!("*** no extensions");
                    warn!("no extensions");
                    continue;
                };

                println!(
                    "*** RECEIVED OPERATION -- topic {topic:?} -- operation: {:?}",
                    extensions.data
                );

                match extensions.data {
                    HeaderData::SpaceControl(control_message) => {
                        match control_message.spaces_args {
                            SpacesArgs::SpaceMembership { space_id, .. }
                            | SpacesArgs::SpaceUpdate { space_id, .. }
                            | SpacesArgs::Application { space_id, .. } => {
                                // TODO: maybe close down the chat tasks if we are kicked out?
                            }
                            _ => {}
                        }
                        node.manager
                            .process(&control_message)
                            .await
                            .expect("TODO ?");
                    }
                    HeaderData::Invitation(invitation) => {
                        println!("*** received invitation message: {:?}", invitation);
                        match invitation {
                            InvitationMessage::JoinGroup(chat_id) => {
                                node.join_group(chat_id).await?;
                                // TODO: maybe close down the chat tasks if we are kicked out?
                            }
                            InvitationMessage::Friend => {
                                println!(
                                    "*** received friend invitation from: {:?}",
                                    header.public_key
                                );
                            }
                            InvitationMessage::Test => {
                                println!("*** received test message from: {:?}", header.public_key);
                            }
                        }
                    }
                    HeaderData::UseBody => {}
                }

                let body_len = body.as_ref().map_or(0, |body| body.size());
                debug!(
                    seq_num = header.seq_num,
                    len = body_len,
                    hash = %hash,
                    "received operation"
                );
            }
            println!("*** ingestion stream ended");
            anyhow::Ok(())
        });
    }
}
