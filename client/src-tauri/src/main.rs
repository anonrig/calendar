// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTrayEvent, WindowEvent};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                // Restore window size/state.
                window.restore_state(StateFlags::all())?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide windows instead of closing on macOS.
                    if cfg!(target_os = "macos") {
                        api.prevent_close();
                        event.window().hide().unwrap();
                    }
                }
                WindowEvent::Resized(_) => {
                    // Persist window state
                    event.window().app_handle().save_window_state(StateFlags::all()).unwrap();
                }
                _ => {}
            }
        })
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::LeftClick { .. } = event {
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
