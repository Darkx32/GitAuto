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

pub fn change_configuration(new_config: &GitAutoConfig) -> color_eyre::Result<()> {
    confy::store(CONFIG_FILENAME, None, new_config)?;

    Ok(())
}