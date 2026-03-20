use crate::arg::cli_handle;

mod arg;
mod ui;
mod core;

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;

    cli_handle()
}