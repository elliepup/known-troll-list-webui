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

use utilities::app::App;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = App::new().unwrap();

    match app.get_trolls() {
        Some(trolls) => {
            for troll in trolls {
                println!("{}", troll);
            }
        },
        None => {
            println!("No trolls found");
        }
    }

    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
