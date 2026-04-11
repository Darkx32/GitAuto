use std::path::PathBuf;

use candle_core::Tensor;
use candle_transformers::{generation::LogitsProcessor, models::quantized_qwen2::ModelWeights};
use tokenizers::Tokenizer;

use crate::core::model::helper::get_device;

pub fn get_qwen_gguf() -> String{
    "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf".into()
}

pub fn get_real_model() -> String{
    "Qwen/Qwen2.5-0.5B-Instruct".into()
}

pub fn run_qwen(model_path: String, prompt: String) -> color_eyre::Result<String> {
    let device = get_device()?;

    let mut model_file = std::fs::File::open(&model_path)?;

    let model_content = candle_core::quantized::gguf_file::Content::read(&mut model_file)?;

    let mut model = ModelWeights::from_gguf(model_content, &mut model_file, &device)?;

    let tokenizer_path = PathBuf::from(&model_path).with_file_name("tokenizer.json");

    let system = get_prompt(prompt);

    let tokenizer = Tokenizer::from_file(tokenizer_path)
        .expect("Error to load tokenizer");
    let tokens = tokenizer.encode(system, true)
        .expect("Error to generate tokens");
    let tokens = tokens.get_ids();

    let mut logits_processor = LogitsProcessor::new(42, Some(0.15), None);

    let mut next_token = {
        let input = Tensor::new(tokens, &device)?.unsqueeze(0)?;
        let logits = model.forward(&input, 0)?;
        let logits = logits.squeeze(0)?;
        logits_processor.sample(&logits)?
    };

    let fallback_token = tokenizer
        .token_to_id("<|im_end|>")
        .unwrap_or(151645);

    let mut all_tokens = Vec::<u32>::new();
    for index in  0..100 {
        let input = Tensor::new(&[next_token], &device)?.unsqueeze(0)?;
        let logits = model.forward(&input, tokens.len() + index)?;
        let logits = logits.squeeze(0)?;

        next_token = logits_processor.sample(&logits)?;
        if next_token == fallback_token {
            break;
        }
        all_tokens.push(next_token);
    }

    let output = tokenizer.decode(&all_tokens, false)
        .expect("Error to decode");

    Ok(output)
}

fn get_prompt(prompt: String) -> String {
    format!("<|im_start|>system
You are a commit message generator that MUST strictly follow the Conventional Commits specification.

OUTPUT FORMAT (STRICT):
type(optional-scope): short description

RULES (MANDATORY):
- Output ONLY one single line.
- NO explanations, NO extra text, NO markdown.
- MAX 72 characters total.
- Description MUST be in English.
- Use imperative mood (e.g., 'add', 'fix', 'remove').
- DO NOT end with a period.
- DO NOT include file names unless necessary.

ALLOWED TYPES:
feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert

SCOPE:
- Optional, short, lowercase (e.g., parser, api, ui)

SELECTION RULES:
- feat → new feature
- fix → bug fix
- refactor → code change without behavior change
- perf → performance improvement
- style → formatting only
- docs → documentation only
- test → tests added/changed
- build/ci → tooling or pipeline
- chore → maintenance

IF DIFF IS EMPTY:
- Output: chore: no changes<|im_end|>
<|im_start|>user
{}<|im_end|>
<|im_start|>assistant", prompt.trim())
}