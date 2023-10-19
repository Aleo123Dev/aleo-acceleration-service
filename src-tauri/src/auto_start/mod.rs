use std::sync::Arc;

use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use lazy_static::lazy_static;
use tauri::utils::platform::current_exe;

use crate::config::consts::APP_NAME;

lazy_static! {
    pub static ref AUTO_LAUNCH: Arc<Option<AutoLaunch>> = Arc::new(init_auto_launch(APP_NAME));
}

fn init_auto_launch(name: &str) -> Option<AutoLaunch> {
    let mut builder = AutoLaunchBuilder::new();
    builder.set_app_name(name);

    builder.set_use_launch_agent(false);

    let current_exe = current_exe().ok()?;

    #[cfg(windows)]
    builder.set_app_path(&current_exe.display().to_string());
    #[cfg(target_os = "macos")]
    {
        // on macOS, current_exe gives path to /Applications/Example.app/MacOS/Example
        // but this results in seeing a Unix Executable in macOS login items
        // It must be: /Applications/Example.app
        // If it didn't find exactly a single occurance of .app, it will default to
        // exe path to not break it.
        let exe_path = current_exe
            .canonicalize()
            .unwrap_or_default()
            .display()
            .to_string();
        if exe_path == "" {
            return None;
        }
        let parts: Vec<&str> = exe_path.split(".app/").collect();
        let app_path = if parts.len() == 2 {
            format!("{}.app", parts.get(0).unwrap().to_string())
        } else {
            exe_path
        };
        // log::info!("auto_start path {}", &app_path);
        builder.set_app_path(&app_path);
    }
    #[cfg(target_os = "linux")]
    return None;
    // if let Some(appimage) = app
    //     .env()
    //     .appimage
    //     .and_then(|p| p.to_str().map(|s| s.to_string()))
    // {
    //     builder.set_app_path(&appimage);
    // } else {
    //     builder.set_app_path(&current_exe.display().to_string());
    // }

    builder.build().ok()
}
