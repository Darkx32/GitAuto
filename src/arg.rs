use clap::{Command, arg};

fn cli() -> Command {
    Command::new("GitAuto")
        .about("IA implementation on git commit")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("commit")
                .about("Commits a code")
                .arg(arg!(<COMMIT> "The commit msg"))
                .arg_required_else_help(true)
        )
}

pub fn cli_handle() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("commit", sub_matches)) => {
            println!("Commiting {}", sub_matches.get_one::<String>("COMMIT").expect("required"))
        },
        _ => unreachable!()
    }
}