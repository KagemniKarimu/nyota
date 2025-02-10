use super::adapter::ApiProvider;
use reqwest::{Client, Response};
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
    // Load All Supported OpenAI Models
    m.insert("chatgpt-4o-latest", ApiProvider::OPENAI);
    m.insert("gpt-4o-mini", ApiProvider::OPENAI);
    m.insert("gpt-3.5-turbo", ApiProvider::OPENAI);
    m.insert("gpt-4", ApiProvider::OPENAI);
    m.insert("gpt-4o", ApiProvider::OPENAI);
    m.insert("gpt-4-turbo", ApiProvider::OPENAI);
    m.insert("o1", ApiProvider::OPENAI);
    m.insert("o1-preview", ApiProvider::OPENAI);
    m.insert("o1-mini", ApiProvider::OPENAI);
    m.insert("o3-mini", ApiProvider::OPENAI);

    // Load All Supported Anthropic Models
    m.insert("claude-3-5-sonnet-20241022", ApiProvider::ANTHROPIC);
    m.insert("claude-3-5-haiku-20241022", ApiProvider::ANTHROPIC);
    m.insert("claude-3-5-sonnet-20240620", ApiProvider::ANTHROPIC);
    m.insert("claude-3-haiku-20240307", ApiProvider::ANTHROPIC);
    m.insert("claude-3-opus-20240229", ApiProvider::ANTHROPIC);
    m.insert("claude-3-sonnet-20240229", ApiProvider::ANTHROPIC);
    m.insert("claude-2.1", ApiProvider::ANTHROPIC);
    m.insert("claude-2.1-sonnet", ApiProvider::ANTHROPIC);

    // Load Ollama Support
    // m.insert("ollama", ApiProvider::OLLAMA);

    m
});
