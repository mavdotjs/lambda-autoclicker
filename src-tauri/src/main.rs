// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{ Enigo, MouseButton, MouseControllable };

#[tauri::command]
fn mouse_button(double: bool, button_id: i32) -> Result<(), String> {
  let mut enigo = Enigo::new();
  let button: MouseButton = match button_id {
    0 => Ok(MouseButton::Left),
    1 => Ok(MouseButton::Middle),
    2 => Ok(MouseButton::Right),
    _ => Err("Invalid button ID")
  }?;
  enigo.mouse_click(button);
  if double { enigo.mouse_click(button) };

  Ok(())
}

#[tauri::command]
fn mouse_pos() -> ( i32, i32 ) {
  let mut enigo = Enigo::new();
  return enigo
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![mouse_button, mouse_pos])
    .plugin(tauri_plugin_store::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
