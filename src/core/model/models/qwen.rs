use std::path::PathBuf;

use candle_core::Tensor;
use candle_transformers::{generation::LogitsProcessor, models::quantized_qwen2::ModelWeights};
use rand::{Rng, SeedableRng, rngs::StdRng};
use tokenizers::Tokenizer;

use crate::core::model::helper::get_device;

pub fn get_qwen_gguf() -> String{
    "qwen2-1_5b-instruct-q4_0.gguf".into()
}

pub fn get_real_model() -> String{
    "Qwen/Qwen2-1.5B-Instruct".into()
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

    let seed = StdRng::seed_from_u64(42).next_u64();
    let mut logits_processor = LogitsProcessor::new(seed, Some(0.2), Some(0.8));

    let mut next_token = {
        let input = Tensor::new(tokens, &device)?.unsqueeze(0)?;
        let logits = model.forward(&input, 0)?;
        let logits = logits.squeeze(0)?;
        logits_processor.sample(&logits)?
    };

    let mut all_tokens = Vec::<u32>::new();
    for index in  0..200 {
        let input = Tensor::new(&[next_token], &device)?.unsqueeze(0)?;
        let logits = model.forward(&input, tokens.len() + index)?;
        let logits = logits.squeeze(0)?;

        next_token = logits_processor.sample(&logits)?;
        all_tokens.push(next_token);
    }

    let output = tokenizer.decode(&all_tokens, false)
        .expect("Error to decode");

    Ok(output)
}

fn get_prompt(prompt: String) -> String {
    format!("
You generate Git commit messages.

Rules:
- Output EXACTLY one line.
- Format: <type>: <short description>
- Language: English only.
- Max 70 characters.
- Do NOT explain anything.
- Do NOT add extra text.

Allowed types:
feat, fix, docs, style, refactor, perf, test, chore

Definitions:
feat = new feature
fix = bug fix
docs = documentation only
style = formatting only
refactor = code change without behavior change
perf = performance improvement
test = adding or updating tests
chore = tooling or maintenance

Examples:
ADD: new login endpoint
DEL: old login logic
-> feat: add login endpoint

ADD: fix null pointer crash
-> fix: prevent null pointer crash

ADD: reformat code
-> style: format code

Now generate the commit:

{}
", prompt)
}