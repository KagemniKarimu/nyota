// #[macro_use]
// extern crate dotenv_codegen;
mod constants;
mod cli;

use cli::modes::Mode;
// use reqwest::{Client, Response};
// use serde_json::{json, Value};



#[tokio::main]
 async fn main() {
    dotenv::dotenv().ok();

    println!("{}", constants::ui::BANNER);
    println!("{}", constants::ui::VERSION_PLAQUE);

    let mode_input = cli::modes::get_mode_input();
    match mode_input.mode {
        Mode::Interactive => println!("TODO:implement handle interactive"), // handle interactive,
        Mode::Development =>  println!("TODO:implement handle development mode"),// handle development,
        Mode::Task => println!("TODO:implement handle task mode"), // handle task,
        Mode::Menu => println!("TODO:implement handle no flag / main menu"), // handle menu,
    }



}
