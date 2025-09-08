use std::{
    fs,
    path::PathBuf,
    process::{self, Command, Stdio},
};

use anyhow::{Result, anyhow};
use clap::Parser;

use crate::{
    foundation::{
        create_temporary_name, get_prefix, kdeglobals::parse_config, registry::HkeyCollection,
    },
    theme::{create_classic_theme_registry, create_colors_registry, create_dark_mode_registry},
};

pub mod foundation;
pub mod theme;

/// Breezify Wine with ease
#[derive(Parser, Debug)]
#[command(version, about)]
struct App {
    prefix: Option<PathBuf>,
}

fn main() {
    match run(App::parse()) {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}

fn run(app: App) -> Result<i32> {
    let prefix = app
        .prefix
        .or_else(|| get_prefix().ok())
        .ok_or(anyhow!("Prefix can't be specified"))?;
    if !fs::exists(&prefix)? {
        return Err(anyhow!("Prefix does not exists"));
    }

    let c = prefix.join("dosdevices/c:");
    if !fs::exists(&c)? {
        return Err(anyhow!("Wine is not initialized"));
    }

    let name = c.join(
        create_temporary_name("reg").or_else(|_| Err(anyhow!("Can't generate temporary name")))?,
    );
    let config = parse_config().unwrap();
    let jar = HkeyCollection(vec![
        create_classic_theme_registry(),
        create_dark_mode_registry(
            config
                .get("General", "ColorScheme")
                .map(|it| it.to_lowercase().contains("dark"))
                .unwrap_or_default(),
        ),
        create_colors_registry(&config),
    ]);

    fs::write(&name, jar.to_string())?;
    let status = Command::new("wine")
        .env("WINEPREFIX", prefix)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("regedit")
        .arg(&name)
        .status()?;

    Ok(status.code().unwrap_or(0))
}
