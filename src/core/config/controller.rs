use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &str = "git_auto_config";

#[derive(Debug, Serialize, Deserialize)]
pub struct GitAutoConfig{
    pub model_name: String,
    pub model_folder: String
}

impl Default for GitAutoConfig {
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("io", "github", "GitAuto")
            .unwrap();

        let model_name = String::from("bartowski/Qwen2.5-0.5B-Instruct-GGUF");

        let model_folder = String::from(project_dirs.data_dir().to_str().unwrap());

        Self {
            model_name,
            model_folder
        }
    }
}

pub fn reset_to_default() -> color_eyre::Result<()> {
    let config_reseted = GitAutoConfig::default();

    confy::store(CONFIG_FILENAME, None, config_reseted)?;

    Ok(())
}

pub fn set_model(new_model: &String) -> color_eyre::Result<()> {
    let mut config = get_configuration()?;
    config.model_name = new_model.into();

    confy::store(CONFIG_FILENAME, None, config)?;

    Ok(())
}

pub fn get_configuration() -> color_eyre::Result<GitAutoConfig> {
    let config: GitAutoConfig = confy::load(CONFIG_FILENAME, None)?;

    Ok(config)
}