#![cfg(feature = "testing")]

use std::time::Duration;

use dashchat_node::{
    HeaderData, InvitationMessage,
    testing::{TestNode, wait_for},
};

#[tokio::test(flavor = "multi_thread")]
async fn test_group_2() -> anyhow::Result<()> {
    dashchat_node::testing::setup_tracing("dashchat=info");

    println!("nodes:");
    let (alice, alice_rx) = TestNode::new().await;
    println!("alice: {:?}", alice.public_key());
    let (bob, mut bob_rx) = TestNode::new().await;
    println!("bob:   {:?}", bob.public_key());

    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(5),
        || async {
            alice.network.known_peers().await.unwrap().len() == 1
                && bob.network.known_peers().await.unwrap().len() == 1
        },
    )
    .await?;

    println!("peers see each other");

    alice.add_friend(bob.me().await?).await?;

    let (chat_id, _) = alice.create_group().await?;
    alice.add_member(chat_id, bob.public_key()).await?;

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            n.data == HeaderData::Invitation(InvitationMessage::Friend)
        })
        .await
        .unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            n.data == HeaderData::Invitation(InvitationMessage::JoinGroup(chat_id))
        })
        .await
        .unwrap();

    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(5),
        || async { bob.get_groups().await.unwrap().contains(&chat_id) },
    )
    .await?;

    Ok(())
}
