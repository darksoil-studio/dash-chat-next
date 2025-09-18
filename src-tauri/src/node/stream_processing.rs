use crate::spaces::{SpaceControlMessage, SpacesArgs};

use super::*;

impl Node {
    /// Internal function to start the necessary tasks for processing chat
    /// network activity.
    ///
    /// This must be called:
    /// - when created a new chat
    /// - when initializing the node, for each existing chat
    pub(super) async fn initialize_chat(&self, chat_id: ChatId) -> anyhow::Result<ChatNetwork> {
        let (network_tx, network_rx, gossip_ready) =
            self.network.subscribe(chat_id.clone()).await?;

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
            .ingest(self.op_store.clone(), 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    warn!("ingest operation error: {err}");
                    None
                }
            });

        {
            let mut author_store = self.author_store.clone();

            let topic = chat_id.clone();
            let chats = self.chats.clone();
            task::spawn(async move {
                while let Some(operation) = stream.next().await {
                    // let log_id: Option<LogId> = operation.header.extension();
                    author_store
                        .add_author(topic.clone(), operation.header.public_key)
                        .await;

                    if let Some(control_message) = operation
                        .header
                        .extensions
                        .and_then(|extensions| extensions.control_message)
                    {
                        match control_message.spaces_args {
                            SpacesArgs::SpaceMembership { space_id, .. }
                            | SpacesArgs::SpaceUpdate { space_id, .. }
                            | SpacesArgs::Application { space_id, .. } => {
                                // TODO: maybe close down the chat tasks if we are kicked out?
                            }
                            _ => {}
                        }
                        chats
                            .write()
                            .await
                            .get_mut(&topic)
                            .ok_or(anyhow!("Chat not found"))?
                            .manager
                            .process(&control_message)
                            .await?;
                    }

                    let body_len = operation.body.as_ref().map_or(0, |body| body.size());
                    debug!(
                        seq_num = operation.header.seq_num,
                        len = body_len,
                        hash = %operation.hash,
                        "received operation"
                    );
                }
                anyhow::Ok(())
            });
        }

        let rng = Rng::default();

        let spaces_store = crate::spaces::create_test_store(self.private_key.clone());
        let forge = DashForge {
            chat_id,
            private_key: self.private_key.clone(),
        };

        let manager = DashManager::new(spaces_store, forge, rng).unwrap();

        let chat_network = ChatNetwork {
            sender: network_tx,
            manager,
        };
        self.chats
            .write()
            .await
            .insert(chat_id, chat_network.clone());

        Ok(chat_network)
    }
}
