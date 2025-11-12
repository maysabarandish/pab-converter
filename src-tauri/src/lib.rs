pub mod converter;

use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn convert_ohh_content(content: String) -> Result<String, String> {
    converter::convert_ohh_file(&content)
}

#[tauri::command]
fn convert_ohh_file_path(file_path: String) -> Result<String, String> {
    let content =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    converter::convert_ohh_file(&content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            convert_ohh_content,
            convert_ohh_file_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
