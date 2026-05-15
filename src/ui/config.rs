use std::path::Path;

use inquire::{Confirm, Select, Text};
use owo_colors::OwoColorize;

use crate::core::{config, model::{hub::{delete_model, download_model, model_is_installed}, models::base::Models}};

pub fn render() -> color_eyre::Result<()> {
    let option = Select::new("Select option to change:", 
    ["Model", "Folder"].into())
        .prompt()?;

    match option {
        "Model" => {
            let model_options = vec![
                Models::Qwen,
                Models::TinyLlama
            ];

            let option = Select::new("What model do you want use?", model_options)
                .prompt()?;

            let (older_is_installed, older_path) = model_is_installed()?;
            if older_is_installed {
                let confirmation = Confirm::new("Delete old model installed on system?")
                    .with_default(true).prompt()?;

                if confirmation {
                    let config = config::get_configuration()?;

                    delete_model(older_path, config.model_name)?;
                    println!("{}", "Old model has deleted successfully".green());
                }
            }

            config::set_model(&option.to_string())?;
            let (actual_is_installed, _) = model_is_installed()?;
            if !actual_is_installed {
                let confirmation = Confirm::new("Download new model?")
                    .with_default(true).prompt()?;

                if confirmation {
                    download_model()?;
                }
            }
        },
        "Folder" => {
            let new_folder = Text::new("What's your new folder?")
                .prompt()?;

            std::fs::create_dir_all(&new_folder)?;

            let old_folder = config::get_configuration()?.model_folder;
            let new_folder_path = Path::new(&new_folder);
            for entry in std::fs::read_dir(&old_folder)? {
                let entry = entry?;
                let entry_path = entry.path();

                let dest = new_folder_path.join(entry.file_name());
                std::fs::rename(&entry_path, dest)?;
            }
            std::fs::remove_dir_all(&old_folder)?;

            config::set_folder(&new_folder)?;
            println!("New folder location is: {}", new_folder.bold());
        },
        _ => {
            println!("{}", "Option not founded.".red());
        }
    }

    Ok(())
}

pub fn see() -> color_eyre::Result<()> {
    let configuration = config::get_configuration()?;

    println!("{}", "All configurations:".bold().yellow());
    println!("{}: {}", "Model name".blue(), configuration.model_name.green());
    println!("{}: {}", "model folder".blue(), configuration.model_folder.green());
    println!("{}", "--- Final ---".bold().yellow());

    Ok(())
}

pub fn reset() -> color_eyre::Result<()> {
    config::reset_to_default()?;
    println!("{}", "Configurations is reseted to default factory".bold().yellow());

    Ok(())
}