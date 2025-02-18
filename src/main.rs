use nyota::api::utilities::*;
use nyota::cli::modes::*;
use nyota::lex::sentiment::Sentiment;
use nyota::lex::*;
use nyota::snd::constants::{DEFAULT_MUTE, DEFAULT_VOLUME};
use nyota::snd::control::AudioControl;
use nyota::tui::banner::*;
use nyota::tui::interactive::*;
use nyota::tui::menu::*;
use nyota::tui::splash::*;

use nyota::lex::thought::{StreamOfThought, MENTAL_CONCEPT_ARCHETYPE_FILE};

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::Stdout, time::Duration};
use thought::test_mental_concepts;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("{}", get_banner());
    println!("{}", get_version_plaque());

    AudioControl::init(DEFAULT_MUTE, DEFAULT_VOLUME).await;

    let sentiment = Sentiment::new();

    let test_cases = [
        "Answer this damn question",
        "You pissing me the hell off",
        "Who the fuck you think you talking to!",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "Answer this damn question",
        "You pissing me the hell off",
        "Who the fuck you think you talking to!",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "Answer this damn question",
        "You pissing me the hell off",
        "Who the fuck you think you talking to!",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
        "I don't KNOW WHAT YOUR DAMN PROBLEM IS",
    ];

    for msg in test_cases.iter() {
        sentiment.process_emotion(msg).await.unwrap();
        let state = sentiment.get_feelings().await.unwrap();
        let mood = sentiment.get_mood().await.unwrap();

        println!(
            "\nMessage: {}\nCompound: {:.9}\nMood: {:?}",
            msg, state.compound_affect, mood,
        );
    }

    let mut stream = StreamOfThought::new(MENTAL_CONCEPT_ARCHETYPE_FILE);

    println!("Starting thought stream simulation...");
    println!("Press Ctrl+C to stop");

    // Run for a while to observe thought patterns
    loop {
        stream.think();

        // Print the timestamp and the thought stream
        println!("Time: {}", stream.last_thought_time.format("%H:%M:%S"));
        println!("Thoughts: {}", stream.last_thought_stream.join(" "));
        println!("---"); // Separator for readability
    }

    //    Ok(())
    //    test_mental_concepts();
    //    let default_adapter = Adapter::new();
    //   let mode_input = get_mode_input();
    //    match mode_input.mode {
    //        Mode::Development => handle_development(default_adapter),
    ////        Mode::Interactive => handle_interactive(default_adapter).await,
    ////        Mode::Task => handle_task(default_adapter),
    //        Mode::Menu => handle_menu(default_adapter).await,
    //    }
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
    match menu.run(terminal).await? {
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
    AudioControl::play_welcome_chirp().await?;
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
