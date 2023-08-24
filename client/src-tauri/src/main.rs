// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use tauri::tray::ClickType;
use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                // Restore window size/state.
                window.restore_state(StateFlags::all())?;
            }

            // The following line is required for when opening the application from scratch.
            if let Some(url) = env::args().skip(1).collect::<Vec<_>>().first() {
                app.emit_all("deeplink-received", url).unwrap();
            }

            Ok(())
        })
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            RunEvent::TrayIconEvent(tray_event) => {
                if tray_event.click_type == ClickType::Left {
                    if let Some(window) = app_handle.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }
            RunEvent::Opened { urls } => {
                // Listen for deeplink events
                if let Some(window) = app_handle.get_window("main") {
                    window.set_focus().unwrap();
                }

                if let Some(url) = urls.first() {
                    app_handle.emit_all("deeplink-received", url).unwrap();
                }
            }
            _ => {}
        });
}
