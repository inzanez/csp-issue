// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize)]
pub struct BridgeError(String);

impl<E> From<E> for BridgeError
where
    E: Error + Send + Sync + Display,
{
    fn from(value: E) -> Self {
        BridgeError(value.to_string())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn verify_instance_uri(instance_uri: &str) -> Result<(), BridgeError> {
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![verify_instance_uri])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
