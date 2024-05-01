// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![get_exec_path, get_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::path::PathBuf;

#[tauri::command]
fn get_exec_path() -> Result<PathBuf, String> {
    match std::env::current_exe() {
        Ok(path) => return Ok(path),
        Err(error) => return Err(format!("{error}")),
    }
}

use std::fs::read;
use sha256::digest;

#[tauri::command]
async fn get_hash(file_path: String) -> Result<String, String> {
    let bytes = read(file_path).unwrap();
    let hash = digest(&bytes);
    Ok(hash)
}