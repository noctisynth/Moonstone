// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn login(username: &str, password: &str) -> bool {
    println!("Hello, {}! You've been greeted from Rust!", username);
    println!("Found password {}", password);
    true
}

#[tauri::command]
fn session_alive(session: &str) -> bool {
    println!("检测到session: {}", session);
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, session_alive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
