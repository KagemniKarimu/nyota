//! Splash screen management module for the application.
//! This module provides the `SplashScreen` struct which is used to display a splash screen with the logo file when the application is launched.
//! It is currently hard-coded to display the current version of the logo file. Loading anything else could produce errors.
//! In theory, the splash screen can be used throughout the program.
use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_splash_screen::{SplashConfig, SplashScreen as RatatuiSplash};
use std::io::Stdout;
use std::time::Duration;

/// `SplashScreen` is a struct for displaying a splash screen with the logo file when the application is launched.
pub struct SplashScreen {
    config: SplashConfig<'static>,
}

impl SplashScreen {
    /// Creates a new `SplashScreen` instance with the default configuration.
    /// The default configuration includes the logo file and the number of render steps.
    pub fn new() -> Self {
        Self {
            config: SplashConfig {
                image_data: include_bytes!("../../logo_with_text_small.png"),
                sha256sum: Some("5750a49fe90b697f5f2e1be2a522f8eaad92310e9056ce74e5c090bb9d0b42c9"),
                render_steps: 6,
                use_colors: true,
            },
        }
    }

    /// Displays the splash screen with the logo file when called.
    /// The splash screen is displayed using the `ratatui` crate and the `CrosstermBackend` struct.
    pub fn show(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let mut splash = RatatuiSplash::new(self.config)?;

        while !splash.is_rendered() {
            terminal.draw(|frame| {
                frame.render_widget(&mut splash, frame.area());
            })?;
            std::thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }
}
