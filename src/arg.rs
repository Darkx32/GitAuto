use clap::{Parser, Subcommand};

use crate::{core::model::hub, ui::{commit, config, model}};

#[derive(Parser)]
#[command(name = "GitAuto")]
#[command(version, about = "IA implementation on git commit")]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Commit { 
        #[arg(short, long)]
        amend: bool
    },
    Config {
        #[arg(short, long)]
        see: bool,

        #[arg(short, long)]
        reset: bool
    },
    Model {
        #[arg(short, long)]
        install: bool,

        #[arg(short, long)]
        clear: bool,
    }
}

pub fn cli_handle() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Commit { amend } => {
            if amend {
                commit::render_amend()
            } else {
                commit::render()
            }
        },
        Commands::Config { see, reset } => {
            if see {
                config::see()
            } else if reset {
                config::reset()
            } else {
                config::render()
            }
        },
        Commands::Model { install, clear } => {
            if install {
                hub::download_model()
            } else if clear {
                model::clear()
            } else {
                model::render()
            }
        }
    }
}