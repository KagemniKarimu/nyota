use crate::api::constants::*;
use anyhow::{Error, Result};
use std::env::{self, VarError};

use reqwest::{Client, Response};
use serde_json::{json, Value};

pub enum Model {
    OPENAI,
    ANTHROPIC,
    //OLLAMA,
    OPENROUTER,
}

pub struct Adapter {
    model: Model,
    api_key: String,
    default_ai_model: String,
}

fn get_api_key(selected_model: Model) -> Result<String, VarError> {
    let api_name = match selected_model {
        Model::OPENAI => env::var("OPENAI_API_KEY"),
        Model::ANTHROPIC => env::var("ANTHROPIC_API_KEY"),
        // Model::OLLAMA => {
        //     Todo!("Implement Ollama Support")
        // }
        Model::OPENROUTER => env::var("OPENROUTER_API_KEY"),
        _ => Err(VarError::NotPresent),
    };

    // match api_name {
    //     Ok(val) => val,
    //     Err(e) => Error::msg("{:?} not found in .env file", api_name),
    // }
    return api_name;
}

fn get_default_ai_model() -> Result<Model, Error> {
    let model_name = env::var("DEFAULT_AI_MODEL");
    match model_name.as_deref() {
        Ok("CHATGPT") => Ok(Model::OPENAI),
        Ok("ANTHROPIC") => Ok(Model::ANTHROPIC),
        // "OLLAMA" => {
        //     Todo!("Implement Ollama Support")
        // }
        Ok("OPENROUTER") => Ok(Model::OPENROUTER),
        _ => Err(Error::msg("Invalid default AI model")),
    }
}

fn construct_test_json(model: Model) {
    json!({
        "model":"gpt-4o-mini",
        "messages": [{ "role": "user", "content": "Say this is a test" }],
        "store": true,
        "stream": false,
    });

    json!({
        "model": "claude-3.5",
        "messages": [
            {
                "role": "user",
                "content": "Say this is a test"
            }
        ],
        "max_tokens": 1024,
    });
}

// async fn send_test_request(model: Model) {
//     let req = json!({
//         "model":"gpt-4o-mini",
//         "messages": [{ "role": "user", "content": "Say this is a test" }],
//         "store": true,
//         "stream": false,
//     });
//     let client = Client::new();

//     let resp: Response = client
//         .post(url)
//         .header("Content-Type", "application/json")
//         .header("Authorization", format!("Bearer {}", api_key))
//         .json(&req)
//         .send()
//         .await
//         .unwrap();

//     let resp_text = resp.text().await.unwrap();

//     if resp.status().is_success() {
//         println!("{:?}", resp);
//         let json_resp = resp.json().await.unwrap();

//         let json_resp: Value = resp.json().await.unwrap();
//         println!("{:?}", json_resp);
//         if let Some(first_choice) = json_resp["choices"].get(0) {
//             //if this arr has content at 0h index
//             if let Some(content) = first_choice["message"]["content"].as_str() {
//                 //if content has key "message"
//                 println!("Message: {}", content); //print
//             }
//         }

//         let zero_val = match json_resp["choices"].get(0) {
//             //if this arr has content at 0h index
//             Some(val) => val,
//             None => &json!({
//                    "e":"e"}),
//         };

//         let message = match zero_val["message"]["content"].as_str() {
//             Some(val) => val,
//             None => "No message",
//         };
//         println!("Message: {}", message); //print
//     } else {
//         let err = resp.text().await.unwrap();
//         eprintln!("Error body: {}", err);
//     }
// }
pub async fn parse_response(model: Model) {
    match model {
        Model::OPENAI => {
            parse_openai_response().await;
        }
        Model::ANTHROPIC => {
            parse_anthropic_response().await;
        }
        //        Model::OLLAMA => {
        //            parse_ollama_response();
        //        }
        Model::OPENROUTER => {
            parse_openrouter_response().await;
        }
    }
}

async fn parse_openai_response() {
    // Implement the async logic for OpenAI response parsing
}

pub async fn parse_anthropic_response() {
    let url = ANTHROPIC_API_URL;
    // curl https://api.anthropic.com/v1/messages \
    //      --header "x-api-key: $ANTHROPIC_API_KEY" \
    //      --header "anthropic-version: 2023-06-01" \
    //      --header "content-type: application/json" \
    //      --data \
    // '{
    //     "model": "claude-3-5-sonnet-20241022",
    //     "max_tokens": 1024,
    //     "messages": [
    //         {"role": "user", "content": "Hello, world"}
    //     ]
    //
    let api_key = "<API>";
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
        .header("x-api-key", format!("Bearer {}", api_key))
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&req)
        .send()
        .await
        .unwrap();

    if resp.status().is_success() {
        //let resp_text = resp.text().await.unwrap();
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

fn parse_ollama_response() {
    todo!("Implement Ollama Response Parsing")
}

async fn parse_openrouter_response() {
    // Implement the async logic for OpenRouter response parsing
}

//match
//  let url = constants::api::OPENAI_API_URL;
//  let req = json!({
//      "model":"gpt-4o-mini",
//      "messages": [{ "role": "user", "content": "Say this is a test" }],
//      "store": true,
//      "stream": false,
//  });
//  let client  = Client::new();

//  let resp:Response = client.post(url).header("Content-Type", "application/json")
//         .header("Authorization", format!("Bearer {}", api_key))
//         .json(&req)
//         .send()
//         .await.unwrap();
// // let resp_text = resp.text().await.unwrap();

//  if resp.status().is_success(){
//     //println!("{:?}", resp);
//     // let json_resp = resp.json().await.unwrap();

//      let json_resp: Value = resp.json().await.unwrap();
//      // println!("{:?}", json_resp);
//      // if let Some(first_choice) = json_resp["choices"].get(0) { //if this arr has content at 0h index
//      //     if let Some(content) = first_choice["message"]["content"].as_str() { //if content has key "message"
//      //         println!("Message: {}", content); //print
//      //     }
//      // }
//      //
//      let zero_val = match json_resp["choices"].get(0){  //if this arr has content at 0h index
//          Some(val) => val,
//          None=> &json!({
//              "e":"e"})
//      };

//      let message = match zero_val["message"]["content"].as_str(){
//          Some(val) => val,
//          None=> "No message"
//      };
//      println!("Message: {}", message); //print
//  } else{
//      let err = resp.text().await.unwrap();
//      eprintln!("Error body: {}", err);

//  }
