use std::{sync::Arc, time::SystemTime};

use p2panda_core::{Body, Operation, PrivateKey, PublicKey};
use p2panda_net::ToNetwork;
use p2panda_store::LogStore;
use tokio::sync::RwLock;

use crate::{
    chat::{ChatId, LogId},
    operation::{encode_gossip_message, Extensions, GroupControlMessage, Header, SpacesArgs},
    spaces::store::TestStore,
};

pub type SpacesStore = Arc<RwLock<TestStore>>;
pub type OpStore = p2panda_store::MemoryStore<LogId, Extensions>;

#[derive(Clone, Debug)]
pub struct DashForge {
    pub chat_id: ChatId,
    pub op_store: OpStore,
    pub gossip_tx: tokio::sync::mpsc::Sender<ToNetwork>,
    pub private_key: PrivateKey,
}

pub enum Payload {
    Message(String),
    Control(SpacesArgs),
}

impl DashForge {
    /// Create the Operation, store it to the op store, and gossip it.
    pub async fn author_message(&mut self, message: String) -> Result<(), anyhow::Error> {
        let Operation {
            header,
            body,
            hash: _,
        } = create_operation(
            &self.op_store,
            &self.private_key,
            &self.chat_id,
            Payload::Message(message),
        )
        .await?;

        p2panda_stream::operation::ingest_operation(
            &mut self.op_store,
            header.clone(),
            body.clone(),
            header.to_bytes(),
            &(self.chat_id, self.private_key.public_key()),
            false,
        )
        .await?;

        self.gossip_tx
            .send(ToNetwork::Message {
                bytes: encode_gossip_message(&header, body.as_ref())?,
            })
            .await?;

        Ok(())
    }
}

impl p2panda_spaces::forge::Forge<ChatId, GroupControlMessage, ()> for DashForge {
    type Error = anyhow::Error;

    fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    async fn forge(&mut self, args: SpacesArgs) -> Result<GroupControlMessage, Self::Error> {
        // TODO: atomicity, see https://github.com/p2panda/p2panda/issues/798
        let Operation { header, body, hash } = create_operation(
            &self.op_store,
            &self.private_key,
            &self.chat_id,
            Payload::Control(args),
        )
        .await?;
        let control = GroupControlMessage::from_header(hash, header.clone()).unwrap();

        p2panda_stream::operation::ingest_operation(
            &mut self.op_store,
            header.clone(),
            body.clone(),
            header.to_bytes(),
            &(self.chat_id, self.private_key.public_key()),
            false,
        )
        .await?;

        Ok(control)
    }

    async fn forge_ephemeral(
        &mut self,
        private_key: PrivateKey,
        args: SpacesArgs,
    ) -> Result<GroupControlMessage, Self::Error> {
        Ok(GroupControlMessage {
            // TODO: is this ok?
            hash: p2panda_core::Hash::new([0; 32]),
            author: private_key.public_key().into(),
            spaces_args: args,
        })
    }
}

async fn create_operation(
    store: &OpStore,
    private_key: &PrivateKey,
    chat_id: &ChatId,
    payload: Payload,
) -> Result<Operation<Extensions>, anyhow::Error> {
    let public_key = private_key.public_key();
    let log_id = (chat_id.clone(), public_key);

    let (spaces_args, body) = match payload {
        Payload::Message(message) => (None, Some(Body::new(message.as_bytes()))),
        Payload::Control(spaces_args) => (Some(spaces_args), None),
    };

    let extensions = Extensions {
        log_id: log_id.clone(),
        spaces_args,
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
