use std::path::PathBuf;

use candle_core::Tensor;
use candle_nn::VarBuilder;
use candle_transformers::{generation::LogitsProcessor, models::llama::{self, Llama, LlamaConfig}};
use rand::{Rng, SeedableRng, rngs::StdRng};
use tokenizers::Tokenizer;

use crate::core::model::helper::get_device;

pub fn run_tiny(model_path: String, prompt: String) -> color_eyre::Result<String> {
    let device = get_device()?;
    
    let tensor_path = PathBuf::from(model_path);
    let config_path = tensor_path.with_file_name("config.json");
    let tokenizer_path = tensor_path.with_file_name("tokenizer.json");

    let tokenizer = Tokenizer::from_file(tokenizer_path)
        .expect("Error to load tokenizer");

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(
            &[tensor_path], 
            candle_core::DType::F32, 
            &device
        )?
    };

    let config: LlamaConfig = serde_json::from_slice(&std::fs::read(config_path)?)?;
    let config = config.into_config(false);

    let mut cache = llama::Cache::new(true, 
        candle_core::DType::F32, 
        &config, 
        &device)?;

    let model = Llama::load(vb, &config)?;
    let mut tokens = tokenizer
        .encode(prompt, true)
        .expect("Error to create tokens")
        .get_ids()
        .to_vec();

    let prompt_length = tokens.len();

    let seed = StdRng::seed_from_u64(42).next_u64();
    let mut logits_processor = LogitsProcessor::new(seed, Some(0.7), Some(40.0));

    let mut index_pos = 0;
    for index in 0..200 {
        let (context_size, context_index) = if index > 0 { (1, index_pos) } else { (tokens.len(), 0) };

        let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
        let input = Tensor::new(ctxt, &device)?.unsqueeze(0)?;
        let logits = model.forward(&input, context_index, &mut cache)?;
        let logits = logits.squeeze(0)?.to_dtype(candle_core::DType::F32)?;
        
        index_pos += ctxt.len();

        let next_token = logits_processor.sample(&logits)?;
        tokens.push(next_token);

        if next_token == 2 {
            break;
        }
    }

    let output = tokenizer.decode(&tokens[prompt_length..], true)
        .expect("Error to decode tokens");

    Ok(output)
}