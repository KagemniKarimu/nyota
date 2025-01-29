// #[macro_use]
// extern crate dotenv_codegen;
mod constants;
mod cli;
mod tui;

use cli::modes::Mode;
use tui::menu::Menu;
// use reqwest::{Client, Response};
// use serde_json::{json, Value};



#[tokio::main]
 async fn main() {
    dotenv::dotenv().ok();

    println!("{}", constants::ui::BANNER);
    println!("{}", constants::ui::VERSION_PLAQUE);

    let mode_input = cli::modes::get_mode_input();
    match mode_input.mode {
        Mode::Interactive => handle_interactive(), // handle interactive,
        Mode::Development =>  handle_development(),// handle development,
        Mode::Task => handle_task(),
        Mode::Menu => display_main_menu(), // handle menu,
    }
}

fn display_main_menu()  {
    let menu = Menu::new();
    match menu.run() {
        Ok(()) => {
            // Everything worked fine
        }
        Err(e) => {
            // Handle the error
            eprintln!("An error occurred: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_task() {
    todo!("TODO:implement handle task");
}

fn handle_interactive() {
    todo!("TODO:implement handle interactive");
}

fn handle_development() {
    todo!("TODO:implement handle development");
}
