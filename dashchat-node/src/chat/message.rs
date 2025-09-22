use serde::{Deserialize, Serialize};

use crate::{Cbor, PK};

/// A standalone chat message suitable for sending to the frontend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub content: ChatMessageContent,
    pub author: PK,
    pub timestamp: u64,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, derive_more::From, derive_more::Deref,
)]
pub struct ChatMessageContent(String);

impl From<&str> for ChatMessageContent {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Cbor for ChatMessageContent {}
