use crate::arg::cli_handle;

mod arg;
mod ui;
mod core;

use owo_colors::OwoColorize;

fn main() {
    #[cfg(debug_assertions)]
    color_eyre::install().unwrap();

    if let Err(err) = run() {
        #[cfg(debug_assertions)]
        eprintln!("{:?}", err.red());

        #[cfg(not(debug_assertions))]
        eprintln!("{}", err.red());

        std::process::exit(1);
    }
    
}

fn run() -> color_eyre::Result<()> {
    cli_handle()
}