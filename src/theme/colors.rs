use anyhow::{anyhow, Result};

use crate::foundation::{
    config::Config,
    registry::{Hkey, HkeyData, HkeyPath},
};

fn val(config: &Config, namespace: &str, name: &str) -> Result<HkeyData> {
    match config.get(namespace, name) {
        Some(it) => Ok(it.replace(",", " ").into()),
        None => Err(anyhow!("Config for {namespace} -> {name} is not found")),
    }
}

pub fn create_colors_registry(config: &Config) -> Result<Hkey> {
    let mut hkey = Hkey::new (
        HkeyPath::new("HKEY_CURRENT_USER/Control Panel/Colors")
    );
    
    // TODO: use better mapping?
    hkey.insert("ActiveBorder".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("ActiveTitle".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("AppWorkSpace".to_owned(), val(config, "Colors:View", "BackgroundNormal")?);
    hkey.insert("Background".to_owned(), val(config, "Colors:View", "BackgroundNormal")?);
    hkey.insert("ButtonAlternativeFace".to_owned(), val(config, "Colors:Button", "BackgroundNormal")?);
    hkey.insert("ButtonDkShadow".to_owned(), val(config, "Colors:Button", "ForegroundInactive")?);
    hkey.insert("ButtonFace".to_owned(), val(config, "Colors:Button", "BackgroundNormal")?);
    hkey.insert("ButtonHilight".to_owned(), val(config, "Colors:Button", "ForegroundInactive")?);
    hkey.insert("ButtonLight".to_owned(), val(config, "Colors:Button", "BackgroundNormal")?);
    hkey.insert("ButtonShadow".to_owned(), val(config, "Colors:Button", "BackgroundNormal")?);
    hkey.insert("ButtonText".to_owned(), val(config, "Colors:Button", "ForegroundNormal")?);
    hkey.insert("GradientActiveTitle".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("GradientInactiveTitle".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("GrayText".to_owned(), val(config, "Colors:Button", "ForegroundInactive")?);
    hkey.insert("Hilight".to_owned(), val(config, "Colors:Selection", "BackgroundNormal")?);
    hkey.insert("HilightText".to_owned(), val(config, "Colors:Selection", "ForegroundNormal")?);
    hkey.insert("InactiveBorder".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("InactiveTitle".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("InactiveTitleText".to_owned(), val(config, "Colors:Window", "ForegroundInactive")?);
    hkey.insert("InfoText".to_owned(), val(config, "Colors:Tooltip", "ForegroundNormal")?);
    hkey.insert("InfoWindow".to_owned(), val(config, "Colors:Tooltip", "BackgroundNormal")?);
    hkey.insert("Menu".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("MenuBar".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("MenuHilight".to_owned(), val(config, "Colors:Selection", "BackgroundNormal")?);
    hkey.insert("MenuText".to_owned(), val(config, "Colors:Window", "ForegroundNormal")?);
    hkey.insert("Scrollbar".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("TitleText".to_owned(), val(config, "Colors:Window", "ForegroundNormal")?);
    hkey.insert("Window".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("WindowFrame".to_owned(), val(config, "Colors:Window", "BackgroundNormal")?);
    hkey.insert("WindowText".to_owned(), val(config, "Colors:Window", "ForegroundNormal")?);

    Ok(hkey)
}
