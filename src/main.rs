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
use std::sync::Arc;
use tokio::sync::Mutex;

use nyota::lex::thought::{StreamOfThought, MENTAL_CONCEPT_ARCHETYPE_FILE};
use std::io;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::Stdout, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("{}", get_banner());
    println!("{}", get_version_plaque());

    AudioControl::init(DEFAULT_MUTE, DEFAULT_VOLUME).await;

    let sentiment = Sentiment::new();

    // Create a shared instance of StreamOfThought.
    let stream = Arc::new(Mutex::new(StreamOfThought::new(
        MENTAL_CONCEPT_ARCHETYPE_FILE,
    )));

    // Spawn a concurrent task that makes the stream "think" repeatedly.
    let thinker = Arc::clone(&stream);
    tokio::spawn(async move {
        loop {
            {
                let mut s = thinker.lock().await;
                s.think().await;
                println!("Time: {}", s.last_thought_time.format("%H:%M:%S"));
                println!("Thoughts: {}\n---", s.last_thought_stream.join(" "));
            }
            sleep(Duration::from_millis(50)).await;
        }
    });

    let test_cases = [
        "I love you Nyota",
        "You are my favorite bot!",
        "Good bot! I appreciate what you do.",
        "Thanks for all your work",
        "I love you Nyota",
        "You are my favorite bot!",
        "Good bot! I appreciate what you do.",
        "Thanks for all your work",
        "I love you Nyota",
        "You are my favorite bot!",
        "Good bot! I appreciate what you do.",
        "Thanks for all your work",
        "I love you Nyota",
        "You are my favorite bot!",
        "Good bot! I appreciate what you do.",
        "Thanks for all your work",
        "I love you Nyota",
        "You are my favorite bot!",
        "Good bot! I appreciate what you do.",
        "Thanks for all your work",
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

    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Type a new concept, 'show' to see frequent thoughts, or 'quit' to exit:");

    while let Some(Ok(line)) = stdin.next() {
        let input = line.trim().to_string();
        if input.eq_ignore_ascii_case("quit") {
            break;
        } else if input.eq_ignore_ascii_case("show") {
            // For convenience, we'll show the top 5 frequent thoughts.
            let s = stream.lock().await;
            let frequent = s.get_frequent_thoughts(5);
            println!("Frequent thoughts:");
            for thought in frequent {
                println!(" - {}", thought);
            }
        } else {
            // Otherwise, treat the input as a new concept.
            let mut s = stream.lock().await;
            if let Err(err) = s.learn_new_concept(input.clone()) {
                eprintln!("Error adding concept {}: {:?}", input, err);
            } else {
                println!("Added new concept: {}", input);
            }
        }
    }

    println!("Exiting...");

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
