use clap::{Command};

use crate::ui::commit;

fn cli() -> Command {
    Command::new("GitAuto")
        .about("IA implementation on git commit")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("commit")
                .about("Commits a code")
        )
}

pub fn cli_handle() -> color_eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("commit", _)) => {
            commit::render()
        },
        _ => unreachable!()
    }
}