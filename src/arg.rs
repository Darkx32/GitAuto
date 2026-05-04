use clap::{Arg, ArgAction, Command};

use crate::{core::model::hub, ui::{commit, config, model}};

fn cli() -> Command {
    Command::new("GitAuto")
        .about("IA implementation on git commit")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("commit")
                .about("Commits a code")
                .arg(
                    Arg::new("amend")
                        .long("amend")
                        .short('a')
                        .action(ArgAction::SetTrue)
                        .help("Create commit amend")
                )
        )
        .subcommand(
            Command::new("config")
                .about("Change configuration")
                .arg(
                    Arg::new("see")
                        .long("see")
                        .short('s')
                        .action(ArgAction::SetTrue)
                        .help("See actual configuration"))
                .arg(
                    Arg::new("reset")
                        .long("reset")
                        .short('r')
                        .action(ArgAction::SetTrue)
                        .help("Reset to default configuration"))
        )
        .subcommand(
            Command::new("model")
                .about("See models installed")
                .arg(
                    Arg::new("install")
                        .short('i')
                        .long("install")
                        .action(ArgAction::SetTrue)
                        .help("Install defined model on configuration"))
                .arg(
                    Arg::new("clear")
                        .short('c')
                        .long("clear")
                        .action(ArgAction::SetTrue)
                        .help("Delete all models installed"))
        )
}

pub fn cli_handle() -> color_eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("commit", sub)) => {
            if sub.get_flag("amend") {
                commit::render_amend()
            } else {
                commit::render()
            }
        },
        Some(("config", sub)) => {
            if sub.get_flag("see") {
                config::see()
            } else if sub.get_flag("reset") {
                config::reset()
            } else {
                config::render()
            }
        },
        Some(("model", sub)) => {
            if sub.get_flag("install") {
                hub::download_model()
            } else if sub.get_flag("clear") {
                model::clear()
            } else {
                model::render()
            }
        },
        _ => unreachable!()
    }
}