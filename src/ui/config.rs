use inquire::{Confirm, Select};
use owo_colors::OwoColorize;

use crate::core::{config::{get_configuration, reset_to_default, set_model}, model::hub::{delete_model, download_model, model_is_installed}};

pub fn render() -> color_eyre::Result<()> {
    let option = Select::new("Select option to change:", 
    ["Model", "Folder"].into())
        .prompt()?;

    match option {
        "Model" => {
            let option = Select::new("What model do you want use?", 
            ["TinyLlama/TinyLlama-1.1B-Chat-v1.0", "microsoft/phi-2"].to_vec())
                .prompt()?;

            let (older_is_installed, older_path) = model_is_installed()?;
            if older_is_installed {
                let confirmation = Confirm::new("Delete old model installed on system?")
                    .with_default(true).prompt()?;

                if confirmation {
                    let config = get_configuration()?;

                    delete_model(older_path, config.model_name)?;
                    println!("{}", "Old model has deleted successfully".green());
                }
            }

            set_model(&String::from(option))?;
            let confirmation = Confirm::new("Download new model?")
                .with_default(true).prompt()?;

            if confirmation {
                download_model()?;
            }
        },
        _ => {
            println!("{}", "Option not founded.".red());
        }
    }

    Ok(())
}

pub fn see() -> color_eyre::Result<()> {
    let configuration = get_configuration()?;

    println!("{}", "All configurations:".bold().yellow());
    println!("{}: {}", "Model name".blue(), configuration.model_name.green());
    println!("{}: {}", "model folder".blue(), configuration.model_folder.green());
    println!("{}", "--- Final ---".bold().yellow());

    Ok(())
}

pub fn reset() -> color_eyre::Result<()> {
    reset_to_default()?;
    println!("{}", "Configurations is reseted to default factory".bold().yellow());

    Ok(())
}