use p2panda_core::cbor::{decode_cbor, encode_cbor, DecodeError, EncodeError};
use p2panda_core::{Body, Extension, Hash, PruneFlag};
use p2panda_spaces::OperationId;
use serde::{Deserialize, Serialize};

use crate::chat::{ChatId, LogId};

pub type SpacesArgs = p2panda_spaces::message::SpacesArgs<ChatId, ()>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extensions {
    pub log_id: LogId,

    /// This determines whether this is a normal chat message or a space control message.
    pub spaces_args: Option<SpacesArgs>,
}

pub type Header = p2panda_core::Header<Extensions>;

impl Extension<LogId> for Extensions {
    fn extract(header: &Header) -> Option<LogId> {
        header
            .extensions
            .as_ref()
            .map(|extensions| extensions.log_id.clone())
    }
}

impl Extension<PruneFlag> for Extensions {
    fn extract(_header: &Header) -> Option<PruneFlag> {
        Some(PruneFlag::new(false))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupControlMessage {
    pub hash: Hash,
    pub author: p2panda_spaces::ActorId,
    pub spaces_args: SpacesArgs,
}

impl p2panda_spaces::message::AuthoredMessage for GroupControlMessage {
    fn id(&self) -> OperationId {
        OperationId::from(self.hash.clone())
    }

    fn author(&self) -> p2panda_spaces::ActorId {
        self.author
    }
}

impl p2panda_spaces::message::SpacesMessage<ChatId, ()> for GroupControlMessage {
    fn args(&self) -> &SpacesArgs {
        &self.spaces_args
    }
}

impl GroupControlMessage {
    pub fn from_header(hash: Hash, header: Header) -> Option<Self> {
        Some(Self {
            hash,
            author: header.public_key.into(),
            spaces_args: header.extensions?.spaces_args?,
        })
    }
}

pub fn encode_gossip_message(header: &Header, body: Option<&Body>) -> Result<Vec<u8>, EncodeError> {
    encode_cbor(&(header.to_bytes(), body.map(|body| body.to_bytes())))
}

pub fn decode_gossip_message(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), DecodeError> {
    decode_cbor(bytes)
}
