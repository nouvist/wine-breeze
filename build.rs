use std::{env, fs, path::Path};

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let profile = env::var("PROFILE").unwrap();

    eprintln!(">> {os}");
    eprintln!(">> {profile}");
    if os != "linux" {
        return;
    }

    let win_target = "x86_64-pc-windows-gnu";
    let win_path = Path::new("target")
        .join(win_target)
        .join(&profile)
        .join("wine-breeze.exe");

    let data =
        fs::read(&win_path).unwrap_or_else(|_| panic!("Failed to read binary at {:?}", win_path));

    let path = format!("../{}", win_path.as_os_str().to_str().unwrap());
    let hash = crc32fast::hash(&data);

    println!("cargo:rustc-env=WIN32_PATH={}", path);
    println!("cargo:rustc-env=WIN32_HASH={}", hash);
}
