pub mod api {
    pub mod account;
    pub mod community;
    pub mod session;
}
pub mod utils {
    pub mod checks;
}
pub mod exceptions;

use api::account;
use api::community;
use api::session;
use serde_json::json;
use serde_json::Value;
use tauri_plugin_sql::{Migration, MigrationKind};
use utils::checks::{internet, node_status, security, system};

#[tauri::command]
async fn login_handler(server: &str, identity: &str, password: &str) -> Result<String, ()> {
    match account::login(server, identity, password, "unique_id").await {
        Ok(session_key) => {
            Ok(json!({"status": true, "session_key": session_key, "error": ""}).to_string())
        }
        Err(error) => {
            Ok(json!({"status": false, "session_key": "", "error": error.to_string()}).to_string())
        }
    }
}

#[tauri::command]
async fn session_alive(server: &str, sessionkey: &str) -> Result<Value, ()> {
    match session::alive(server, sessionkey).await {
        Ok(is_alive) => Ok(json!({"status": true, "is_alive": is_alive, "error": ""})),
        Err(error) => Ok(json!({"status": false, "is_alive": false, "error": error.to_string()})),
    }
}

#[tauri::command]
async fn get_account_profile(node: &str, token: &str) -> Result<Value, ()> {
    match account::profile(node, token).await {
        Ok(profile) => Ok(json!({"status": true, "error": json!("null"), "profile": profile})),
        Err(error) => {
            Ok(json!({"status": false, "error": error.to_string(), "profile": json!("null")}))
        }
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
    match account::register(&node, &tuta_mail, &username, &nickname, &password).await {
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
) -> Result<Value, ()> {
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
        Ok(id) => Ok(json!({"status": true, "error": json!("null"), "id": id})),
        Err(error) => Ok(json!({"status": false, "error": error.to_string(), "id": json!("null")})),
    }
}

#[tauri::command]
async fn join_community_handler(
    node: String,
    session_key: String,
    user_id: String,
    community_id: String,
) -> Result<Value, ()> {
    match community::add(&node, &session_key, &user_id, &community_id).await {
        Ok(_) => Ok(json!({"status": true, "error": json!("null")})),
        Err(error) => Ok(json!({"status": false, "error": error.to_string()})),
    }
}

// #[tauri::command]
// async fn send_message(
//     node: String,
//     token: String,
//     community_id: String,
//     message_id: String,
//     text: String,
// ) -> Result<Value, ()> {
//     match message::new(&node, &token, &community_id, &message_id, &text).await {
//         Ok(status) => Ok(json!({"status": status, "error": json!("null")})),
//         Err(error) => Ok(json!({"status": false, "error": error.to_string()})),
//     }
// }

// #[tauri::command]
// async fn get_all_messages(node: String, token: String) -> Result<Value, ()> {
//     match message::get_all(&node, &token).await {
//         Ok(messages) => Ok(json!({"status": true, "error": json!("null"), "messages": messages})),
//         Err(error) => {
//             Ok(json!({"status": false, "error": error.to_string(), "messages": json!("null")}))
//         }
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sessions_migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: r#"CREATE TABLE IF NOT EXISTS community (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            node TEXT NOT NULL,
            sequence TEXT NOT NULL,
            name TEXT NOT NULL,
            token TEXT
        );
        CREATE TABLE IF NOT EXISTS tunnels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            node TEXT NOT NULL,
            iden TEXT NOT NULL,
            token TEXT NOT NULL
        );
        "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_initial_message_table",
            sql: r#"
        CREATE TABLE IF NOT EXISTS message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            node TEXT NOT NULL,
            community_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            text TEXT NOT NULL
        );
        "#,
            kind: MigrationKind::Up,
        },
    ];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:sessions.db", sessions_migrations)
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            login_handler,
            session_alive,
            get_account_profile,
            check_internet,
            check_system,
            check_security,
            check_node,
            register_handler,
            new_community_handler,
            // send_message,
            // get_all_messages,
            join_community_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
