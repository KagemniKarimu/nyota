#[macro_use]
extern crate dotenv_codegen;

use reqwest::{Client, Response};
use serde_json::{json, Value};

#[tokio::main]
 async fn main() {
     dotenv::dotenv().ok();


    println!("
                                         █████             
                                        ░░███              
         ████████   █████ ████  ██████  ███████    ██████  
        ░░███░░███ ░░███ ░███  ███░░███░░░███░    ░░░░░███ 
         ░███ ░███  ░███ ░███ ░███ ░███  ░███      ███████ 
         ░███ ░███  ░███ ░███ ░███ ░███  ░███ ███ ███░░███ 
         ████ █████ ░░███████ ░░██████   ░░█████ ░░████████
        ░░░░ ░░░░░   ░░░░░███  ░░░░░░     ░░░░░   ░░░░░░░░ 
                     ███ ░███                              
                    ░░██████                               
                     ░░░░░░                                 ");

    println!("
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
┌╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶┐
╎                                 ╎
╎    version:  v0.1.0             ╎
╎                                 ╎
╎    authors: DariaAG             ╎
╎             KagemniKarimu       ╎
╎                                 ╎
╎                                 ╎
└╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶┘
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
");

    let api_key = dotenv!("OPEN_AI_API_KEY");
    // let api_key = match std::env::var("OPEN_AI_API_KEY"){
    //     Ok(String) => api_key,
    //     Error(e) => {
    //         println!("Error: {}", e);
    //         std::process::exit(1);
    //         }



    let url = "https://api.openai.com/v1/chat/completions";
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
   // let resp_text = resp.text().await.unwrap();

    if resp.status().is_success(){
       //println!("{:?}", resp);
       // let json_resp = resp.json().await.unwrap();

        let json_resp: Value = resp.json().await.unwrap();
        // println!("{:?}", json_resp);
        // if let Some(first_choice) = json_resp["choices"].get(0) { //if this arr has content at 0h index
        //     if let Some(content) = first_choice["message"]["content"].as_str() { //if content has key "message"
        //         println!("Message: {}", content); //print
        //     }
        // }
        //
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

}
