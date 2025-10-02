use std::time::Duration;

use p2panda_auth::Access;
use p2panda_net::{Network, NodeAddress};
use p2panda_spaces::message::AuthoredMessage;
use p2panda_store::LogStore;

use crate::{
    AsBody, ChatMessage, InvitationMessage, PK, Payload, ShortId,
    network::Topic,
    spaces::{SpaceControlMessage, SpacesArgs},
    testing::{
        ClusterConfig, TestCluster, TestNode, consistency, introduce, introduce_and_wait, wait_for,
    },
};

const TRACING_FILTER: &str =
    "dashchat=info,p2panda_stream=info,p2panda_auth=warn,p2panda_spaces=warn";

#[tokio::test(flavor = "multi_thread")]
async fn test_group_2() {
    crate::testing::setup_tracing(TRACING_FILTER);

    println!("nodes:");
    let (alice, mut alice_rx) = TestNode::new().await;
    println!("alice: {:?}", alice.public_key());
    let (bob, mut bob_rx) = TestNode::new().await;
    println!("bob:   {:?}", bob.public_key());

    introduce_and_wait([&alice.network, &bob.network]).await;

    println!("peers see each other");

    alice.add_friend(bob.me().await.unwrap()).await.unwrap();
    // TODO: doesn't work without this
    bob.add_friend(alice.me().await.unwrap()).await.unwrap();

    let (chat_id, _) = alice.create_group().await.unwrap();

    alice.add_member(chat_id, bob.public_key()).await.unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            matches!(n.payload, Payload::Invitation(InvitationMessage::Friend))
        })
        .await
        .unwrap();

    bob_rx
        .watch_for(Duration::from_secs(5), |n| {
            matches!(
                n.payload,
                Payload::Invitation(InvitationMessage::JoinGroup(id)) if id == chat_id
            )
        })
        .await
        .unwrap();

    // Bob has joined the group via his inbox topic
    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(5),
        || async { bob.get_groups().await.unwrap().contains(&chat_id).ok_or(()) },
    )
    .await
    .unwrap();

    alice.send_message(chat_id, "Hello".into()).await.unwrap();

    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(5),
        || async {
            let msgs = [
                alice.get_messages(chat_id).await.unwrap().len(),
                bob.get_messages(chat_id).await.unwrap().len(),
            ];
            msgs.iter().all(|m| *m == 1).ok_or(msgs)
        },
    )
    .await
    .unwrap();

    let alice_messages = alice.get_messages(chat_id).await.unwrap();
    let bob_messages = bob.get_messages(chat_id).await.unwrap();

    assert_eq!(alice_messages, bob_messages);
    assert_eq!(
        bob_messages.first().map(|m| m.content.clone()),
        Some("Hello".into())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_group_3() {
    crate::testing::setup_tracing(TRACING_FILTER);

    let cluster = TestCluster::new(ClusterConfig::default()).await;
    let [alice, bob, carol] = cluster.nodes().await;

    introduce_and_wait([&alice.network, &bob.network, &carol.network]).await;

    println!("=== NODES ===");
    println!("alice:    {:?}", alice.public_key());
    println!("bob:      {:?}", bob.public_key());
    println!("carol:    {:?}", carol.public_key());

    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(10),
        || async {
            (alice.network.known_peers().await.unwrap().len() == 2
                && bob.network.known_peers().await.unwrap().len() == 2
                && carol.network.known_peers().await.unwrap().len() == 2)
                .ok_or(())
        },
    )
    .await
    .unwrap();

    println!("peers see each other");

    // alice -- bob -- carol (bob is the pivot)
    alice.add_friend(bob.me().await.unwrap()).await.unwrap();
    bob.add_friend(alice.me().await.unwrap()).await.unwrap();
    bob.add_friend(carol.me().await.unwrap()).await.unwrap();
    carol.add_friend(bob.me().await.unwrap()).await.unwrap();

    // undesirable
    alice.add_friend(carol.me().await.unwrap()).await.unwrap();
    carol.add_friend(alice.me().await.unwrap()).await.unwrap();

    println!("==> alice creates group");
    let (chat_id, _) = alice.create_group().await.unwrap();
    println!("==> alice adds bob");
    alice.add_member(chat_id, bob.public_key()).await.unwrap();

    // Bob has joined the group via his inbox topic and is a manager
    wait_for(
        Duration::from_millis(100),
        Duration::from_secs(10),
        || async {
            if let Ok(space) = bob.space(chat_id).await {
                space
                    .members()
                    .await
                    .map(|m| m.contains(&(bob.public_key().into(), Access::manage())))
                    .unwrap_or(false)
                    .ok_or(())
            } else {
                Err(())
            }
        },
    )
    .await
    .unwrap();

    println!("==> alice sends message");
    alice
        .send_message(chat_id, "alice is my name".into())
        .await
        .unwrap();

    consistency([&alice, &bob], cluster.config.clone()).await;

    assert_eq!(
        alice.get_messages(chat_id).await.unwrap(),
        bob.get_messages(chat_id).await.unwrap()
    );
    assert_eq!(alice.get_messages(chat_id).await.unwrap().len(), 1);

    println!("==> bob sends message");
    bob.send_message(chat_id, "i am bob".into()).await.unwrap();

    consistency([&alice, &bob], cluster.config.clone()).await;

    assert_eq!(
        alice.get_messages(chat_id).await.unwrap(),
        bob.get_messages(chat_id).await.unwrap()
    );
    assert_eq!(alice.get_messages(chat_id).await.unwrap().len(), 2);

    println!("==> bob adds carol");
    bob.add_member(chat_id, carol.public_key()).await.unwrap();

    // Carol has joined the group via her inbox topic and is a manager
    wait_for(
        Duration::from_millis(500),
        Duration::from_secs(10),
        || async {
            if let Ok(space) = carol.space(chat_id).await {
                space
                    .members()
                    .await
                    .map(|m| m.contains(&(carol.public_key().into(), Access::manage())))
                    .unwrap_or(false)
                    .ok_or(())
            } else {
                Err(())
            }
        },
    )
    .await
    .unwrap();

    println!("==> carol sends message");
    carol
        .send_message(chat_id, "watashi no namae wa carol".into())
        .await
        .unwrap();

    consistency([&alice, &bob, &carol], cluster.config.clone()).await;

    wait_for(
        Duration::from_millis(500),
        Duration::from_secs(10),
        || async {
            futures::future::join_all([&alice, &bob, &carol].iter().map(|n| async {
                n.space(chat_id)
                    .await
                    .unwrap()
                    .members()
                    .await
                    .unwrap()
                    .len()
            }))
            .await
            .iter()
            .all(|l| *l == 3)
            .ok_or(())
        },
    )
    .await
    .unwrap();

    wait_for(Duration::from_secs(1), Duration::from_secs(10), || async {
        let msgs = [
            alice.get_messages(chat_id).await.unwrap(),
            bob.get_messages(chat_id).await.unwrap(),
            carol.get_messages(chat_id).await.unwrap(),
        ];
        msgs.iter().all(|m| m.len() == 3).ok_or(msgs)
    })
    .await
    .ok();
    // .unwrap_or_else(|e| panic!("{:#?}", e));

    {
        for n in [&alice, &bob, &carol] {
            println!("\n\n========== {:?} ===============", n.public_key());
            // for p in [&alice, &bob, &carol] {
            //     println!("\n----- {:?} ", p.public_key());
            let p = n;

            let logs = n
                .op_store
                .get_log(&p.public_key().into(), &chat_id.into(), None)
                .await
                .unwrap()
                .unwrap();

            for (h, b) in logs {
                let payload = b
                    .map(|body| Payload::try_from_body(body))
                    .transpose()
                    .unwrap();

                let bod = match payload {
                    Some(Payload::SpaceControl(msgs)) => msgs
                        .iter()
                        .map(|m| (m.arg_type(), m.id().short()))
                        .collect::<Vec<_>>(),
                    // Some(Payload::Invitation(invitation)) => vec![invitation.id().short()],
                    _ => vec![],
                };

                println!(
                    "\n{:3} {:?} {} {:?} {:?}",
                    h.seq_num,
                    PK::from(h.public_key),
                    h.timestamp,
                    h.payload_hash.map(|h| h.short()),
                    bod,
                    // h.signature.map(|s| s.to_hex())
                );
            }
            // }
        }
    }

    let alice_messages = alice.get_messages(chat_id).await.unwrap();
    let bob_messages = bob.get_messages(chat_id).await.unwrap();
    let carol_messages = carol.get_messages(chat_id).await.unwrap();

    pretty_assertions::assert_eq!(alice_messages, bob_messages);
    pretty_assertions::assert_eq!(bob_messages, carol_messages);
}
