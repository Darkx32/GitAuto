use inquire::{Select, Text};
use owo_colors::OwoColorize;

use crate::core::{config::{GitAutoConfig, change_configuration, get_configuration, reset_to_default}, model::hub::model_exists, string::BetterString};

pub fn render() -> color_eyre::Result<()> {
    let mut config = get_configuration()?;

    let config_options = Vec::from(GitAutoConfig::get_all_variables());
    let config_to_change = Select::new("Select configuration to change:", config_options.clone())
        .prompt()?;

    match config_to_change {
        x if config_options[0] == x => {

            loop {
                let new_model = Text::new("Type your new model:").prompt()?
                .clean();

                let exists = model_exists(&new_model)?;
                if !exists {
                    println!("{}", "This model not exists on hugging face.".red())
                } else {
                    config.model_name = new_model;
                }
            }
        },
        x if config_options[1] == x => {
            let new_tensor = Text::new("Type your new tensor from model:").prompt()?
                .clean();

            config.model_tensor = new_tensor;
        },
        x if config_options[2] == x => {
            let new_folder = Text::new("Type new model folder location:").prompt()?
                .clean();
            
            config.model_folder = new_folder;
        },
        _ => {
            println!("{}", "Option has not founded.".red());
            return Ok(())
        }
    }

    change_configuration(&config)?;
    println!("{}", "Option has been updated.".green());

    Ok(())
}

pub fn see() -> color_eyre::Result<()> {
    let config_options = Vec::from(GitAutoConfig::get_all_variables());
    let configuration = get_configuration()?;

    println!("{}", "All configurations:".bold().yellow());
    println!("{}: {}", config_options[0].blue(), configuration.model_name.green());
    println!("{}: {}", config_options[1].blue(), configuration.model_tensor.green());
    println!("{}: {}", config_options[2].blue(), configuration.model_folder.green());
    println!("{}", "--- Final ---".bold().yellow());


    Ok(())
}

pub fn reset() -> color_eyre::Result<()> {
    reset_to_default()?;
    println!("{}", "Configurations is reseted to default factory".bold().yellow());

    Ok(())
}