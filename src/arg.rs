use clap::{Arg, ArgAction, Command};

use crate::ui::{commit, config};

fn cli() -> Command {
    Command::new("GitAuto")
        .about("IA implementation on git commit")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("commit")
                .about("Commits a code")
        )
        .subcommand(
            Command::new("config")
                .about("To change configuration")
                .arg(
                    Arg::new("see")
                        .long("see")
                        .short('s')
                        .action(ArgAction::SetTrue)
                        .help("See actual configuration")
                )
        )
}

pub fn cli_handle() -> color_eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("commit", _)) => {
            commit::render()
        },
        Some(("config", sub)) => {
            if sub.get_flag("see") {
                config::render_see()
            } else {
                config::render()
            }
        },
        _ => unreachable!()
    }
}