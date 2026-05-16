use core::fmt;
use std::str::FromStr;

pub enum Models {
    Qwen
}

impl fmt::Display for Models {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Models::Qwen => write!(f, "bartowski/Qwen2.5-0.5B-Instruct-GGUF")
        }
    }
}

impl FromStr for Models {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bartowski/Qwen2.5-0.5B-Instruct-GGUF" => Ok(Models::Qwen),
            _ => Err(())
        }
    }
}

pub trait ModelBase {
    fn get_gguf_name() -> String
        where 
            Self: Sized;

    fn get_original_name() -> String
        where 
            Self: Sized;

    fn run(&self, model_path: String, prompt: String) -> color_eyre::Result<String>;

    fn prepare_prompt(&self, prompt: String) -> String;
}