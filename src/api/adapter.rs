use crate::api::constants::*;
use anyhow::{Error, Result};
use std::{
    collections::HashMap,
    env::{self, VarError},
};

use reqwest::{Client, Response};
use serde_json::{json, Value};

#[derive(Debug)]
pub enum ApiProvider {
    OPENAI,
    ANTHROPIC,
    OLLAMA,
    OPENROUTER,
}

impl Default for ApiProvider {
    fn default() -> Self {
        ApiProvider::OPENAI
    }
}

// Adapter will act as an instance which tracks global state
#[derive(Debug)]
pub struct Adapter {
    current_api_provider: ApiProvider,
    active_api_key: String,
    selected_model: String,
}

impl Adapter {
    const DEFAULT_MODEL: &'static str = "gpt-4o-mini";
    const SUPPORTED_MODELS: &'static [&'static str] =
        &["gpt-4o-mini", "gpt-4o", "claude-3", "ollama", "openrouter"];

    pub fn new() -> Self {
        let ai_model = get_ai_model_from_env().unwrap_or_else(|_| Adapter::set_default_ai_model());
        let api_provider = get_api_provider_from_model(&ai_model).unwrap_or_default();
        let api_key = get_api_key_from_env(&api_provider).unwrap_or_else(|_| String::from(""));

        Self {
            selected_model: ai_model,
            current_api_provider: api_provider,
            active_api_key: api_key,
        }
    }

    fn set_default_ai_model() -> String {
        println!("🟢[ADAPTER] No AI Model set from env...");
        println!(
            "🟢[ADAPTER] Using Default Model: {}",
            Adapter::DEFAULT_MODEL
        );
        return String::from(Adapter::DEFAULT_MODEL);
    }
}

fn get_api_key_from_env(selected_provider: &ApiProvider) -> Result<String, VarError> {
    let api_name = match selected_provider {
        ApiProvider::OPENAI => env::var("OPENAI_API_KEY"),
        ApiProvider::ANTHROPIC => env::var("ANTHROPIC_API_KEY"),
        ApiProvider::OLLAMA => {
            todo!("Implement Ollama Support")
        }
        ApiProvider::OPENROUTER => env::var("OPENROUTER_API_KEY"),
    };

    match api_name {
        Ok(ref _val) => println!("🟢[ENV] {:?} | API Key Loaded.", selected_provider),
        Err(VarError::NotPresent) => {
            eprintln!(
                "🟢[ENV] Error: {:?} | API Key undetected.",
                selected_provider
            )
        }
        Err(VarError::NotUnicode(_)) => {
            println!(
                "🟢[ENV] Error: {:?} | API Key unreadable.",
                selected_provider
            )
        }
    }

    return api_name;
}

fn get_api_provider_from_model(model_name: &String) -> Result<ApiProvider, Error> {
    // Rewrite this to use Supported Models and handle dynamically

    match model_name.as_str() {
        "gpt-4" => Ok(ApiProvider::OPENAI),
        "gpt-4o" => Ok(ApiProvider::OPENAI),
        "gpt-4o-mini" => Ok(ApiProvider::OPENAI),
        "sonnet" => Ok(ApiProvider::ANTHROPIC),
        "ollama" => {
            todo!("Implement Ollama Support")
        }
        "openrouter" => Ok(ApiProvider::OPENROUTER),
        _ => Err(Error::msg("[ADAPTER] Invalid default AI model")),
    }
}

fn get_ai_model_from_env() -> Result<String, VarError> {
    let model_name = env::var("NYOTA_DEFAULT_AI_MODEL");
    match model_name {
        Ok(ref val) => println!("🟢[ENV] Default AI Model Set from environment: {}", val),
        Err(VarError::NotPresent) => {
            eprintln!("🟢[ENV] Error: {:?} AI Model undetected.", model_name)
        }
        Err(VarError::NotUnicode(_)) => {
            eprintln!("🟢[ENV] Error: {:?} AI Model unreadable.", model_name)
        }
    };
    return model_name;
}

