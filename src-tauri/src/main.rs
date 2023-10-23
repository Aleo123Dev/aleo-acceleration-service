// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(internal_output_capture)]

mod auto_start;
mod config;
mod logger;
mod os;
mod rpc;
mod service;
mod tls;

use anyhow::Result;
use std::{io::BufReader, process::Command};

use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

#[cfg(target_os = "windows")]
use window_vibrancy::apply_mica;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use window_shadows::set_shadow;

use tauri::{
    generate_handler, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

use config::{has_password, input_password, set_password, try_password};
use logger::get_logs;
use os::{is_win11, os_info};
use rpc::{run_rpc_server, stop_rpc_server};
use service::app::get_server_url;

const MENU_ITEM_AUTO_START: &str = "auto_start";
const MENU_ITEM_QUIT: &str = "quit";
const MENU_ITEM_ABOUT: &str = "about";
const MENUITEM_COPY_ADDR: &str = "copy server address";
const MENUITEM_SHOW: &str = "show window";

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() {
    logger::setup_logger();
    log::info!("app started!");

    #[cfg(not(target_os = "windows"))]
    tokio::spawn(async {
        let mut piped_stdout =
            <capture_stdio::PipedStdout as capture_stdio::Capture>::capture().unwrap();
        let mut buf_reader = BufReader::new(piped_stdout.get_reader());
        loop {
            let mut output = String::new();
            std::io::BufRead::read_line(&mut buf_reader, &mut output).unwrap();
            log::info!("{}", output);
        }
    });

    let show = CustomMenuItem::new(MENUITEM_SHOW, "show window");
    let copy_addr = CustomMenuItem::new(MENUITEM_COPY_ADDR, "copy server address");
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
        .add_item(show)
        .add_item(auto_start)
        .add_item(copy_addr)
        .add_item(about)
        .add_item(quit);
    let app = tauri::Builder::default()
        .invoke_handler(generate_handler![
            get_logs,
            stop_rpc_server,
            run_rpc_server,
            is_win11,
            os_info,
            get_server_url,
            has_password,
            input_password,
            set_password,
            try_password
        ])
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .setup(|app| {
            // hide dock icon on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                let _ = window.set_decorations(true);
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            }

            // #[cfg(target_os = "windows")]
            // apply_blur(&window, Some((18, 18, 18, 125)))
            //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            #[cfg(target_os = "windows")]
            if is_win11() {
                apply_mica(&window, None)
                    .expect("Unsupported platform! 'apply_mica' is only supported on Windows11");
            }

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();

                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                MENUITEM_SHOW => {
                    let handle = app.app_handle();
                    if let Some(window) = handle.get_window("main") {
                        let _ = window.show();
                    };
                }
                MENU_ITEM_QUIT => {
                    println!("system tray received quit");
                    std::process::exit(0);
                }
                MENU_ITEM_ABOUT => {
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
                MENUITEM_COPY_ADDR => {
                    if let Ok(addr) = get_server_url() {
                        match ClipboardContext::new() {
                            Err(e) => {
                                log::error!("faied to get clipboard context: {}", e);
                            }
                            Ok(mut ctx) => {
                                if let Err(e) = ctx.set_contents(addr) {
                                    log::error!("faied to write clipboard: {}", e);
                                }
                            }
                        };
                    }
                }
                MENU_ITEM_AUTO_START => {
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
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "main" {
                    //prevent main window close
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
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    //store app handle
    {
        let apphandle = app.app_handle();

        let mut handle = service::app::APP_HANDLE.lock().unwrap();

        *handle = Some(apphandle.clone());

        drop(handle);
    }

    #[cfg(target_os = "macos")]
    if let Ok(running) = is_another_instance_running(&app.config().tauri.bundle.identifier) {
        if running {
            return;
        }
    }

    #[allow(unused)]
    app.run(|app, event| {});
}

fn is_another_instance_running(bundle_identifier: &str) -> Result<bool> {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg("-f")
        .arg(&bundle_identifier)
        .output()?;

    Ok(output.status.success())
}
