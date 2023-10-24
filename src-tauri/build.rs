use std::{env, fs::File, io::Write, path::Path, process::Command};

fn main() {
    tauri_build::build();
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("cant get git commit info");

    let git_commit = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("git_commit")).unwrap();
    f.write_all(git_commit.trim().as_bytes()).unwrap();

    let build_time = chrono::Utc::now().to_rfc3339();

    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("build_time")).unwrap();
    f.write_all(build_time.as_bytes()).unwrap();
}
