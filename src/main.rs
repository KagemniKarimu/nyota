// #[macro_use]
// extern crate dotenv_codegen;
mod api;
mod cli;
mod snd;
mod tui;

use anyhow::Result;
use api::adapter::*;
use ratatui::{backend::CrosstermBackend, Terminal};
use snd::player::play_welcome_chirp;
use std::io::Stdout;
use tui::interactive::ChatInterface;
use tui::menu::MenuAction;
use tui::{banner::get_banner, banner::get_version_plaque, menu::Menu, splash::SplashScreen};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("{}", get_banner());
    println!("{}", get_version_plaque());

    let api_adapter = Adapter::new();

    println!("{:?}", api_adapter);
    // let mode_input = cli::modes::get_mode_input();
    // match mode_input.mode {
    //    Mode::Development => handle_development(),
    //    Mode::Interactive => handle_interactive(),
    //    Mode::Task => handle_task(),
    //    Mode::Menu => handle_menu().await,
    //}
    Ok(())
}

async fn display_splash_screen(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    play_welcome_chirp();
    let splash = SplashScreen::new();
    splash.show(terminal)
}

fn display_main_menu(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<MenuAction> {
    let mut menu = Menu::new();
    match menu.run(terminal)? {
        MenuAction::Interactive => handle_interactive()?,
        MenuAction::Task => handle_task()?,
        MenuAction::Development => handle_development()?,
        MenuAction::Help => {
            todo!("  /* TODO: Implement help */ ")
        }
        MenuAction::About => {
            todo!("/* TODO: Implement about */")
        }
        MenuAction::Exit => {}
    }
    Ok(MenuAction::Exit)
}

async fn handle_menu() -> Result<()> {
    // Initialize terminal once
    let mut terminal = ratatui::init();

    display_splash_screen(&mut terminal).await?;
    display_main_menu(&mut terminal)?;

    // Cleanup
    ratatui::restore();
    Ok(())
}

fn handle_task() -> Result<()> {
    todo!("TODO:implement handle task");
}

fn handle_interactive() -> Result<()> {
    // Initialize terminal
    let mut terminal = ratatui::init();

    // Create and run chat interface
    let mut chat = ChatInterface::new();
    let result = chat.run(&mut terminal);

    // Cleanup
    ratatui::restore();

    result
}

fn handle_development() -> Result<()> {
    todo!("TODO:implement handle development");
}
