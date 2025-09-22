use std::time::{Duration, Instant};

use p2panda_core::PrivateKey;
use tokio::sync::mpsc::Receiver;

use crate::{NodeConfig, Notification, node::Node};

#[derive(Debug, Clone, derive_more::Deref)]
pub struct TestNode(Node);

impl TestNode {
    pub async fn new() -> (Self, Watcher<Notification>) {
        let private_key = PrivateKey::new();
        let (notification_tx, notification_rx) = tokio::sync::mpsc::channel(100);
        let node = Self(
            Node::new(private_key, NodeConfig::default(), Some(notification_tx))
                .await
                .unwrap(),
        );
        (node, Watcher(notification_rx))
    }
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Watcher<T>(Receiver<T>);

impl<T> Watcher<T> {
    pub async fn watch_for(
        &mut self,
        timeout: tokio::time::Duration,
        f: impl Fn(&T) -> bool,
    ) -> anyhow::Result<T> {
        let timeout = tokio::time::sleep(timeout);
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                item = self.0.recv() => {
                    match item {
                        Some(item) if f(&item) => return Ok(item),
                        Some(_) => continue,
                        None => return Err(anyhow::anyhow!("channel closed")),
                    }
                }
                _ = &mut timeout => return Err(anyhow::anyhow!("timeout")),
            }
        }
    }
}

pub async fn wait_for<F>(poll: Duration, timeout: Duration, f: impl Fn() -> F) -> anyhow::Result<()>
where
    F: Future<Output = bool>,
{
    assert!(poll < timeout);
    let start = Instant::now();
    loop {
        if f().await {
            break Ok(());
        }
        tokio::time::sleep(poll).await;
        if start.elapsed() > timeout {
            return Err(anyhow::anyhow!("timeout"));
        }
    }
}
