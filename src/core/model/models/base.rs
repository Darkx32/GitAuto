pub trait ModelBase {
    fn get_gguf_name() -> String
        where 
            Self: Sized;

    fn get_original_name() -> String
        where 
            Self: Sized;

    fn run(&self, model_path: String, prompt: String) -> color_eyre::Result<String>;

    fn prepare_prompt(&self, prompt: String) -> String {
        format!("<|im_start|>system
You are a commit message generator. Analyze the git diff and output ONE commit message.

OUTPUT: type(scope): description

TYPES: feat fix docs style refactor perf test build ci chore revert

RULES:
- One line only
- Max 72 characters
- English only
- No period at end
- No explanations
- No markdown

EXAMPLES:
feat(auth): add OAuth2 login support
fix(api): handle null response from endpoint
refactor(parser): simplify token extraction logic
chore: update dependencies
docs(readme): add installation steps

IF EMPTY DIFF: chore: no changes<|im_end|>
<|im_start|>user
diff:
{}<|im_end|>
<|im_start|>assistant", prompt.trim())
    }
}