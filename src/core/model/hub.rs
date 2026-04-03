use std::path::PathBuf;
use owo_colors::OwoColorize;
use hf_hub::{Cache, Repo, api::sync::{Api, ApiBuilder}};

use crate::core::{config, model::helper::create_folder_it_not_exists};

pub fn download_model() -> color_eyre::Result<()> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    create_folder_it_not_exists(&path_buf)?;

    let api = ApiBuilder::new()
        .with_cache_dir(path_buf)
        .build()?;
    let model = api.model(config.model_name);

    let filepath = model.get(&config.model_tensor)
        .expect("Error to find tensor model");

    println!("Modelo baixado em: {}", filepath.display().green());

    Ok(())
}

pub fn model_exists(model: &String) -> color_eyre::Result<bool> {
    let api = Api::new()?.model(model.into());

    match api.info() {
        Ok(_) => return Ok(true),
        Err(_) => return Ok(false)
    }
}

pub fn model_is_installed() -> color_eyre::Result<bool> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    let cache = Cache::new(path_buf);
    let repo = Repo::model(config.model_name);

    if let Some(_) = cache.repo(repo).get(&config.model_tensor) {
        return Ok(true)
    } else {
        Ok(false)
    }
}