// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use database::Database;
use tauri::Manager;

#[tauri::command]
fn add_item(db: tauri::State<Database>, content: String) -> Result<(), String> {
    db.add_item(&content).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_items(db: tauri::State<Database>) -> Result<Vec<(i64, String)>, String> {
    db.get_items().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db = Database::new().expect("Failed to create database");
            db.init().expect("Failed to initialize database");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_item, get_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
