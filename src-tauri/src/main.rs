// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gug_lib::api;
use tauri::Manager;

struct ApiConfig {
    port: u16,
}

#[tokio::main]
async fn main() {
    const API_PORT: u16 = 3000;

    tokio::spawn(async move {
        api::run_server(API_PORT).await;
    });

    println!("🚀 Axum listening on: {}", API_PORT);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(ApiConfig { port: API_PORT });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
