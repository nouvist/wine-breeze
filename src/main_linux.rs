use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio, exit},
};

use clap::Parser;

use crate::App;

const WIN32_BINARY: &[u8] = include_bytes!(env!("WIN32_PATH"));
const WIN32_HASH: u32 = parse_u32(env!("WIN32_HASH"));

#[inline]
pub fn main() {
    let mut args = env::args();
    if args.any(|it| it == "-h" || it == "--help" || it == "-V" || it == "--version") {
        App::parse();
        return;
    }

    let debug = env::var("WINEDEBUG").unwrap_or("-all".to_owned());
    let prefix = env::var("WINEPREFIX")
        .map(PathBuf::from)
        .unwrap_or(env::home_dir().unwrap().join(".wine"));
    let system32 = prefix
        .join("dosdevices")
        .join("c:")
        .join("windows")
        .join("system32");
    let filename = system32.join("wine-breeze.exe");

    if !fs::exists(&system32).unwrap() {
        println!("Error: Wine has not been initialized yet");
        exit(1);
    }

    let mut should_upgrade = true;
    if fs::exists(&filename).unwrap() {
        let hash = crc32fast::hash(&fs::read(&filename).unwrap());
        should_upgrade = hash != WIN32_HASH;
        if should_upgrade {
            fs::remove_file(&filename).unwrap();
            should_upgrade = true;
        }
    }

    if should_upgrade {
        fs::write(&filename, WIN32_BINARY).unwrap();
    }

    let mut child = Command::new("wine")
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .env("WINEPREFIX", prefix)
        .env("WINEDEBUG", debug)
        .arg("c:\\windows\\system32\\wine-breeze.exe")
        .args(args)
        .spawn()
        .unwrap();

    exit(child.wait().unwrap().code().unwrap());
}

const fn parse_u32(s: &str) -> u32 {
    let str = s.as_bytes();
    let mut index = 0;
    let mut result: u32 = 0;

    while index < str.len() {
        let b = str[index];
        assert!(b >= b'0' && b <= b'9');
        result = result * 10 + (b - b'0') as u32;
        index += 1;
    }

    result
}
