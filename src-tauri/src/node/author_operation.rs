use p2panda_core::Operation;

use crate::operation::Payload;

use super::*;

pub type OpStore = p2panda_store::MemoryStore<LogId, Extensions>;

impl Node {
    pub(super) async fn author_operation(
        &self,
        topic: Topic,
        payload: Payload,
    ) -> Result<(), anyhow::Error> {
        let Operation {
            header,
            body,
            hash: _,
        } = create_operation(&self.op_store, &self.private_key, topic.clone(), payload).await?;

        p2panda_stream::operation::ingest_operation(
            &mut self.op_store.clone(),
            header.clone(),
            body.clone(),
            header.to_bytes(),
            &topic,
            false,
        )
        .await?;

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
            Topic::Inbox(_public_key) => {
                todo!();
            }
        }

        Ok(())
    }
}

async fn create_operation(
    store: &OpStore,
    private_key: &PrivateKey,
    topic: Topic,
    payload: Payload,
) -> Result<Operation<Extensions>, anyhow::Error> {
    let public_key = private_key.public_key();
    let log_id = topic.clone();

    let (control_message, body) = match payload {
        Payload::Message(message) => (None, Some(Body::new(message.as_bytes()))),
        Payload::Control(spaces_args) => (Some(spaces_args), None),
    };

    let extensions = Extensions {
        log_id: log_id.clone(),
        control_message,
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
        previous: vec![],
        extensions: Some(extensions),
    };
    header.sign(private_key);

    Ok(Operation {
        hash: header.hash(),
        header,
        body,
    })
}
