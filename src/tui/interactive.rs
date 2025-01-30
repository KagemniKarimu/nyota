use crate::snd::player::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::collections::HashMap;
use std::io::Stdout;
use tui_textarea::TextArea;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ChatId(String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    Visual,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum ConnectionStatus {
    #[default]
    Connected,
    Disconnected,
    Thinking,
    Error,
}

#[derive(Default)]
struct StatusLine {
    mode: InputMode,
    connection_status: ConnectionStatus,
    message_count: usize,
    current_model: String,
}

/// Represents a single message in the chat
#[derive(Clone, Debug)]
pub struct Message {
    content: String,
    timestamp: DateTime<Utc>,
    is_user: bool, // true if from user, false if from bot
}

/// Main chat interface state
pub struct ChatInterface<'a> {
    /// History of all messages
    messages: Vec<Message>,
    /// Text input area
    input: TextArea<'a>,
    /// Whether the interface should exit
    should_quit: bool,
    /// Status information
    status: StatusLine,
}

impl<'a> ChatInterface<'a> {
    pub fn new() -> Self {
        let mut input = TextArea::default();
        input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Input "),
        );

        Self {
            messages: Vec::new(),
            input,
            should_quit: false,
            status: StatusLine {
                mode: InputMode::Normal,
                connection_status: ConnectionStatus::Connected,
                message_count: 0,
                current_model: "GPT-4".to_string(),
            },
        }
    }

    /// Creates the main layout divisions with status line
    fn create_layout(area: Rect) -> (Rect, Rect, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Message area (expands)
                Constraint::Length(5), // Input area (fixed)
                Constraint::Length(1), // Status line (single line)
            ])
            .split(area);

        (chunks[0], chunks[1], chunks[2])
    }

    fn render_status_line(&self, frame: &mut Frame, area: Rect) {
        let mode_indicator = match self.status.mode {
            InputMode::Normal => Span::styled("NORMAL", Style::default().fg(Color::Green)),
            InputMode::Insert => Span::styled("INSERT", Style::default().fg(Color::Yellow)),
            InputMode::Visual => Span::styled("VISUAL", Style::default().fg(Color::Blue)),
        };

        let connection_indicator = match self.status.connection_status {
            ConnectionStatus::Connected => Span::styled("●", Style::default().fg(Color::Green)),
            ConnectionStatus::Disconnected => Span::styled("●", Style::default().fg(Color::Red)),
            ConnectionStatus::Thinking => Span::styled("●", Style::default().fg(Color::Yellow)),
            ConnectionStatus::Error => Span::styled("●", Style::default().fg(Color::Red)),
        };

        let msg_count = Span::styled(
            format!("Messages: {}", self.status.message_count),
            Style::default().fg(Color::Cyan),
        );

        let model = Span::styled(
            format!("Model: {}", self.status.current_model),
            Style::default().fg(Color::Magenta),
        );

        let status_line = Line::from(vec![
            Span::raw(" "),
            mode_indicator,
            Span::raw(" | "),
            connection_indicator,
            Span::raw(" | "),
            msg_count,
            Span::raw(" | "),
            model,
        ]);

        let status_widget = Paragraph::new(status_line)
            .style(Style::default().bg(Color::DarkGray))
            .block(Block::default());

        frame.render_widget(status_widget, area);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let (message_area, input_area, status_area) = Self::create_layout(area);

        // Render message history area
        let messages_block = Block::default()
            .title(" Messages ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));
        frame.render_widget(messages_block, message_area);

        // Render input area
        frame.render_widget(self.input.widget(), input_area);

        // Render status line
        self.render_status_line(frame, status_area);
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|frame| {
                self.render(frame, frame.area());
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        play_message_sent(); // Play sound when sending message
                                             // Handle message sending
                    }
                    KeyCode::Backspace => {
                        play_enter();
                        play_backspace(); // Play sound when deleting
                        self.input.input(key);
                    }
                    _ => {
                        play_keystroke2(); // Play sound for normal keystrokes
                        self.input.input(key);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Manages multiple chat interfaces
pub struct ChatManager<'a> {
    /// All active chat sessions
    chats: HashMap<ChatId, ChatInterface<'a>>,
    /// Currently focused chat
    active_chat: ChatId,
    /// Input mode (Normal, Insert, Visual - vim-like)
    mode: InputMode,
}

impl<'a> ChatManager<'a> {
    pub fn new() -> Self {
        let default_chat = ChatId("main".to_string());
        let mut chats = HashMap::new();
        chats.insert(default_chat.clone(), ChatInterface::new());

        Self {
            chats,
            active_chat: default_chat,
            mode: InputMode::Normal,
        }
    }

    /// Switch to a different chat
    pub fn switch_chat(&mut self, id: ChatId) {
        if !self.chats.contains_key(&id) {
            self.chats.insert(id.clone(), ChatInterface::new());
        }
        self.active_chat = id;
    }

    /// Get the current active chat
    pub fn current_chat(&mut self) -> &mut ChatInterface<'a> {
        self.chats
            .get_mut(&self.active_chat)
            .expect("Active chat should always exist")
    }
}
