use super::adapter::ApiProvider;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
pub const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

pub const DEFAULT_PROVIDER: ApiProvider = ApiProvider::OPENAI;
pub const DEFAULT_MODEL: &'static str = "gpt-4o-mini";

pub const SUPPORTED_PROVIDERS: [ApiProvider; 3] = [
    ApiProvider::ANTHROPIC,
    ApiProvider::OPENAI,
    ApiProvider::OPENROUTER,
    //    ApiProvider::OLLAMA,
];

pub static SUPPORTED_MODELS: LazyLock<HashMap<&str, ApiProvider>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("gpt-4o-mini", ApiProvider::OPENAI);
    m.insert("gpt-3.5-turbo", ApiProvider::OPENAI);
    m.insert("gpt-4", ApiProvider::OPENAI);
    m.insert("claude-v1", ApiProvider::ANTHROPIC);
    m.insert("ollama", ApiProvider::OLLAMA);

    m
});
