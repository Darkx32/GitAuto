use inquire::{Select, Text};
use owo_colors::OwoColorize;

use crate::core::{config::controller::{GitAutoConfig, change_model_folder, change_model_name, change_model_tensor, get_configuration}, string::BetterString};

pub fn render() -> color_eyre::Result<()> {
    let config_options = Vec::from(GitAutoConfig::get_all_variables());
    let config_to_change = Select::new("Select configuration to change:", config_options.clone())
        .prompt()?;

    match config_to_change {
        x if config_options[0] == x => {
            let new_model = Text::new("Type your new model:").prompt()?
                .clean();

            change_model_name(&new_model)?;
        },
        x if config_options[1] == x => {
            let new_tensor = Text::new("Type your new tensor from model:").prompt()?
                .clean();

            change_model_tensor(&new_tensor)?;
        },
        x if config_options[2] == x => {
            let new_folder = Text::new("Type new model folder location:").prompt()?
                .clean();
            
            change_model_folder(&new_folder)?;
        },
        _ => {
            println!("{}", "Option has not founded.".red());
            return Ok(())
        }
    }

    println!("{}", "Option has been updated.".green());

    Ok(())
}

pub fn render_see() -> color_eyre::Result<()> {
    let config_options = Vec::from(GitAutoConfig::get_all_variables());
    let configuration = get_configuration()?;

    println!("{}", "All configurations:".bold().yellow());
    println!("{}: {}", config_options[0].blue(), configuration.model_name.green());
    println!("{}: {}", config_options[1].blue(), configuration.model_tensor.green());
    println!("{}: {}", config_options[2].blue(), configuration.model_folder.green());
    println!("{}", "--- Final ---".bold().yellow());


    Ok(())
}