use std::time::SystemTime;

use p2panda_core::cbor::{decode_cbor, encode_cbor, DecodeError, EncodeError};
use p2panda_core::{Body, Extension, Header, PrivateKey, PruneFlag};
use p2panda_store::{LocalLogStore, LogStore, MemoryStore};
use serde::{Deserialize, Serialize};

use crate::chat::LogId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extensions {
   pub log_id: LogId,

    #[serde(
        rename = "prune",
        skip_serializing_if = "PruneFlag::is_not_set",
        default = "PruneFlag::default"
    )]
   pub prune_flag: PruneFlag,
}

impl Extension<LogId> for Extensions {
    fn extract(header: &Header<Self>) -> Option<LogId> {
        header
            .extensions
            .as_ref()
            .map(|extensions| extensions.log_id.clone())
    }
}

impl Extension<PruneFlag> for Extensions {
    fn extract(header: &Header<Self>) -> Option<PruneFlag> {
        header
            .extensions
            .as_ref()
            .map(|extensions| extensions.prune_flag.clone())
    }
}

pub async fn create_operation<S: LogStore<LogId, Extensions>>(
    store: &mut S,
    log_id: LogId,
    private_key: &PrivateKey,
    body: Option<&[u8]>,
) -> Result<(Header<Extensions>, Option<Body>), S::Error> {
    let body = body.map(Body::new);
    let public_key = private_key.public_key();

    let latest_operation = store.latest_operation(&public_key, &log_id).await?;

    let (seq_num, backlink) = match latest_operation {
        Some((header, _)) => (header.seq_num + 1, Some(header.hash())),
        None => (0, None),
    };

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time from operation system")
        .as_secs();

    let extensions = Extensions {
        log_id,
        prune_flag: PruneFlag::new(false),
    };

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

    Ok((header, body))
}

pub fn encode_gossip_message(
    header: &Header<Extensions>,
    body: Option<&Body>,
) -> Result<Vec<u8>, EncodeError> {
    encode_cbor(&(header.to_bytes(), body.map(|body| body.to_bytes())))
}

pub fn decode_gossip_message(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), DecodeError> {
    decode_cbor(bytes)
}
