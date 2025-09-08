use std::collections::HashMap;

use crate::foundation::{
    config::Config,
    registry::{Hkey, HkeyData, HkeyPath},
};

fn val(config: &Config, key: &str, namespace: &str, name: &str) -> Option<(String, HkeyData)> {
    match config.get(namespace, name) {
        Some(it) => Some((key.to_owned(), it.replace(",", " ").into())),
        None => None,
    }
}

fn ins(map: &mut HashMap<String, HkeyData>, item: Option<(String, HkeyData)>) {
    if let Some((key, value)) = item {
        map.insert(key, value);
    }
}

pub fn create_colors_registry(config: &Config) -> Hkey {
    let mut map = HashMap::<String, HkeyData>::new();
    
    // TODO: use better mapping?
    ins(&mut map, val(config, "ActiveBorder", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "ActiveTitle", "Colors:Header", "BackgroundNormal"));
    ins(&mut map, val(config, "AppWorkSpace", "Colors:View", "BackgroundNormal"));
    ins(&mut map, val(config, "Background", "Colors:View", "BackgroundNormal"));
    ins(&mut map, val(config, "ButtonAlternativeFace", "Colors:Button", "BackgroundNormal"));
    ins(&mut map, val(config, "ButtonDkShadow", "Colors:Button", "ForegroundInactive"));
    ins(&mut map, val(config, "ButtonFace", "Colors:Button", "BackgroundNormal"));
    ins(&mut map, val(config, "ButtonHilight", "Colors:Button", "ForegroundInactive"));
    ins(&mut map, val(config, "ButtonLight", "Colors:Button", "BackgroundNormal"));
    ins(&mut map, val(config, "ButtonShadow", "Colors:Button", "BackgroundNormal"));
    ins(&mut map, val(config, "ButtonText", "Colors:Button", "ForegroundNormal"));
    ins(&mut map, val(config, "GradientActiveTitle", "Colors:Header", "BackgroundNormal"));
    ins(&mut map, val(config, "GradientInactiveTitle", "Colors:Header", "BackgroundNormal"));
    ins(&mut map, val(config, "GrayText", "Colors:Button", "ForegroundInactive"));
    ins(&mut map, val(config, "Hilight", "Colors:Selection", "BackgroundNormal"));
    ins(&mut map, val(config, "HilightText", "Colors:Selection", "ForegroundNormal"));
    ins(&mut map, val(config, "InactiveBorder", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "InactiveTitle", "Colors:Header", "BackgroundNormal"));
    ins(&mut map, val(config, "InactiveTitleText", "Colors:Window", "ForegroundInactive"));
    ins(&mut map, val(config, "InfoText", "Colors:Tooltip", "ForegroundNormal"));
    ins(&mut map, val(config, "InfoWindow", "Colors:Tooltip", "BackgroundNormal"));
    ins(&mut map, val(config, "Menu", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "MenuBar", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "MenuHilight", "Colors:Selection", "BackgroundNormal"));
    ins(&mut map, val(config, "MenuText", "Colors:Window", "ForegroundNormal"));
    ins(&mut map, val(config, "Scrollbar", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "TitleText", "Colors:Header", "ForegroundNormal"));
    ins(&mut map, val(config, "Window", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "WindowFrame", "Colors:Window", "BackgroundNormal"));
    ins(&mut map, val(config, "WindowText", "Colors:Window", "ForegroundNormal"));

    Hkey {
        path: HkeyPath::new("HKEY_CURRENT_USER/Control Panel/Colors"),
        content: map,
    }
}
