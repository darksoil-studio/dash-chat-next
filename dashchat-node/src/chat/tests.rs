use std::time::Duration;

use crate::{
    InvitationMessage, Payload,
    spaces::{SpaceControlMessage, SpacesArgs},
    testing::{TestNode, wait_for},
};

#[tokio::test(flavor = "multi_thread")]
async fn test_group_2() {
    crate::testing::setup_tracing("dashchat=info");

    println!("nodes:");
    let (alice, mut alice_rx) = TestNode::new().await;
    println!("alice: {:?}", alice.public_key());
    let (bob, mut bob_rx) = TestNode::new().await;
    println!("bob:   {:?}", bob.public_key());

    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(10),
        || async {
            alice.network.known_peers().await.unwrap().len() == 1
                && bob.network.known_peers().await.unwrap().len() == 1
        },
    )
    .await
    .unwrap();

    println!("peers see each other");

    alice.add_friend(bob.me().await.unwrap()).await.unwrap();
    // TODO: doesn't work without this
    bob.add_friend(alice.me().await.unwrap()).await.unwrap();

    let (chat_id, _) = alice.create_group().await.unwrap();

    alice.add_member(chat_id, bob.public_key()).await.unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            n.payload == Payload::Invitation(InvitationMessage::Friend)
        })
        .await
        .unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            n.payload == Payload::Invitation(InvitationMessage::JoinGroup(chat_id))
        })
        .await
        .unwrap();

    // Bob has joined the group via his inbox topic
    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(5),
        || async { bob.get_groups().await.unwrap().contains(&chat_id) },
    )
    .await
    .unwrap();

    alice.send_message(chat_id, "Hello".into()).await.unwrap();

    alice_rx
        .watch_for(Duration::from_secs(5), |n| {
            matches!(
                n.payload,
                Payload::SpaceControl(SpaceControlMessage {
                    spaces_args: SpacesArgs::Application { .. },
                    ..
                })
            )
        })
        .await
        .unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            matches!(
                n.payload,
                Payload::SpaceControl(SpaceControlMessage {
                    spaces_args: SpacesArgs::Application { .. },
                    ..
                })
            )
        })
        .await
        .unwrap();

    let alice_messages = alice.get_messages(chat_id).await.unwrap();
    let bob_messages = bob.get_messages(chat_id).await.unwrap();

    assert_eq!(alice_messages, bob_messages);
}
