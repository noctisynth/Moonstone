pub mod api {
    pub mod account;
    pub mod community;
    pub mod session;
}
pub mod utils {
    pub mod checks;
}
pub mod exceptions;

use api::account::{account, login, register};
use api::community;
use api::session::alive;
use serde_json::json;
use utils::checks::{internet, node_status, security, system};

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
    match alive(server, sessionkey).await {
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

#[tauri::command]
async fn check_internet() -> Result<bool, String> {
    Ok(internet())
}

#[tauri::command]
async fn check_system() -> Result<bool, String> {
    Ok(system())
}

#[tauri::command]
async fn check_security() -> Result<bool, String> {
    Ok(security())
}

#[tauri::command]
async fn check_node(node: String) -> Result<String, ()> {
    match node_status(&node).await {
        Ok(_) => Ok(json!({"status": true, "error": json!("null")}).to_string()),
        Err(error) => Ok(json!({"status": false, "error": error.to_string()}).to_string()),
    }
}

#[tauri::command]
async fn register_handler(
    node: String,
    tuta_mail: String,
    username: String,
    nickname: String,
    password: String,
) -> Result<String, ()> {
    match register(&node, &tuta_mail, &username, &nickname, &password).await {
        Ok(_) => Ok(json!({"status": true, "error": json!("null")}).to_string()),
        Err(error) => Ok(json!({"status": false, "error": error.to_string()}).to_string()),
    }
}

#[tauri::command]
async fn new_community_handler(
    node: String,
    session_key: String,
    name: String,
    security_level: i32,
    cross_origin: bool,
    token: Option<String>,
) -> Result<String, ()> {
    match community::new(
        &node,
        &session_key,
        &name,
        security_level,
        cross_origin,
        token,
    )
    .await
    {
        Ok(id) => Ok(json!({"status": true, "error": json!("null"), "id": id}).to_string()),
        Err(error) => Ok(
            json!({"status": false, "error": error.to_string(), "id": json!("null")}).to_string(),
        ),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            login_handler,
            session_alive,
            account_handler,
            check_internet,
            check_system,
            check_security,
            check_node,
            register_handler,
            new_community_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
