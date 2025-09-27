use p2panda_core::{Hash, Operation};
use p2panda_spaces::{OperationId, message::SpacesArgs};
use p2panda_stream::operation::IngestResult;

use crate::{AsBody, ShortId, operation::Payload};

use super::*;

pub type OpStore = p2panda_store::MemoryStore<LogId, Extensions>;

impl Node {
    #[tracing::instrument(skip_all)]
    pub(super) async fn author_operation(
        &self,
        topic: Topic,
        payload: Payload,
    ) -> Result<Header<Extensions>, anyhow::Error> {
        self.author_operation_with_deps(topic, payload, vec![])
            .await
    }

    #[tracing::instrument(skip_all)]
    pub(super) async fn author_operation_with_deps(
        &self,
        topic: Topic,
        payload: Payload,
        mut deps: Vec<p2panda_core::Hash>,
    ) -> Result<Header<Extensions>, anyhow::Error> {
        let space_deps: Vec<OperationId> = match &payload {
            Payload::SpaceControl(msgs) => msgs
                .iter()
                .flat_map(|msg| match &msg.spaces_args {
                    SpacesArgs::KeyBundle { .. } => vec![],
                    SpacesArgs::SpaceMembership {
                        space_dependencies,
                        auth_message_id,
                        ..
                    } => [auth_message_id.clone()]
                        .into_iter()
                        .chain(space_dependencies.clone())
                        .collect(),
                    SpacesArgs::Auth {
                        auth_dependencies, ..
                    } => auth_dependencies.into_iter().cloned().collect::<Vec<_>>(),
                    SpacesArgs::SpaceUpdate {
                        space_dependencies, ..
                    } => space_dependencies.into_iter().cloned().collect::<Vec<_>>(),
                    SpacesArgs::Application {
                        space_dependencies, ..
                    } => space_dependencies.into_iter().cloned().collect::<Vec<_>>(),
                })
                .collect(),

            Payload::Invitation(invitation) => {
                vec![]
            }
        };

        deps.extend(
            space_deps
                .into_iter()
                .map(|id| Hash::from_bytes(id.as_bytes().clone())),
        );

        let Operation { header, body, hash } = create_operation(
            &self.op_store,
            &self.private_key,
            topic.clone(),
            payload.clone(),
            deps,
        )
        .await?;

        let result = p2panda_stream::operation::ingest_operation(
            &mut self.op_store.clone(),
            header.clone(),
            body.clone(),
            header.to_bytes(),
            &topic,
            false,
        )
        .await?;

        match result {
            IngestResult::Complete(op @ Operation { hash: hash2, .. }) => {
                assert_eq!(hash, hash2);

                // TODO: ask p2panda what's up with this?
                // XXX: this produces tons of duplicates, but I couldn't make it work any other way!
                self.process_operation(topic, op, self.author_store.clone(), true)
                    .await?;

                // self.notify_payload(&header, &payload).await?;
                tracing::debug!(?topic, hash = hash.short(), "authored operation");
            }

            IngestResult::Retry(h, _, _, missing) => {
                let backlink = h.backlink.as_ref().map(|h| h.short());
                tracing::warn!(
                    ?topic,
                    hash = hash.short(),
                    ?backlink,
                    ?missing,
                    "operation could not be ingested"
                );
            } // IngestResult::Duplicate(op) => {
              //     tracing::warn!(?topic, hash = hash.short(), "operation already exists");
              //     return Ok(op.header);
              // }
        }

        // Do gossip broadcast for newly created operations
        match topic {
            Topic::Chat(chat_id) => {
                let chat_network = self
                    .chats
                    .read()
                    .await
                    .get(&chat_id)
                    .cloned()
                    .ok_or(anyhow!("Chat not found"))?;

                chat_network
                    .sender
                    .send(ToNetwork::Message {
                        bytes: encode_gossip_message(&header, body.as_ref())?,
                    })
                    .await?;
            }
            Topic::Inbox(public_key) => {
                let friend = self.friends.read().await.get(&public_key).cloned();

                if let Some(friend) = friend {
                    friend
                        .network_tx
                        .send(ToNetwork::Message {
                            bytes: encode_gossip_message(&header, body.as_ref())?,
                        })
                        .await?;
                    tracing::debug!(%public_key, "Friend found, gossiping invite");
                } else {
                    tracing::warn!(%public_key, "Friend not found, skipping gossip");
                }
            }
        }

        Ok(header)
    }
}

pub(crate) async fn create_operation(
    store: &OpStore,
    private_key: &PrivateKey,
    topic: Topic,
    payload: Payload,
    deps: Vec<p2panda_core::Hash>,
) -> Result<Operation<Extensions>, anyhow::Error> {
    let public_key = private_key.public_key();
    let log_id = topic.clone();

    let body = Some(payload.try_into_body()?);

    let extensions = Extensions {
        log_id: log_id.clone(),
    };

    // TODO: atomicity, see https://github.com/p2panda/p2panda/issues/798
    let latest_operation = store.latest_operation(&public_key, &log_id).await?;

    let (seq_num, backlink) = match latest_operation {
        Some((header, _)) => (header.seq_num + 1, Some(header.hash())),
        None => (0, None),
    };

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time from operation system")
        .as_secs();
    let mut header = Header {
        version: 1,
        public_key,
        signature: None,
        payload_size: body.as_ref().map_or(0, |body| body.size()),
        payload_hash: body.as_ref().map(|body| body.hash()),
        timestamp,
        seq_num,
        backlink,
        previous: deps,
        extensions: Some(extensions),
    };
    header.sign(private_key);

    Ok(Operation {
        hash: header.hash(),
        header,
        body,
    })
}
