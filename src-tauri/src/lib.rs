use std::str::FromStr;

use node::Node;
use p2panda_core::PrivateKey;
use tauri::{Manager, State};
use tauri_plugin_log::log::LevelFilter;

use crate::chat::ChatId;

#[tauri::command]
async fn create_chat(node: State<'_, Node>) -> Result<String, String> {
    match node.create_chat().await {
        Ok((chat_id, _)) => Ok(chat_id.to_string()),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}
#[tauri::command]
async fn join_chat(chat_id: &str, node: State<'_, Node>) -> Result<(), String> {
    match node.join_chat(ChatId::from_str(chat_id).unwrap()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn send_message(chat_id: &str, message: &str, node: State<'_, Node>) -> Result<(), String> {
    match node
        .send_message(ChatId::from_str(chat_id).unwrap(), message.to_string())
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error sending message: {err:?}")),
    }
}

#[tauri::command]
async fn get_messages(chat_id: &str, node: State<'_, Node>) -> Result<Vec<String>, String> {
    match node.get_messages(ChatId::from_str(chat_id).unwrap()).await {
        Ok(messages) => Ok(messages),
        Err(err) => Err(format!("Failed to get messages: {err:?}")),
    }
}

mod chat;
mod forge;
mod node;
mod operation;
pub mod spaces;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![send_message, get_messages])
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
