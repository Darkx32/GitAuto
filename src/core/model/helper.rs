use std::{fs, path::PathBuf};

use candle_core::Device;
use owo_colors::OwoColorize;

use crate::core::model::models::{base::{ModelBase, Models}, qwen::QwenModel};

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

pub(super) fn get_model_data(model: Models) -> (String, String){
    match model {
        Models::Qwen => {
            (QwenModel::get_gguf_name(), QwenModel::get_original_name())
        }
    }
} 