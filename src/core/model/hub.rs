use std::path::PathBuf;
use color_eyre::eyre::eyre;
use owo_colors::OwoColorize;
use hf_hub::{Cache, Repo, api::sync::ApiBuilder};

use crate::core::{config::{self, get_configuration}, git::git_controller::get_all_lines_changed, model::{helper::{create_folder_it_not_exists, get_model_data}, models}};

pub fn download_model() -> color_eyre::Result<()> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    create_folder_it_not_exists(&path_buf)?;

    let api = ApiBuilder::new()
        .with_cache_dir(path_buf)
        .build()?;
    let model = api.model(config.model_name.clone());

    println!("Downloading model: {}", config.model_name.bold());

    let (tensor, model_name) = get_model_data(config.model_name);
    let filepath = model.get(&tensor)
        .expect("Error to find tensor model");

    let model = api.model(model_name.clone());
    let toke_filepath = model.get("tokenizer.json")?;

    std::fs::copy(&toke_filepath, filepath.with_file_name("tokenizer.json"))?;
    
    let folder_to_delete = toke_filepath.parent().unwrap()
        .parent().unwrap().parent().unwrap();
    std::fs::remove_dir_all(folder_to_delete)?;

    println!("Model is installed on: {}", filepath.display().green());

    Ok(())
}

pub fn run(filter: Option<Vec<String>>) -> color_eyre::Result<String> {
    let (is_installed, model_path) = model_is_installed()?;
    if !is_installed {
        return Err(eyre!("{}", "Model not found.".red()))
    }

    let config = get_configuration()?;
    let diff = get_all_lines_changed(filter)?;

    let prompt = diff.join("\n");

    let output = match config.model_name.as_str() {
        "bartowski/Qwen2.5-0.5B-Instruct-GGUF" => {
            models::qwen::run_qwen(model_path, prompt)?
        },
        _ => unreachable!()
    };

    Ok(output)
}

pub fn delete_model(start: String, target: String) -> color_eyre::Result<()> {
    let start_buf = PathBuf::from(start);
    let mut current = Some(start_buf.as_path());

    while let Some(path) = current {
        if let Some(name) = path.file_name() {
            if name == target.as_str() {
                std::fs::remove_dir_all(path)?;
                break;
            }
        }

        current = path.parent();
    }

    Ok(())
}

pub fn clear_model_folder() -> color_eyre::Result<()> {
    let config = config::get_configuration()?;
    
    std::fs::remove_dir_all(&config.model_folder)?;
    std::fs::create_dir(&config.model_folder)?;

    Ok(())
}

pub fn model_is_installed() -> color_eyre::Result<(bool, String)> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    let cache = Cache::new(path_buf);
    let repo = Repo::model(config.model_name.clone());

    let (model_tensor, _) = get_model_data(config.model_name);

    if let Some(path) = cache.repo(repo).get(&model_tensor) {
        return Ok((true, path.display().to_string()))
    } else {
        Ok((false, String::new()))
    }
}