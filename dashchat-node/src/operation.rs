use p2panda_core::cbor::{DecodeError, EncodeError, decode_cbor, encode_cbor};
use p2panda_core::{Body, Extension, PruneFlag};
use serde::{Deserialize, Serialize};

use crate::chat::ChatId;
use crate::message::ChatMessage;
use crate::network::LogId;
use crate::spaces::SpaceControlMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extensions {
    pub log_id: LogId,

    /// This determines whether this is a normal chat message or a space control message.
    ///
    // TODO: some redundancy here with fields in the header, but I see no way around it
    //       due to the requirement of the Forge to not be concerned with log replication.
    // - the author field is redundant
    // - the hash could just be the hash of the header
    pub data: HeaderData,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeaderData {
    SpaceControl(Box<SpaceControlMessage>),
    Invitation(InvitationMessage),
    UseBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvitationMessage {
    /// Instructs the recipient to subscribe to the group chat topic.
    JoinGroup(ChatId),
    Friend,
}

#[derive(Clone, Debug)]
pub enum Payload {
    SpaceControl(SpaceControlMessage),
    Invitation(InvitationMessage),
    ChatMessage(ChatMessage),
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

pub fn encode_gossip_message(header: &Header, body: Option<&Body>) -> Result<Vec<u8>, EncodeError> {
    encode_cbor(&(header.to_bytes(), body.map(|body| body.to_bytes())))
}

pub fn decode_gossip_message(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), DecodeError> {
    decode_cbor(bytes)
}
