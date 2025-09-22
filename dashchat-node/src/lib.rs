mod chat;
mod forge;
mod friend;
mod message;
mod network;
mod node;
mod operation;
mod spaces;

#[cfg(feature = "testing")]
pub mod testing;

use p2panda_core::IdentityError;

pub use node::{Node, NodeConfig, Notification};
pub use operation::InvitationMessage;
pub use p2panda_core::PrivateKey;
pub use p2panda_spaces::ActorId;
use serde::{Serialize, de::DeserializeOwned};

pub use crate::{chat::ChatId, message::ChatMessage, spaces::MemberCode};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
)]
pub struct PK(p2panda_core::PublicKey);

impl std::fmt::Debug for PK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut k = self.0.to_string();
        k.truncate(8);
        write!(f, "PK|{k}")
    }
}

impl PK {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, IdentityError> {
        Ok(Self(p2panda_core::PublicKey::from_bytes(bytes)?))
    }
}

impl From<ActorId> for PK {
    fn from(id: ActorId) -> Self {
        Self(p2panda_core::PublicKey::from_bytes(id.as_bytes()).unwrap())
    }
}

impl From<PK> for ActorId {
    fn from(pk: PK) -> Self {
        Self::from_bytes(pk.0.as_bytes()).unwrap()
    }
}

pub trait Cbor: serde::Serialize + serde::de::DeserializeOwned {
    fn as_bytes(&self) -> Vec<u8> {
        p2panda_core::cbor::encode_cbor(&self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, p2panda_core::cbor::DecodeError> {
        p2panda_core::cbor::decode_cbor(bytes)
    }
}
