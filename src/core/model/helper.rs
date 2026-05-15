use std::{fs, path::PathBuf};

use candle_core::Device;
use owo_colors::OwoColorize;

use crate::core::model::models::{base::ModelBase, qwen::QwenModel, tiny::TinyLlamaModel};

pub(super) fn create_folder_it_not_exists(folder: &PathBuf) -> color_eyre::Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(folder)?;
    }

    Ok(())
}

pub(super) fn get_device() -> color_eyre::Result<Device> {
    if let Ok(device) = Device::new_cuda(0) {
        println!("{}", "CUDA is using on backend!".bold());
        return Ok(device)
    }

    if let Ok(device) = Device::new_metal(0) {
        println!("{}", "Metal is using on backend!".bold());
        return Ok(device);
    }

    println!("{}", "CPU is using on backend!".bold());
    Ok(Device::Cpu)
}

pub(super) fn get_model_data(model: String) -> (String, String){
    match model.as_str() {
        "bartowski/Qwen2.5-0.5B-Instruct-GGUF" => {
            (QwenModel::get_gguf_name(), QwenModel::get_original_name())
        },
        "s3nh/Tensoic-TinyLlama-1.1B-3T-openhermes-GGUF" => {
            (TinyLlamaModel::get_gguf_name(), TinyLlamaModel::get_original_name())
        }
        _ => unreachable!()
    }
} 