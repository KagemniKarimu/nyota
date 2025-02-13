use crate::snd::control::AudioControl;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    prelude::{Alignment, Modifier, Stylize},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use std::io::Stdout;

/// Represents a menu option
struct MenuItem {
    title: String,
    description: String,
}

/// Our main menu structure
pub struct Menu {
    items: Vec<MenuItem>,
    state: ListState, // Keeps track of which item is selected
}

pub enum MenuAction {
    Interactive,
    Task,
    Development,
    Help,
    About,
    Exit,
}

impl Menu {
    /// Create a new menu with some items
    pub fn new() -> Self {
        // Create some example menu items
        let items = vec![
            MenuItem {
                title: String::from("Interactive Mode"),
                description: String::from(
                    "Start an interactive REPL session (Multi-turn Conversation)",
                ),
            },
            MenuItem {
                title: String::from("Task Mode"),
                description: String::from("Execute an isolated task (Single-turn Conversation)"),
            },
            MenuItem {
                title: String::from("Development Mode"),
                description: String::from("Start an interactive REPL session with raw outputs"),
            },
            MenuItem {
                title: String::from("Help"),
                description: String::from("Access NYOTA documentation"),
            },
            MenuItem {
                title: String::from("About"),
                description: String::from("Learn about Nyota"),
            },
            MenuItem {
                title: String::from("Exit"),
                description: String::from("Quit the application"),
            },
        ];

        // Create the menu with those items and initialize the state
        let mut menu = Self {
            items,
            state: ListState::default(),
        };

        // Set the initial selection to the first item
        menu.state.select(Some(0));

        menu
    }

    /// Move the selection down
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0 // Wrap around to the first item
                } else {
                    i + 1 // Move to next item
                }
            }
            None => 0, // If nothing is selected, select the first item
        };
        self.state.select(Some(i));
    }

    /// Move the selection up
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1 // Wrap around to the last item
                } else {
                    i - 1 // Move to previous item
                }
            }
            None => 0, // If nothing is selected, select the first item
        };
        self.state.select(Some(i));
    }

    /// Draw the menu on the screen
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        // Add a fancy title block
        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double) // ═══ style border
            .border_style(Style::default().fg(Color::Cyan).bold())
            .title(" Nyota ") // Space padding for aesthetics
            .title_alignment(Alignment::Center);

        // Style the menu items
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| {
                // Create a styled title
                let title = Line::from(vec![Span::styled(
                    &item.title,
                    Style::default().fg(Color::White).bold(),
                )]);

                // Create a styled description
                let description = Line::from(vec![Span::styled(
                    &item.description,
                    Style::default().fg(Color::Gray), // Dimmer color for description
                )]);

                // Combine them into a single ListItem with spacing
                ListItem::new(vec![title, description, Line::from("")]) // Empty line for spacing
            })
            .collect();

        // Create styled list
        let list = List::new(items)
            .block(title_block)
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ✨ ");

        // Render the list widget with our state
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<MenuAction> {
        // Application loop
        loop {
            // Draw the current state
            terminal.draw(|frame| {
                // Use the entire screen area
                self.draw(frame, frame.area());
            })?;

            // Handle keyboard input
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            // Quit if user presses 'q' or Esc
                            break;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            // Move selection down
                            AudioControl::play_menu_toggle().await?;
                            self.next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            // Move selection up
                            AudioControl::play_menu_toggle().await?;
                            self.previous();
                        }
                        KeyCode::Enter => {
                            // Convert menu selection to action
                            if let Some(selected) = self.state.selected() {
                                return Ok(match selected {
                                    0 => MenuAction::Interactive,
                                    1 => MenuAction::Task,
                                    2 => MenuAction::Development,
                                    3 => MenuAction::Help,
                                    4 => MenuAction::About,
                                    5 => MenuAction::Exit,
                                    _ => MenuAction::Exit,
                                });
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Restore terminal
        ratatui::restore();
        Ok(MenuAction::Exit)
    }
}
