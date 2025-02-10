use crate::api::constants::*;
use anyhow::{Error, Result};
use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    env::{self, VarError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiProvider {
    OPENAI,
    ANTHROPIC,
    OLLAMA,
    OPENROUTER,
}
// Adapter will act as an instance which tracks global state
#[derive(Debug)]
pub struct Adapter {
    api_keys: HashMap<ApiProvider, String>,
    current_provider: ApiProvider,
    current_model: String,
}

impl Adapter {
    pub fn new() -> Self {
        // Get All API Keys from Environment
        let mut api_keys = HashMap::new();
        for provider in SUPPORTED_PROVIDERS.iter() {
            match get_api_key_from_env(provider) {
                Ok(api_key) => {
                    api_keys.insert(*provider, api_key);
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }

        // Get Default AI Model from Environment
        // If not found, use the default model
        let current_model = match get_ai_model_from_env() {
            Ok(current_model) => current_model,
            Err(e) => {
                eprintln!("{:?}", e);
                println!(
                    "🟡[ADAPTER] 🧩 Warning: Using Default Model - `{}`",
                    DEFAULT_MODEL
                );
                String::from(DEFAULT_MODEL)
            }
        };

        // Get the API Provider from the Specified Model
        // If not found, use the default provider
        let current_provider = match Self::get_api_provider_from_model(&current_model) {
            Ok(provider) => *provider,
            Err(e) => {
                eprintln!("{:?}", e);
                println!(
                    "🟡[ADAPTER] 🧩 Warning: Using Default Provider {:#?}",
                    DEFAULT_PROVIDER
                );
                DEFAULT_PROVIDER
            }
        };

        Self {
            api_keys,
            current_provider,
            current_model,
        }
    }

    pub fn get_current_provider(&self) -> ApiProvider {
        self.current_provider
    }

    pub fn get_current_model(&self) -> &str {
        &self.current_model
    }

    pub fn get_api_key(&self, provider: &ApiProvider) -> Option<&String> {
        self.api_keys.get(provider)
    }

    pub fn set_current_provider(&mut self, provider: ApiProvider) {
        // add validation for provider
        self.current_provider = provider;
    }

    pub fn set_current_model(&mut self, model: String) {
        // add validation for model
        self.current_model = model;
    }

    pub fn set_api_key(&mut self, provider: ApiProvider, key: String) {
        // add validation for api key (HOW?)
        self.api_keys.insert(provider, key);
    }

    fn get_api_provider_from_model(model_name: &str) -> Result<&ApiProvider, Error> {
        SUPPORTED_MODELS.get(model_name).ok_or_else(|| {
            Error::msg(format!(
                "🟡[ADAPTER] 💔 Warning: Invalid/Unknown Model | `{}` API Provider Unsupported!",
                model_name
            ))
        })
    }

    pub async fn send_test_request(&self, msg: &str) -> Result<()> {
        println!(
            "🟢[ADAPTER] 🚀📡 Sending Test Request to API Provider {:#?}...",
            self.current_provider
        );
        let request = formulate_request(self.current_provider, &self.current_model, msg).await;
        let response = self.send_request(&request, &self.current_provider).await?;
        println!("{}", parse_response(self.current_provider, response).await?);
        Ok(())
    }

    pub async fn send_request(
        &self,
        request: &Value,
        provider: &ApiProvider,
    ) -> Result<Response, Error> {
        let client = Client::new();
        let submission = match provider {
            ApiProvider::OPENAI => client
                .post(OPENAI_API_URL)
                .header("Content-Type", "application/json")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.get_api_key(provider).unwrap()),
                ),
            ApiProvider::ANTHROPIC => client
                .post(ANTHROPIC_API_URL)
                .header("x-api-key", self.get_api_key(provider).unwrap())
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json"),
            ApiProvider::OLLAMA => {
                todo!("API Provider Ollama - Send Request - To Be Implemented")
            }
            ApiProvider::OPENROUTER => client
                .post(OPENROUTER_API_URL)
                .header("Content-Type", "application/json")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.get_api_key(provider).unwrap()),
                ),
        };
        submission.json(&request).send().await.map_err(|e| {
            Error::msg(format!(
                "🔴[ADAPTER] 🚫🔌 Error: Failed to send request to API Provider {:#?} | {:?}",
                provider, e
            ))
        })
    }
}

