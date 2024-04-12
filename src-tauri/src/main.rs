// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use tauri::Manager;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .unwrap();

    app.run(|app, event| {
        match event {
            tauri::RunEvent::Ready => {
                // start sending events
                let app_clone = app.clone();
                std::thread::spawn(move || loop {
                    std::thread::sleep(Duration::from_millis(10));
                    app_clone.emit("hello", ()).unwrap();
                });
            }
            _ => {}
        }
    });
}
