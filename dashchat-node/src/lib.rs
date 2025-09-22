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
pub use operation::{HeaderData, InvitationMessage};
pub use p2panda_core::PrivateKey;
pub use p2panda_spaces::ActorId;

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
