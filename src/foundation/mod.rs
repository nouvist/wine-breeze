use std::{env, fmt::Write, path::PathBuf};

use anyhow::{Result, anyhow};

pub mod config;
pub mod kdeglobals;
pub mod registry;

pub fn get_prefix() -> Result<PathBuf> {
    env::var("WINEPREFIX")
        .map(|it| PathBuf::from(it))
        .or_else(|_| {
            env::home_dir()
                .ok_or(anyhow!("Home directory not found"))
                .map(|home| home.join(".wine"))
        })
}

pub fn create_temporary_name(ext: &str) -> Result<String> {
    let mut buf = [0u8; 16];
    let mut name = String::with_capacity(13 + 16 * 2 + 1 + ext.len());
    getrandom::fill(&mut buf).map_err(|_| anyhow!("Unable to create random buffer"))?;

    name.push_str("__winebreeze-");
    for byte in &buf {
        write!(&mut name, "{byte:02x}")?;
    }
    name.push('.');
    name.push_str(ext);

    Ok(name)
}
