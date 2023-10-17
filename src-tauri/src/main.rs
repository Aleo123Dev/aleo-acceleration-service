// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(internal_output_capture)]

mod auto_start;
mod config;
mod logger;
mod rpc;
mod tls;

use std::io::BufReader;

use tauri::{
    generate_handler, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tauri_plugin_positioner::{Position, WindowExt};

use logger::get_logs;
use rpc::{run_rpc_server, stop_rpc_server};

const MENU_ITEM_AUTO_START: &str = "auto_start";
const MENU_ITEM_QUIT: &str = "quit";
const MENU_ITEM_ABOUT: &str = "about";

#[tokio::main]
async fn main() {
    logger::setup_logger();
    rpc::run_rpc_server();
    log::info!("app started!");

    #[cfg(not(target_os = "windows"))]
    tokio::spawn(async {
        let mut piped_stdout =
            <capture_stdio::PipedStdout as capture_stdio::Capture>::capture().unwrap();
        let mut output = String::new();
        loop {
            let mut buf_reader = BufReader::new(piped_stdout.get_reader());
            std::io::BufRead::read_line(&mut buf_reader, &mut output).unwrap();
            log::info!("{}", output);
        }
    });

    let quit = CustomMenuItem::new(MENU_ITEM_QUIT, "Quit").accelerator("Cmd+Q");
    let auto_start = match auto_start::AUTO_LAUNCH.as_ref() {
        Some(v) => {
            let enabled = v.is_enabled();
            if enabled.is_err() {
                CustomMenuItem::new(MENU_ITEM_AUTO_START, "Start at login").disabled()
            } else {
                if enabled.unwrap() {
                    CustomMenuItem::new(MENU_ITEM_AUTO_START, "Start at login").selected()
                } else {
                    CustomMenuItem::new(MENU_ITEM_AUTO_START, "Start at login")
                }
            }
        }
        None => CustomMenuItem::new(MENU_ITEM_AUTO_START, "Start at login").disabled(),
    };
    let about = CustomMenuItem::new(MENU_ITEM_ABOUT, "About");

    let system_tray_menu = SystemTrayMenu::new()
        .add_item(auto_start)
        .add_item(about)
        .add_item(quit);
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(generate_handler![get_logs, stop_rpc_server, run_rpc_server])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .setup(|app| {
            // hide dock icon on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
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
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        println!("system tray received quit");
                        std::process::exit(0);
                    }
                    "about" => {
                        let handle = app.app_handle();
                        tauri::WindowBuilder::new(
                            &handle,
                            "about",
                            tauri::WindowUrl::App("/about".into()),
                        )
                        .inner_size(400.0, 300.0)
                        .build()
                        .unwrap();
                    }
                    "auto_start" => {
                        match auto_start::AUTO_LAUNCH.as_ref() {
                            Some(v) => {
                                let enabled = v.is_enabled();
                                if enabled.is_err() {
                                    log::warn!(
                                        "failed to get auto start menu item enabled state: {}",
                                        enabled.as_ref().err().unwrap()
                                    );
                                    if let Err(e) = app
                                        .tray_handle()
                                        .get_item(MENU_ITEM_AUTO_START)
                                        .set_enabled(false)
                                    {
                                        log::warn!(
                                            "failed to set auto start menu item to disabled: {}",
                                            e
                                        )
                                    }
                                }

                                if enabled.is_ok() && enabled.unwrap() {
                                    if let Err(e) = v.disable() {
                                        log::warn!("failed to disable auto start: {}", e);
                                    } else {
                                        if let Err(e) = app
                                            .tray_handle()
                                            .get_item(MENU_ITEM_AUTO_START)
                                            .set_selected(false)
                                        {
                                            log::warn!(
                                            "failed to set auto start menu item to disabled: {}",
                                            e
                                        )
                                        }
                                    }
                                } else {
                                    if let Err(e) = v.enable() {
                                        log::warn!("failed to enable auto start: {}", e);
                                    } else {
                                        if let Err(e) = app
                                            .tray_handle()
                                            .get_item(MENU_ITEM_AUTO_START)
                                            .set_selected(true)
                                        {
                                            log::warn!(
                                            "failed to set auto start menu item to disabled: {}",
                                            e
                                        )
                                        }
                                    }
                                }
                            }
                            None => {}
                        };
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
