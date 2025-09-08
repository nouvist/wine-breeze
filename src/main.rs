use clap::Parser;

use crate::foundation::{
    registry::{Hkey, HkeyCollection, HkeyPath},
};

pub mod foundation;

/// Breezify Wine with ease
#[derive(Parser, Debug)]
#[command(version, about)]
struct App {}

fn main() {
    let colors = Hkey {
        path: HkeyPath::new("HKEY_CURRENT_USER/Control Panel/Colors"),
        content: [("ActiveBorder".to_owned(), "galon wkwkwk".to_owned())].into(),
    };

    let jar = HkeyCollection(vec![colors]);

    println!("{}", jar.to_string());
}
