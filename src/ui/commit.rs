use core::fmt;

use inquire::{Confirm, MultiSelect, Select, Text};
use owo_colors::OwoColorize;

use crate::core::{git::{git_check, git_controller, git_see}, model::hub};

enum CommitMethods {
    Custom,
    Generated
}

enum CommitTypes {
    Feature,
    Refactor,
    Docs,
    Style,
    Fix,
    Performance,
    Test
}

pub fn render() -> color_eyre::Result<()> {
    if !git_check::check_if_directory_is_repo() {
        println!("{}", "Actual directory is not a repository.".bold().red());
    }
    if !git_check::check_if_repo_has_changes()? {
        println!("{}", "Actual directory don't have any change files".bold().red());
    }

    let commit_methods_options = vec![
        CommitMethods::Custom, CommitMethods::Generated
    ];

    let commit_types_options = vec![
        CommitTypes::Feature,
        CommitTypes::Refactor,
        CommitTypes::Docs,
        CommitTypes::Style,
        CommitTypes::Fix,
        CommitTypes::Performance,
        CommitTypes::Test
    ];

    let commit_method = Select::new("What's method to commit message?", commit_methods_options)
        .prompt()?;

    let create_checkout = Confirm::new("Do you want create a new branch?")
        .with_default(true)
        .prompt()?;

    let branch_name = if create_checkout {
        Text::new("What's a branch name?")
            .with_help_message(git_see::get_actual_branch()?.as_str())
            .prompt()?
    } else { String::new() };

    let add_all = Confirm::new("Add all files to commit?")
        .with_default(true)
        .prompt().expect("Not enable to find any changes on files in this directory");


    let all_files_untracked = git_controller::get_all_files_untracked()?;
    let choosed_files = if !add_all {
        let choosed_files = 
            MultiSelect::new("Choose files to auto track", all_files_untracked)
            .prompt()?;

        choosed_files
    } else {
        all_files_untracked
    };

    if choosed_files.is_empty() {
        println!("{}", "You have to select at least one to generate auto commit.".red());
        return Ok(());
    }

    let commit_msg: String;
    match commit_method {
        CommitMethods::Custom => {
            let commit_type = Select::new("What's type of your commit?", commit_types_options)
                .prompt()?;

            let msg = Text::new("Commit message:")
                .prompt()?;

            commit_msg = format!("{}: {}", commit_type, msg);
        },
        CommitMethods::Generated => {
            let generated = hub::run(Some(choosed_files.clone()))?;

            commit_msg = Text::new("Commit message(generated):")
                .with_initial_value(&generated)
                .prompt()?;
        }
    }

    if !branch_name.is_empty() {
        git_controller::create_checkout(branch_name)?;
    }
    git_controller::add(choosed_files)?;
    let result = git_controller::commit(commit_msg, add_all.into())?;
    println!("{}", result.green());

    Ok(())
}

pub fn render_amend() -> color_eyre::Result<()> {
    let new_msg = Text::new("Type you new message commit:")
        .with_help_message(format!("Your old commit is '{}'", git_see::get_last_commit()?).as_str())
        .prompt()?;

    match git_controller::amend_last_commit(new_msg) {
        Ok(_) => {
            println!("{}", "New commit msg has been amend.".green())
        }, 
        Err(_) => {
            println!("{}", "Error to amend new msg!".red())
        }
    }

    Ok(())
}

impl fmt::Display for CommitMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitMethods::Custom => write!(f, "Custom"),
            CommitMethods::Generated => write!(f, "Generated")
        }
    }
}

impl fmt::Display for CommitTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitTypes::Feature => write!(f, "feat"),
            CommitTypes::Refactor => write!(f, "refactor"),
            CommitTypes::Docs => write!(f, "docs"),
            CommitTypes::Style => write!(f, "style"),
            CommitTypes::Fix => write!(f, "fix"),
            CommitTypes::Performance => write!(f, "perf"),
            CommitTypes::Test => write!(f, "test")
        }
    }
}