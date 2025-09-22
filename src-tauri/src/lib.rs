use std::time::SystemTime;

use dashchat_node::*;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_log::log::{Level, LevelFilter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatOverview {
    pub chat_id: ChatId,
    pub name: String,
    pub member_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub content: String,
    pub author: PK,
    pub timestamp: u64,
}

#[tauri::command]
async fn me(node: State<'_, Node>) -> Result<MemberCode, String> {
    let member = node.me().await.map_err(|e| e.to_string())?;
    Ok(member.into())
}

#[tauri::command]
async fn create_group(name: &str, node: State<'_, Node>) -> Result<ChatId, String> {
    match node.create_group().await {
        Ok((chat_id, _)) => Ok(chat_id),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}

#[tauri::command]
async fn join_group(chat_id: ChatId, node: State<'_, Node>) -> Result<(), String> {
    match node.join_group(chat_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}

#[tauri::command]
async fn get_groups(node: State<'_, Node>) -> Result<Vec<ChatOverview>, String> {
    let mut overviews = vec![];
    let chat_ids = node.get_groups().await.map_err(|e| e.to_string())?;

    for chat_id in chat_ids {
        let overview = ChatOverview {
            chat_id,
            name: chat_id.to_string(),
            member_count: node
                .get_members(chat_id)
                .await
                .map_err(|e| e.to_string())?
                .len(),
        };
        overviews.push(overview);
    }

    Ok(overviews)
}

#[tauri::command]
async fn add_member(chat_id: ChatId, pubkey: PK, node: State<'_, Node>) -> Result<(), String> {
    match node.add_member(chat_id, pubkey).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error adding member: {err:?}")),
    }
}

#[tauri::command]
async fn get_members(chat_id: ChatId, node: State<'_, Node>) -> Result<Vec<PK>, String> {
    match node.get_members(chat_id).await {
        Ok(members) => Ok(members
            .into_iter()
            .map(|(actor_id, _access)| PK::try_from(actor_id).unwrap())
            .collect()),
        Err(err) => Err(format!("Error getting participants: {err:?}")),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn send_message(
    chat_id: ChatId,
    message: String,
    node: State<'_, Node>,
) -> Result<ChatMessage, String> {
    let message = ChatMessage {
        content: message,
        author: node.public_key(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time from operation system")
            .as_secs(),
    };
    match node.send_message(chat_id, message.clone()).await {
        Ok(_) => Ok(message),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}

#[tauri::command]
async fn get_messages(chat_id: ChatId, node: State<'_, Node>) -> Result<Vec<ChatMessage>, String> {
    match node.get_messages(chat_id).await {
        Ok(messages) => Ok(messages),
        Err(err) => Err(format!("Failed to get messages: {err:?}")),
    }
}

// Friend management commands
#[tauri::command]
async fn add_friend(friend_code: MemberCode, node: State<'_, Node>) -> Result<PK, String> {
    match node.add_friend(friend_code.into()).await {
        Ok(public_key) => Ok(public_key),
        Err(err) => Err(format!("Error adding friend: {err:?}")),
    }
}

#[tauri::command]
async fn get_friends(node: State<'_, Node>) -> Result<Vec<PK>, String> {
    match node.get_friends().await {
        Ok(friends) => Ok(friends),
        Err(err) => Err(format!("Error getting friends: {err:?}")),
    }
}

#[tauri::command]
async fn remove_friend(public_key: PK, node: State<'_, Node>) -> Result<(), String> {
    match node.remove_friend(public_key).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error removing friend: {err:?}")),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .filter(|m| {
                    m.level() <= Level::Error
                        || (m.target().starts_with("dashchat_node") && m.level() <= Level::Info)
                })
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            me,
            create_group,
            join_group,
            get_groups,
            add_member,
            get_members,
            send_message,
            get_messages,
            add_friend,
            get_friends,
            remove_friend
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let private_key = dashchat_node::PrivateKey::new();

                let node = Node::new(private_key, dashchat_node::NodeConfig::default(), None).await;

                match node {
                    Ok(node) => {
                        handle.manage(node);
                    }
                    Err(err) => {
                        println!("*** Error creating the node: {err:?}");
                        tracing::error!("Error creating the node: {err:?}");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
