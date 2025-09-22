use super::*;

use p2panda_net::ToNetwork;
use p2panda_spaces::member::Member;

#[derive(Clone, Debug)]
pub struct Friend {
    pub member: Member,
    pub network_tx: tokio::sync::mpsc::Sender<ToNetwork>,
}
