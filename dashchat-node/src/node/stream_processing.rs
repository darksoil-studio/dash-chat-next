use p2panda_core::Operation;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio_stream::Stream;

use crate::{operation::InvitationMessage, spaces::SpacesArgs};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub data: HeaderData,
    pub author: PK,
    pub timestamp: u64,
}

impl Node {
    pub(super) async fn initialize_inbox(
        &self,
        pubkey: PK,
    ) -> anyhow::Result<tokio::sync::mpsc::Sender<ToNetwork>> {
        if let Some(friend) = self.friends.read().await.get(&pubkey) {
            return Ok(friend.network_tx.clone());
        }

        let (network_tx, _gossip_ready) = self.initialize_topic(pubkey.into()).await?;
        Ok(network_tx)
    }

    pub(super) async fn initialize_group(&self, chat_id: ChatId) -> anyhow::Result<ChatNetwork> {
        if let Some(chat_network) = self.chats.read().await.get(&chat_id) {
            return Ok(chat_network.clone());
        }

        self.author_store
            .add_author(chat_id.into(), self.public_key())
            .await;

        let (network_tx, _gossip_ready) = self.initialize_topic(chat_id.into()).await?;

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
    ) -> anyhow::Result<(Sender<ToNetwork>, tokio::sync::oneshot::Receiver<()>)> {
        let (network_tx, network_rx, gossip_ready) = self.network.subscribe(topic.clone()).await?;
        tracing::info!(?topic, "subscribed to topic");

        let stream = ReceiverStream::new(network_rx);
        let stream = stream.filter_map(|event| match event {
            FromNetwork::GossipMessage { bytes, .. } => match decode_gossip_message(&bytes) {
                Ok(result) => Some(result),
                Err(err) => {
                    tracing::warn!(?err, "decode gossip message error");
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
                    tracing::warn!(?err, "decode operation error");
                    None
                }
            })
            .ingest(self.op_store.clone(), 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    tracing::warn!(?err, "ingest operation error");
                    None
                }
            });

        let author_store = self.author_store.clone();
        self.spawn_stream_process_loop(stream, author_store, topic.clone());

        Ok((network_tx, gossip_ready))
    }

    fn spawn_stream_process_loop(
        &self,
        stream: impl Stream<Item = Operation<Extensions>> + Send + 'static,
        author_store: AuthorStore<Topic>,
        topic: Topic,
    ) {
        let node = self.clone();
        let mut stream = Box::pin(stream);
        task::spawn(async move {
            tracing::debug!(?topic, "stream process loop started");
            while let Some(operation) = stream.next().await {
                // let log_id: Option<LogId> = operation.header.extension();

                let Operation { header, body, hash } = operation;

                author_store
                    .add_author(topic.clone(), header.public_key)
                    .await;

                tracing::debug!(?topic, "adding author");

                let Some(extensions) = header.extensions else {
                    tracing::warn!("no extensions");
                    continue;
                };

                tracing::info!(
                    ?topic,
                    ?extensions.data,
                    "RECEIVED OPERATION"
                );

                match &extensions.data {
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
                        tracing::debug!(?invitation, "received invitation message");
                        match invitation {
                            InvitationMessage::JoinGroup(chat_id) => {
                                node.join_group(*chat_id).await?;
                                // TODO: maybe close down the chat tasks if we are kicked out?
                            }
                            InvitationMessage::Friend => {
                                tracing::debug!(
                                    "received friend invitation from: {:?}",
                                    header.public_key
                                );
                            }
                        }
                    }
                    HeaderData::UseBody => {}
                }

                if let Some(notification_tx) = &node.notification_tx {
                    notification_tx
                        .send(Notification {
                            data: extensions.data,
                            author: header.public_key.into(),
                            timestamp: header.timestamp,
                        })
                        .await?;
                }
            }
            tracing::warn!("ingestion stream ended");
            anyhow::Ok(())
        });
    }
}
