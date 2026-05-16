use std::path::PathBuf;

use candle_core::Tensor;
use candle_transformers::{generation::LogitsProcessor, models::quantized_qwen2::ModelWeights};
use tokenizers::Tokenizer;

use crate::core::model::{helper::get_device, models::base::ModelBase};

pub struct QwenModel;

impl ModelBase for QwenModel {
    fn get_gguf_name() -> String {
        return "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf".into();
    }

    fn get_original_name() -> String {
        return "Qwen/Qwen2.5-0.5B-Instruct".into();
    }

    fn run(&self, model_path: String, prompt: String) -> color_eyre::Result<String> {
        let device = get_device()?;

        let mut model_file = std::fs::File::open(&model_path)?;

        let model_content = candle_core::quantized::gguf_file::Content::read(&mut model_file)?;

        let mut model = ModelWeights::from_gguf(model_content, &mut model_file, &device)?;

        let tokenizer_path = PathBuf::from(&model_path).with_file_name("tokenizer.json");

        let system = self.prepare_prompt(prompt);

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .expect("Error to load tokenizer");
        let tokens = tokenizer.encode(system, true)
            .expect("Error to generate tokens");
        let tokens = tokens.get_ids();

        let mut logits_processor = LogitsProcessor::new(42, Some(0.0), Some(0.9));

        let eos_tokens = vec![
            tokenizer.token_to_id("<|im_end|>"),
            tokenizer.token_to_id("<|endoftext|>"),
            tokenizer.token_to_id("</s>")
        ].into_iter().flatten().collect::<Vec<_>>();

        let input = Tensor::new(tokens, &device)?.unsqueeze(0)?;
        
        let logits = model.forward(&input, 0)?;
        let logits = logits.squeeze(0)?;

        let mut next_token = logits_processor.sample(&logits)?;

        let mut all_tokens = Vec::<u32>::new();
        for index in 0..32 {
            if eos_tokens.contains(&next_token) {
                break;
            }

            all_tokens.push(next_token);

            let input = Tensor::new(&[next_token], &device)?.unsqueeze(0)?;
            let logits = model.forward(&input, tokens.len() + index)?.squeeze(0)?;
            let logits = candle_transformers::utils::apply_repeat_penalty(&logits, 1.1, &all_tokens)?;

            next_token = logits_processor.sample(&logits)?;
        }

        let output = tokenizer.decode(&all_tokens, false)
            .expect("Error to decode");

        Ok(output)
    }

    fn prepare_prompt(&self, prompt: String) -> String {
        format!(include_str!("../../../../prompts/qwen.txt"), prompt)
    }
}