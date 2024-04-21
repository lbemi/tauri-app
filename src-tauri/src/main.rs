// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "linux")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod api;
mod services;
mod error;

use api::pods::pod;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, 
            pod::list_pods,
            pod::get_pod_by_name,
            pod::delete_pod_by_name,
            pod::create_pod,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
