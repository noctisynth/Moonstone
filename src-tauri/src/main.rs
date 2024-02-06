// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api {
    pub mod account;
}
pub mod exceptions;

use api::account::{account, login, session};
use serde_json::json;

#[tauri::command]
async fn login_handler(server: &str, identity: &str, password: &str) -> Result<String, ()> {
    match login(server, identity, password, "unique_id").await {
        Ok(session_key) => {
            Ok(json!({"status": true, "session_key": session_key, "error": ""}).to_string())
        }
        Err(error) => {
            Ok(json!({"status": false, "session_key": "", "error": error.to_string()}).to_string())
        }
    }
}

#[tauri::command]
async fn session_alive(server: &str, sessionkey: &str) -> Result<String, ()> {
    match session(server, sessionkey).await {
        Ok(is_alive) => Ok(json!({"status": true, "is_alive": is_alive, "error": ""}).to_string()),
        Err(error) => {
            Ok(json!({"status": false, "is_alive": false, "error": error.to_string()}).to_string())
        }
    }
}

#[tauri::command]
async fn account_handler(server: &str, sessionkey: &str) -> Result<String, ()> {
    match account(server, sessionkey).await {
        Ok(mut json) => {
            json["status"] = json!(true);
            json["error"] = json!("null");
            Ok(json.to_string())
        }
        Err(error) => Ok(json!({"status": false, "error": error.to_string()}).to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login_handler,
            session_alive,
            account_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
