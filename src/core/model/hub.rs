use std::path::PathBuf;
use owo_colors::OwoColorize;
use hf_hub::{Cache, Repo, api::sync::ApiBuilder};

use crate::core::{config::{self, get_configuration}, git::git_controller::get_all_lines_changed, model::{helper::create_folder_it_not_exists, models}};

pub fn download_model() -> color_eyre::Result<()> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    create_folder_it_not_exists(&path_buf)?;

    let api = ApiBuilder::new()
        .with_cache_dir(path_buf)
        .build()?;
    let model = api.model(config.model_name.clone());

    println!("{}{}", "Instalando o modelo: ", config.model_name.bold());

    model.get("config.json")?;
    model.get("tokenizer.json")?;
    let filepath = model.get("model.safetensors")
        .expect("Error to find tensor model");

    println!("Modelo baixado em: {}", filepath.display().green());

    Ok(())
}

pub fn run(filter: Option<Vec<String>>) -> color_eyre::Result<String> {
    let (is_installed, model_path) = model_is_installed()?;
    if !is_installed {
        println!("{}", "Model not found.".red());
        return Ok("".into())
    }

    let config = get_configuration()?;
    let diff = get_all_lines_changed(filter)?;

    let prompt = format!(
    "<|system|>\n\
    You are a strict commit message generator. Output ONLY a single-line Conventional Commit message for the code changes provided. No explanations, no markdown, no quotes.</s>\n\
    <|user|>\n\
    Changes:\n\
    {}</s>\n\
    <|assistant|>\n", diff.join("\n"));

    println!("{}", prompt);

    let output = match config.model_name.as_str() {
        "TinyLlama/TinyLlama-1.1B-Chat-v1.0" => {
            models::tiny::run_tiny(model_path, prompt)?
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

pub fn model_is_installed() -> color_eyre::Result<(bool, String)> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    let cache = Cache::new(path_buf);
    let repo = Repo::model(config.model_name);

    if let Some(path) = cache.repo(repo).get("model.safetensors") {
        return Ok((true, path.display().to_string()))
    } else {
        Ok((false, String::new()))
    }
}