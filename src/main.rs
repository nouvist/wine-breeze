use clap::Parser;

#[cfg(unix)]
mod main_linux;

#[cfg(windows)]
mod main_win32;

/// Breezify Wine with ease
#[derive(Parser, Debug)]
#[command(version, about)]
struct App {}

fn main() {
    #[cfg(unix)]
    main_linux::main();
    #[cfg(windows)]
    main_win32::main();
}
