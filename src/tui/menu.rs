use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io::Stdout;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use color_eyre::Result;

/// Represents a menu option
struct MenuItem {
    title: String,
    description: String,
}

/// Our main menu structure
pub struct Menu {
    items: Vec<MenuItem>,
    state: ListState,  // Keeps track of which item is selected
}

impl Menu {
    /// Create a new menu with some items
    pub fn new() -> Self {
        // Create some example menu items
        let items = vec![
            MenuItem {
                title: String::from("Interactive Mode"),
                description: String::from("Start an interactive chat session"),
            },
            MenuItem {
                title: String::from("Development Mode"),
                description: String::from("Access development tools"),
            },
            MenuItem {
                title: String::from("Task Mode"),
                description: String::from("Run specific tasks"),
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
                    0  // Wrap around to the first item
                } else {
                    i + 1  // Move to next item
                }
            }
            None => 0,  // If nothing is selected, select the first item
        };
        self.state.select(Some(i));
    }

    /// Move the selection up
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1  // Wrap around to the last item
                } else {
                    i - 1  // Move to previous item
                }
            }
            None => 0,  // If nothing is selected, select the first item
        };
        self.state.select(Some(i));
    }

    /// Draw the menu on the screen
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        // First, let's create a block that will serve as our menu's border and title
        let menu_block = Block::default()
            .title("Nyota Menu")  // Add a title to our menu
            .borders(Borders::ALL);  // Add borders on all sides

        // Now let's prepare our menu items for display
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| {
                // Convert each MenuItem into a ListItem
                ListItem::new(format!("{}\n{}", item.title, item.description))
            })
            .collect();

        // Create the actual list widget
        let list = List::new(items)
            .block(menu_block)  // Use our block from above
            .highlight_style(  // Style for the selected item
                Style::default()
                    .bg(Color::Blue)  // Blue background
                    .fg(Color::Black)  // Black text
            )
            .highlight_symbol(">> ");  // Show an arrow before selected item

        // Render the list widget with our state
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {

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
                             self.next();
                         }
                         KeyCode::Up | KeyCode::Char('k') => {
                             // Move selection up
                             self.previous();
                         }
                         KeyCode::Enter => {
                             // Here you would handle menu item selection
                             // For now, we'll just break
                             break;
                         }
                         _ => {}
                     }
                 }
             }
         }

         // Restore terminal
         ratatui::restore();
         Ok(())
     }
 }
