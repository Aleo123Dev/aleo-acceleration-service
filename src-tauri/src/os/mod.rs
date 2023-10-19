use os_info::Info;
#[cfg(target_os = "windows")]
use os_info::Version::Semantic;

#[tauri::command]
pub fn os_info() -> Info {
    let info = os_info::get();

    info

    // // Print full information:
    // println!("OS information: {info}");

    // // Print information separately:
    // println!("Type: {}", info.os_type());
    // println!("Version: {}", info.version());
    // println!("Bitness: {}", info.bitness());
    // println!("Architecture: {:#?}", info.architecture());
}

#[tauri::command]
pub fn is_win11() -> bool {
    #[cfg(target_os = "windows")]
    {
        let info = os_info::get();

        if let Semantic(major, _minor, patch) = info.version() {
            if major < &10 {
                return false;
            }

            if patch < &20000 {
                return false;
            }
            return true;
        };
    }

    return false;
}
