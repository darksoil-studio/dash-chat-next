use std::str::FromStr;

use node::Node;
use p2panda_core::{PrivateKey, PublicKey};
use tauri::{Manager, State};
use tauri_plugin_log::log::LevelFilter;

use crate::{chat::ChatId, message::ChatMessage};

#[tauri::command]
async fn me(node: State<'_, Node>) -> Result<String, String> {
    Ok(node.public_key().to_string())
}

#[tauri::command]
async fn create_group(name: &str, node: State<'_, Node>) -> Result<ChatId, String> {
    match node.create_group().await {
        Ok((chat_id, _)) => {
            println!("[rust] created group, chat_id: {}", chat_id);
            Ok(chat_id)
        }
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
async fn add_member(
    chat_id: ChatId,
    public_key: PublicKey,
    node: State<'_, Node>,
) -> Result<(), String> {
    match node.add_member(chat_id, public_key).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error adding member: {err:?}")),
    }
}

#[tauri::command]
async fn get_members(chat_id: ChatId, node: State<'_, Node>) -> Result<Vec<String>, String> {
    match node.get_members(chat_id).await {
        Ok(members) => Ok(members
            .into_iter()
            .map(|(actor_id, _access)| actor_id.to_string())
            .collect()),
        Err(err) => Err(format!("Error getting participants: {err:?}")),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn send_message(
    chat_id: ChatId,
    message: ChatMessage,
    node: State<'_, Node>,
) -> Result<(), String> {
    match node.send_message(chat_id, message).await {
        Ok(_) => Ok(()),
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

mod chat;
mod forge;
mod message;
mod node;
mod operation;
pub mod spaces;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(LevelFilter::Error)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            me,
            create_group,
            join_group,
            add_member,
            get_members,
            send_message,
            get_messages
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let private_key = PrivateKey::new();
                let node = Node::new(private_key, node::Config::default()).await;

                match node {
                    Ok(node) => {
                        handle.manage(node);
                    }
                    Err(err) => {
                        tracing::error!("Error creating the node: {err:?}");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
