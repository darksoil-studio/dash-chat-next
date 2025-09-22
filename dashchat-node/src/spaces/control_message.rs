use p2panda_core::cbor::{EncodeError, encode_cbor};
use p2panda_spaces::OperationId;
use serde::{Deserialize, Serialize};

pub type SpacesArgs = p2panda_spaces::message::SpacesArgs<ChatId, ()>;

use crate::operation::Header;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpaceControlMessage {
    pub hash: p2panda_core::Hash,
    pub author: p2panda_spaces::ActorId,
    pub spaces_args: SpacesArgs,
}

impl p2panda_spaces::message::AuthoredMessage for SpaceControlMessage {
    fn id(&self) -> OperationId {
        OperationId::from(self.hash.clone())
    }

    fn author(&self) -> p2panda_spaces::ActorId {
        self.author
    }
}

impl p2panda_spaces::message::SpacesMessage<ChatId, ()> for SpaceControlMessage {
    fn args(&self) -> &SpacesArgs {
        &self.spaces_args
    }
}

impl SpaceControlMessage {
    pub fn new(
        author: p2panda_spaces::ActorId,
        spaces_args: SpacesArgs,
    ) -> Result<Self, EncodeError> {
        let bytes = encode_cbor(&spaces_args).unwrap();
        Ok(Self {
            hash: p2panda_core::Hash::new(bytes),
            author,
            spaces_args,
        })
    }
}
