// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod data{
    pub mod api_models;
    pub mod local_models;
}

mod utilities{
    pub mod app;
    pub mod data_manager;
    pub mod file_manager;
}

use tauri::State;
use utilities::app::App;

// TODO - Return Troll instead of string in the future, easier to test strings for now
#[tauri::command]
async fn get_trolls_by_name(tool: State<'_, App>, name: String) -> Result<Vec<String>, String> {
    tool.get_trolls_by_name(&name)
        .iter()
        .map(|troll| Ok(format!("{} {}", troll.first_name, troll.last_name)))
        .collect()
}

#[tauri::command]
async fn get_troll_by_id(tool: State<'_, App>, id: i32) -> Result<String, String> {
    tool.get_troll_by_id(id)
        .map(|troll| format!("{} {}", troll.first_name, troll.last_name))
        .ok_or_else(|| "Troll not found".to_string())
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = App::new().unwrap();
    app.initialize().unwrap();

    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![greet, get_troll_by_id, get_trolls_by_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
