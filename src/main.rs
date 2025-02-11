use nyota::api::utilities::*;
use nyota::cli::modes::*;
use nyota::snd::player::*;
use nyota::tui::banner::*;
use nyota::tui::interactive::*;
use nyota::tui::menu::*;
use nyota::tui::splash::*;

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::Stdout, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("{}", get_banner());
    println!("{}", get_version_plaque());

    let default_adapter = Adapter::new();
    let mode_input = get_mode_input();
    match mode_input.mode {
        Mode::Development => handle_development(default_adapter),
        Mode::Interactive => handle_interactive(default_adapter).await,
        Mode::Task => handle_task(default_adapter),
        Mode::Menu => handle_menu(default_adapter).await,
    }
}

async fn display_splash_screen(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let splash = SplashScreen::new();
    splash.show(terminal)
}

async fn display_main_menu(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    api_adapter: Adapter,
) -> Result<MenuAction> {
    let mut menu = Menu::new();
    match menu.run(terminal)? {
        MenuAction::Interactive => handle_interactive(api_adapter).await?,
        MenuAction::Task => handle_task(api_adapter)?,
        MenuAction::Development => handle_development(api_adapter)?,
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

async fn handle_menu(api_adapter: Adapter) -> Result<()> {
    play_welcome_chirp();
    sleep(Duration::from_millis(500)).await; // DEBUG - sleep so we can read initialisation messages

    // Initialize terminal once
    let mut terminal = ratatui::init();

    display_splash_screen(&mut terminal).await?;
    display_main_menu(&mut terminal, api_adapter).await?;

    // Cleanup
    ratatui::restore();
    Ok(())
}

fn handle_task(_api_adapter: Adapter) -> Result<()> {
    todo!("TODO:implement handle task");
}

async fn handle_interactive(api_adapter: Adapter) -> Result<()> {
    // Initialize terminal
    let mut terminal = ratatui::init();

    // Create and run chat interface
    let mut chat = ChatInterface::new(api_adapter).await;
    let result = chat.run(&mut terminal).await;

    // Cleanup
    ratatui::restore();

    result
}

fn handle_development(_api_adapter: Adapter) -> Result<()> {
    todo!("TODO:implement handle development");
}
