use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &str = "git_auto_config";

#[derive(Debug, Serialize, Deserialize)]
struct GitAutoConfig{
    model_name: String,
    model_folder: String
}

impl Default for GitAutoConfig {
    fn default() -> Self {
        Self {
            model_name: String::from("microsoft/Phi-4-mini-instruct"),
            model_folder: String::from("")
        }
    }
}

pub fn change_model_name(new_model: &str) -> color_eyre::Result<()> {
    let mut config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    config.model_name = new_model.into();
    confy::store(CONFIG_FILENAME, None, config)?;

    Ok(())
}