async fn formulate_request(provider: ApiProvider, model: &str, msg: &str) -> Value {
    let req: Value;
    match provider {
        ApiProvider::OPENAI => {
            req = json!({
                "model": model,
               "store": true,
                "stream": false,
            });
        }
        ApiProvider::ANTHROPIC => {
            req = json!({
            "model": model,
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": msg}]
            });
        }
        ApiProvider::OLLAMA => {
            todo!("API Provider - Formulate Request JSON - To Be Implemented")
        }
        ApiProvider::OPENROUTER => {
            req = json!({
                "model": model,
                "messages": [
                    {"role":"system", "content": "You are a helpful assistant."},
                    {"role":"user", "content": msg}
                ]
            })
        }
    }
    return req;
}

pub async fn send_request(request: &Value, provider: &ApiProvider) -> () {
    let client = Client::new();
    let resp: Response;
    match provider {
        ApiProvider::OPENAI => {
            resp = client
                .post(OPENAI_API_URL)
                .header("Content-Type", "application/json")
                .header(
                    "Authorization",
                    format!(
                        "Bearer {}",
                        get_api_key_from_env(&ApiProvider::OPENAI).unwrap()
                    ),
                )
                .json(&request)
                .send()
                .await
                .unwrap();
        }
        ApiProvider::ANTHROPIC => {}
        ApiProvider::OLLAMA => {}
        ApiProvider::OPENROUTER => {}
    }
}

pub async fn parse_response(model: ApiProvider) {
    match model {
        ApiProvider::OPENAI => {
            parse_openai_response().await;
        }
        ApiProvider::ANTHROPIC => {
            parse_anthropic_response().await;
        }
        ApiProvider::OLLAMA => {
            parse_ollama_response().await;
        }
        ApiProvider::OPENROUTER => {
            parse_openrouter_response().await;
        }
    }
}

async fn parse_openai_response() {
    let url = OPENAI_API_URL;
    let api_key = get_api_key_from_env(&ApiProvider::OPENAI)
        .expect("Make sure your environmental variables are exposed");
    let req = json!({
        "model": "gpt-4o-mini",
       "store": true,
        "stream": false,
    });

    let client = Client::new();

    let resp: Response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req)
        .send()
        .await
        .unwrap();

    if resp.status().is_success() {
        let json_resp: Value = resp.json().await.unwrap();
        if let Some(first_choice) = json_resp["choices"].get(0) {
            if let Some(content) = first_choice["message"]["content"].as_str() {
                println!("Message: {}", content);
            }
        }
    } else {
        let err = resp.text().await.unwrap();
        eprintln!("Error body: {}", err);
    }
}

async fn parse_anthropic_response() {
    let url = ANTHROPIC_API_URL;
    let api_key = get_api_key_from_env(&ApiProvider::ANTHROPIC)
        .expect("Make sure your environmental variables are exposed");
    let req = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 1024,
        "messages": [
        {"role": "user", "content": "Hello, world"}
        ]
    });

    let client = Client::new();

    let resp: Response = client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&req)
        .send()
        .await
        .unwrap();

    if resp.status().is_success() {
        let json_resp: Value = resp.json().await.unwrap();
        if let Some(content) = json_resp["content"].get(0) {
            if let Some(text) = content["text"].as_str() {
                println!("Message: {}", text);
            }
        }
    } else {
        let err = resp.text().await.unwrap();
        eprintln!("Error body: {}", err);
    }
}

async fn parse_ollama_response() {
    todo!("Implement Ollama Response Parsing")
}

async fn parse_openrouter_response() {
    let url = OPENROUTER_API_URL;
    let api_key = get_api_key_from_env(&ApiProvider::OPENROUTER)
        .expect("Make sure your environmental variables are loaded correctly.");
    let req = json!({
        "model": "openai/gpt-4o",
        "messages": [
            {"role":"system", "content": "You are a helpful assistant."},
            {"role":"user", "content": "Hello"}
        ]
    });

    let client = Client::new();

    let resp: Response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req)
        .send()
        .await
        .unwrap();

    println!("Raw Open Router Response {:?}", resp);
    //Partial Parsing Completed
    todo!("Implement OpenRouter Parsing!")
}
