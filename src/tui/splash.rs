use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_splash_screen::{SplashConfig, SplashScreen as RatatuiSplash};
use std::io::Stdout;
use std::time::Duration;

pub struct SplashScreen {
    config: SplashConfig<'static>,
}

impl SplashScreen {
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
