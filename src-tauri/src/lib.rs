// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Button, Enigo, Mouse, Settings};

#[tauri::command]
fn mouse_button(double: bool, button_id: i32) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).or(Err("Failed to load enigo"))?;
    let button: Button = match button_id {
        0 => Ok(Button::Left),
        1 => Ok(Button::Middle),
        2 => Ok(Button::Right),
        _ => Err("Invalid button ID"),
    }?;
    enigo
        .button(button, enigo::Direction::Click)
        .or(Err("Failed to Click"))?;
    if double {
        enigo
            .button(button, enigo::Direction::Click)
            .or(Err("Failed to Click"))?;
    };

    Ok(())
}

#[tauri::command]
fn mouse_pos() -> Result<(i32, i32), String> {
    let enigo = Enigo::new(&Settings::default()).or(Err("Failed to load enigo"))?;
    let location = enigo.location().or(Err("Failed to get location"))?;
    Ok(location)
}

#[tauri::command]
fn set_mouse_pos(location: (i32, i32)) -> Result<(), String> {
    let (x, y) = location;
    let mut enigo: Enigo = Enigo::new(&Settings::default()).or(Err("Failed to load enigo"))?;
    enigo
        .move_mouse(x, y, enigo::Coordinate::Abs)
        .or(Err("Failed to move mouse"))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            mouse_button,
            mouse_pos,
            set_mouse_pos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
