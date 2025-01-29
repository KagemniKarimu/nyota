
use reqwest::Client;
use anyhow::{Result, Error};

pub enum Model
{
    OPENAI,
    ANTHROPIC,
    OLLAMA,
    OPENROUTER
}

pub struct Adapter{
    model: Model,
    api_key: String,
    default_ai_model: String
}




fn get_api_key(selected_model: Model) -> Result<String, Error>{
    let api_name = match selected_model {
        Model::OPENAI => {"OPENAI_API_KEY"}
        Model::ANTHROPIC=>{"ANTHROPIC_API_KEY"}
        Model::OLLAMA=>{Todo!("Implement Ollama Support")}
        Model::OPENROUTER=>{"OPENROUTER_API_KEY"}
    };
    match dotenv!(api_name){
        Ok(val) => val,
        Err(e) => {
            Error::msg("{:?} not found in .env file", api_name)
        }
    }
}

fn get_default_ai_model() -> Result<Model, Error>{
    let api_name = match dotenv!("DEFAULT_AI_MODEL"){
        Ok(val) => val,
        Err(e) => {
            Error::msg("Default AI model not found in .env file")
        }
    };

    match api_name {
        "OPENAI" => Model::OPENAI,
        "ANTHROPIC"=>Model::ANTHROPIC,
        "OLLAMA"=>{Todo!("Implement Ollama Support")},
        "OPENROUTER"=>Model::OPENROUTER
    };

}

fn construct_test_json(model: Model){



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


fn send_test_request(model: Model){
      let req = json!({
          "model":"gpt-4o-mini",
          "messages": [{ "role": "user", "content": "Say this is a test" }],
          "store": true,
          "stream": false,
      });
      let client  = Client::new();

      let resp:Response = client.post(url).header("Content-Type", "application/json")
          .header("Authorization", format!("Bearer {}", api_key))
          .json(&req)
          .send()
          .await.unwrap();

      let resp_text = resp.text().await.unwrap();

      if resp.status().is_success(){

      println!("{:?}", resp);
          let json_resp = resp.json().await.unwrap();

           let json_resp: Value = resp.json().await.unwrap();
           println!("{:?}", json_resp);
           if let Some(first_choice) = json_resp["choices"].get(0) { //if this arr has content at 0h index
               if let Some(content) = first_choice["message"]["content"].as_str() { //if content has key "message"
                   println!("Message: {}", content); //print
               }
           }

           let zero_val = match json_resp["choices"].get(0){  //if this arr has content at 0h index
               Some(val) => val,
               None=> &json!({
                   "e":"e"})
           };

           let message = match zero_val["message"]["content"].as_str(){
               Some(val) => val,
               None=> "No message"
           };
           println!("Message: {}", message); //print
       } else{
           let err = resp.text().await.unwrap();
           eprintln!("Error body: {}", err);
}

fn parse_response(model: Model){
    match model {
        Model::OPENAI => {
            parse_openai_response();
        }
        Model::ANTHROPIC=>{}
        Model::OLLAMA=>{}
        Model::OPENROUTER=>{}
    }
}

fn parse_openai_response(){

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
