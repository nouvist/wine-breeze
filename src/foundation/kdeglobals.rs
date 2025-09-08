use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

use crate::foundation::config::Config;

pub fn get_paths() -> Result<Vec<PathBuf>, io::Error> {
    let mut result = Vec::<PathBuf>::new();
    let xdg_config = env::var("XDG_CONFIG_DIRS").unwrap_or("".to_owned());

    for dir in xdg_config.split(":") {
        let path = PathBuf::from(dir).join("kdeglobals");
        if fs::exists(&path)? {
            result.push(path);
        }
    }

    if let Some(home) = env::home_dir() {
        let fallback = home.join(".config").join("kdeglobals");
        if !result.contains(&fallback) && fs::exists(&fallback)? {
            result.push(fallback);
        }
    }

    Ok(result)
}

pub fn parse_config() -> Result<Config, io::Error> {
    let mut config = Config::new();
    for path in get_paths()? {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        config.try_parse(reader)?;
    }

    Ok(config)
}
