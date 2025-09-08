use clap::Parser;

use crate::{
    foundation::{create_temporary_name, kdeglobals::parse_config, registry::HkeyCollection},
    theme::{create_classic_theme_registry, create_colors_registry, create_dark_mode_registry},
};

pub mod foundation;
pub mod theme;

/// Breezify Wine with ease
#[derive(Parser, Debug)]
#[command(version, about)]
struct App {}

fn main() {
    let _name = create_temporary_name("reg").unwrap();

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

    println!("{}", jar.to_string());
}
