use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &str = "git_auto_config";

#[derive(Debug, Serialize, Deserialize)]
pub struct GitAutoConfig{
    pub model_name: String,
    pub model_tensor: String,
    pub model_folder: String
}

impl GitAutoConfig {
    pub fn get_all_variables() -> &'static [&'static str] {
        &["model_name", "model_tensor", "model_folder"]
    }
}

impl Default for GitAutoConfig {
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("io", "github", "GitAuto")
            .unwrap();

        let model_name = String::from("microsoft/Phi-4-mini-instruct");
        let model: Vec<&str> = model_name.split("/").collect();

        let model_folder = format!("{}/{}", String::from(project_dirs.data_dir().to_str().unwrap()), model[1]);

        Self {
            model_name,
            model_tensor: String::from("model-00002-of-00002.safetensors"),
            model_folder
        }
    }
}

pub fn get_configuration() -> color_eyre::Result<GitAutoConfig> {
    let config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    Ok(config)
}

pub fn change_model_name(new_model: &str) -> color_eyre::Result<()> {
    let mut config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    config.model_name = new_model.into();
    confy::store(CONFIG_FILENAME, None, config)?;

    Ok(())
}

pub fn change_model_tensor(new_tensor: &str) -> color_eyre::Result<()> {
    let mut config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    config.model_tensor = new_tensor.into();
    confy::store(CONFIG_FILENAME, None, config)?;

    Ok(())
}

pub fn change_model_folder(new_folder: &str) -> color_eyre::Result<()> {
    let mut config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    config.model_folder = new_folder.into();
    confy::store(CONFIG_FILENAME, None, config)?;

    Ok(())
}