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
                image_data: include_bytes!("../../logo.png"),
                sha256sum: Some("a7163048ee584787222302a945a9963492b8721f963b92be6cbf35cb28be5590"), // We can add hash later if needed
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
