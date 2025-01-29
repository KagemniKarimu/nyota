// #[macro_use]
// extern crate dotenv_codegen;
mod cli;
mod tui;

use cli::modes::Mode;
use tui::banner::get_version_plaque;
use tui::{banner::get_banner, menu::Menu};
use tui::splash::SplashScreen;
use anyhow::Result;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::Stdout;



#[tokio::main]
 async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("{}", get_banner());
    println!("{}", get_version_plaque());

    let mode_input = cli::modes::get_mode_input();
    match mode_input.mode {
        Mode::Interactive => handle_interactive(),
        Mode::Development =>  handle_development(),
        Mode::Task => handle_task(),
        Mode::Menu => handle_menu(),
    }
}

fn display_splash_screen(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let splash = SplashScreen::new();
    splash.show(terminal)
}

fn display_main_menu(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut menu = Menu::new();
    match menu.run(terminal) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}

fn handle_menu() -> Result<()> {
    // Initialize terminal once
    let mut terminal = ratatui::init();

    display_splash_screen(&mut terminal)?;
    display_main_menu(&mut terminal)?;

    // Cleanup
    ratatui::restore();
    Ok(())
}

fn handle_task() -> Result<()> {
    todo!("TODO:implement handle task");
}

fn handle_interactive() -> Result<()> {
    todo!("TODO:implement handle interactive");
}

fn handle_development() -> Result<()> {
    todo!("TODO:implement handle development");
}
