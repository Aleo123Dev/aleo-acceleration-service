// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod rpc;

use tauri::{
    generate_handler, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tauri_plugin_positioner::{Position, WindowExt};

use logger::get_logs;

#[tokio::main]
async fn main() {
    logger::setup_logger();
    rpc::run_rpc_server();
    log::info!("app started!");

    let quit = CustomMenuItem::new("quit".to_string(), "Quit Appp").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(generate_handler![get_logs])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .setup(|app| {
            // hide dock icon on macOS
            if cfg!(target_os = "macos") {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            Ok(())
        })
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        println!("system tray received quit");
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                #[allow(unused_unsafe)]
                #[cfg(not(target_os = "macos"))]
                {
                    event.window().hide().unwrap();
                }

                #[allow(unused_unsafe)]
                #[cfg(target_os = "macos")]
                unsafe {
                    tauri::AppHandle::hide(&event.window().app_handle()).unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[allow(unused)]
    app.run(|app, event| {});
}