fn get_api_key_from_env(selected_provider: &ApiProvider) -> Result<String, Error> {
    let api_name = match selected_provider {
        ApiProvider::OPENAI => env::var("OPENAI_API_KEY"),
        ApiProvider::ANTHROPIC => env::var("ANTHROPIC_API_KEY"),
        ApiProvider::OLLAMA => {
            todo!("Implement Ollama Support")
        }
        ApiProvider::OPENROUTER => env::var("OPENROUTER_API_KEY"),
    };

    match api_name {
        Ok(val) if val.is_empty() => {
            return Err(Error::msg(format!(
                "🟡[ENV] 🚫🔑 Warning:  {:?} | API Key is empty.",
                selected_provider
            )));
        }
        Ok(val) => {
            println!(
                "🟢[ENV] ✅🔑 Success: {:?} | API Key Loaded.",
                selected_provider
            );
            Ok(val)
        }
        Err(VarError::NotPresent) => {
            return Err(Error::msg(format!(
                "🟡[ENV] 🔍🔑 Warning: {:?} | API Key undetected.",
                selected_provider,
            )));
        }
        Err(VarError::NotUnicode(_)) => {
            return Err(Error::msg(format!(
                "🟡[ENV] 🔍🔑 Warning: {:?} | API Key unreadable.",
                selected_provider
            )));
        }
    }
}

fn get_ai_model_from_env() -> Result<String, Error> {
    let model_name = env::var("NYOTA_DEFAULT_AI_MODEL");
    match model_name {
        Ok(val) if val.is_empty() => {
            return Err(Error::msg(
                "🟡[ENV] 🚫🔩 Warning: Default AI Model was not set.",
            ));
        }
        Ok(val) if !SUPPORTED_MODELS.contains_key(val.as_str()) => {
            return Err(Error::msg(format!(
                "🟡[ENV] 💔🔩 Warning: Specified Default AI Model {:#?} is not supported.",
                val
            )));
        }
        Ok(val) => {
            println!(
                "🟢[ENV] ⚙️🔩 Success: Default AI Model Set from environment: {}",
                val
            );
            return Ok(val);
        }
        Err(VarError::NotPresent) => {
            return Err(Error::msg(
                "🟡[ENV] 🔍🔩 Warning: Default AI Model undetected.",
            ));
        }
        Err(VarError::NotUnicode(_)) => {
            return Err(Error::msg(
                "🟡[ENV] 🔍🔩 Warning: Default AI Model unreadable",
            ));
        }
    };
}

async fn formulate_request(provider: ApiProvider, model: &str, msg: &str) -> Value {
    let req: Value;
    match provider {
        ApiProvider::OPENAI => {
            req = json!({
                "model": model,
               "store": true,
                "stream": false,
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."},
                    {"role": "user", "content": msg}
                ]
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

pub async fn send_request(request: &Value, provider: &ApiProvider) -> Response {
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
        ApiProvider::ANTHROPIC => {
            resp = client
                .post(ANTHROPIC_API_URL)
                .header(
                    "x-api-key",
                    get_api_key_from_env(&ApiProvider::ANTHROPIC).unwrap(),
                )
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request)
                .send()
                .await
                .unwrap();
        }
        ApiProvider::OLLAMA => {
            todo!("API Provider Ollama - Send Request - To Be Implemented")
        }
        ApiProvider::OPENROUTER => {
            resp = client
                .post(OPENROUTER_API_URL)
                .header("Content-Type", "application/json")
                .header(
                    "Authorization",
                    format!(
                        "Bearer {}",
                        get_api_key_from_env(&ApiProvider::OPENROUTER).unwrap()
                    ),
                )
                .json(&request)
                .send()
                .await
                .unwrap();
        }
    }
    return resp;
}

pub async fn parse_response(model: ApiProvider, api_response: Response) -> Result<String, Error> {
    if api_response.status().is_success() {
        let json: Value = api_response.json().await?;

        match model {
            ApiProvider::OPENAI => parse_openai_response(json).await,
            ApiProvider::ANTHROPIC => parse_anthropic_response(json).await,
            ApiProvider::OLLAMA => parse_ollama_response(json).await,
            ApiProvider::OPENROUTER => parse_openrouter_response(json).await,
        }
    } else {
        let error_text = api_response.text().await?;
        Err(Error::msg(format!("API error: {}", error_text)))
    }
}

async fn parse_openai_response(json_response: Value) -> Result<String, Error> {
    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| Error::msg("No content found in response!"))?;

    Ok(String::from(content))
}

async fn parse_anthropic_response(json_response: Value) -> Result<String, Error> {
    let content = json_response["messages"][0]["text"]
        .as_str()
        .ok_or_else(|| Error::msg("No content found in response!"))?;
    Ok(String::from(content))
}

async fn parse_ollama_response(json_response: Value) -> Result<String, Error> {
    println!("Raw Open Router Response {:?}", json_response);
    todo!("Implement Ollama Response Parsing")
}

async fn parse_openrouter_response(json_response: Value) -> Result<String, Error> {
    println!("Raw Open Router Response {:?}", json_response);
    todo!("Implement OpenRouter Parsing!")
}
