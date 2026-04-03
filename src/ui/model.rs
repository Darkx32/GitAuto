use crate::core::model::hub::{self, model_is_installed};
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