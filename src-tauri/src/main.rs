// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{ Enigo, MouseButton };

struct EnigoState(Enigo);
unsafe impl Send for EnigoState {}
unsafe impl Sync for EnigoState {}

#[tauri::command]
fn mouse_button(double: bool, button_id: i32, enigo: tauri::State<EnigoState>) -> Result<(), String> {
  let button: MouseButton = match button_id {
    0 => Ok(MouseButton::Left),
    1 => Ok(MouseButton::Middle),
    2 => Ok(MouseButton::Right),
    _ => Err("Invalid button ID")
  }?;
  enigo.lock_mut().0.mouse_click(button);
  if double { enigo.lock_mut().0.mouse_click(button) };

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .manage(EnigoState(Enigo::new()))
    .invoke_handler(tauri::generate_handler![mouse_button])
    .plugin(tauri_plugin_store::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
