use std::path::PathBuf;
use candle_nn::VarBuilder;
use candle_transformers::models::phi3::Config;
use color_eyre::eyre::eyre;
use owo_colors::OwoColorize;
use hf_hub::{Cache, Repo, api::sync::{Api, ApiBuilder}};
use tokenizers::Tokenizer;

use crate::core::{config, model::helper::{create_folder_it_not_exists, get_device}};

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
    let filepath = model.get(&config.model_tensor)
        .expect("Error to find tensor model");

    println!("Modelo baixado em: {}", filepath.display().green());

    Ok(())
}

pub fn run_model() -> color_eyre::Result<()> {
    let tensor_path = model_is_installed()?;
    if !tensor_path.0 {
        return Err(eyre!("Model not installed"))
    }
    let device = get_device()?;

    let tensor_path_buf = PathBuf::from(tensor_path.1);
    let tokenizer_path_buf = tensor_path_buf.with_file_name("tokenizer.json");
    let config_path_buf = tensor_path_buf.with_file_name("config.json");

    let config_str = std::fs::read_to_string(config_path_buf)?;
    let config: Config = serde_json::from_str(&config_str)?;
    let tokenizer = Tokenizer::from_file(tokenizer_path_buf)
        .expect("Error to load tokenizer");

    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[tensor_path_buf], candle_core::DType::F32, &device)?
    };

    Ok(())
}

pub fn model_exists(model: &String) -> color_eyre::Result<bool> {
    let api = Api::new()?.model(model.into());

    match api.info() {
        Ok(_) => return Ok(true),
        Err(_) => return Ok(false)
    }
}

pub fn model_is_installed() -> color_eyre::Result<(bool, String)> {
    let config = config::get_configuration()?;
    let path_buf = PathBuf::from(config.model_folder);

    let cache = Cache::new(path_buf);
    let repo = Repo::model(config.model_name);

    if let Some(path) = cache.repo(repo).get(&config.model_tensor) {
        return Ok((true, path.display().to_string()))
    } else {
        Ok((false, String::new()))
    }
}