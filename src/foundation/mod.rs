use std::fmt::{self, Write};

use thiserror::Error;

pub mod config;
pub mod kdeglobals;
pub mod registry;

pub fn create_temporary_name(ext: &str) -> Result<String, NameError> {
    let mut buf = [0u8; 16];
    let mut name = String::with_capacity(13 + 16 * 2 + 1 + ext.len());
    getrandom::fill(&mut buf)?;

    name.push_str("__winebreeze-");
    for byte in &buf {
        write!(&mut name, "{byte:02x}")?;
    }
    name.push('.');
    name.push_str(ext);

    Ok(name)
}

#[derive(Error, Debug)]
pub enum NameError {
    #[error("Unable to create random buffer")]
    Random(getrandom::Error),
    #[error("Unable to format")]
    Format(fmt::Error),
}

impl From<getrandom::Error> for NameError {
    fn from(value: getrandom::Error) -> Self {
        Self::Random(value)
    }
}

impl From<fmt::Error> for NameError {
    fn from(value: fmt::Error) -> Self {
        Self::Format(value)
    }
}
