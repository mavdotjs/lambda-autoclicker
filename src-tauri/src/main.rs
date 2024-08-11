// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn click_left() {

}

#[tauri::command]
fn click_right() {

}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![click_left, click_right])
    .plugin(tauri_plugin_store::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
