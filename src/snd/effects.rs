use rodio::cpal::SampleRate;
use rodio::source::chirp;
use rodio::source::{PinkNoise, SineWave, Source};
use std::time::Duration;

pub enum SoundEffects {
    WelcomeChirp,
    MenuToggle,
    Keystroke,
    Keystroke2,
    Enter,
    Backspace,
    ModeNormal,
    ModeInsert,
    MessageSent,
    MessageReceived,
    Error,
    ConnectionChange,
}

impl SoundEffects {
    pub fn create_source(&self) -> Box<dyn Source<Item = f32> + Send + 'static> {
        match self {
            Self::WelcomeChirp => Box::new(
                chirp(SampleRate(44100), 600.0, 900.0, Duration::from_millis(100))
                    .take_duration(Duration::from_secs_f32(0.3)),
            ),
            Self::MenuToggle => Box::new(
                chirp(SampleRate(44100), 200.0, 240.0, Duration::from_millis(100))
                    .take_duration(Duration::from_secs_f32(0.3)),
            ),
            Self::Keystroke => Box::new(
                chirp(SampleRate(44100), 2000.0, 2000.0, Duration::from_millis(10))
                    .take_duration(Duration::from_millis(10)),
            ),
            Self::Keystroke2 => Box::new(
                PinkNoise::new(SampleRate(3000))
                    .take_duration(Duration::from_millis(10))
                    .amplify(0.1),
            ),
            Self::Enter => Box::new(
                SineWave::new(1000.0)
                    .take_duration(Duration::from_millis(100))
                    .amplify(0.2),
            ),
            Self::Backspace => Box::new(
                PinkNoise::new(SampleRate(10))
                    .take_duration(Duration::from_millis(20))
                    .amplify(0.1),
            ),
            Self::ModeNormal => Box::new(
                SineWave::new(800.0)
                    .take_duration(Duration::from_millis(50))
                    .amplify(0.15),
            ),
            Self::ModeInsert => Box::new(
                SineWave::new(600.0)
                    .take_duration(Duration::from_millis(80))
                    .amplify(0.15),
            ),
            Self::MessageSent => Box::new(
                SineWave::new(1200.0)
                    .take_duration(Duration::from_millis(150))
                    .amplify(0.2),
            ),
            Self::MessageReceived => Box::new(
                SineWave::new(800.0)
                    .take_duration(Duration::from_millis(100))
                    .fade_in(Duration::from_millis(20))
                    .amplify(0.15),
            ),
            Self::Error => Box::new(
                SineWave::new(1000.0)
                    .take_duration(Duration::from_millis(200))
                    .amplify(0.25),
            ),
            Self::ConnectionChange => Box::new(
                SineWave::new(600.0)
                    .take_duration(Duration::from_millis(30))
                    .amplify(0.15),
            ),
        }
    }
}
