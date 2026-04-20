use crate::core::model::hub::{self, clear_model_folder, model_is_installed};
use inquire::Confirm;
use owo_colors::OwoColorize;

pub fn render() -> color_eyre::Result<()> {
    let (is_installed, path) = model_is_installed()?;

    if is_installed {
        println!("{} on {}", "Installed".green(), path.blue());
    } else {
        println!("{}", "Not installed".red());

        let to_install = Confirm::new("Do you wanna install?")
            .with_default(true)
            .prompt()?;

        if to_install {
            hub::download_model()?;
        }
    }

    Ok(())
}

pub fn clear() -> color_eyre::Result<()> {
    match clear_model_folder() {
        Ok(()) => {
            println!("{}", "All models installed has been deleted.".green());
        },
        Err(err) => {
            println!("{} {}", "Error on clear models: ".red(), err.red());
        }
    }

    Ok(())
}