use core::fmt;

use inquire::{Confirm, MultiSelect, Select, Text};

use crate::core::git::git_controller;

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

    let add_all = Confirm::new("Add all files to commit?")
        .with_default(true)
        .prompt()?;

    if !add_all {
        let all_files_untracked = git_controller::get_all_files_untracked()?;

        let choosed_files = 
            MultiSelect::new("Choose files to auto track", all_files_untracked)
            .prompt()?;

        git_controller::add(choosed_files)?;
    }

    match commit_method {
        CommitMethods::Custom => {
            let commit_type = Select::new("What's type of your commit?", commit_types_options)
                .prompt()?;

            let commit_msg = Text::new("Commit message:")
                .prompt()?;

            
            let msg = format!("{}: {}", commit_type, commit_msg);
            git_controller::commit(msg, add_all.into())?;
        },
        CommitMethods::Generated => {

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