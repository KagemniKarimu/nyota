use rodio::source::chirp;
use rodio::{
    source::{PinkNoise, SineWave, Source},
    OutputStream, Sink,
};
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
    fn create_source(&self) -> impl Source {
        match self {
            Self::WelcomeChirp => chirp(
                rodio::cpal::SampleRate(44100),
                600.0,
                900.0,
                Duration::from_millis(100),
            ),
            Self::MenuToggle => chirp(
                rodio::cpal::SampleRate(44100),
                200.0,
                240.0,
                Duration::from_millis(100),
            ),
            Self::Keystroke => chirp(
                rodio::cpal::SampleRate(44100),
                2000.0,
                2000.0,
                Duration::from_millis(10),
            ),
            Self::Keystroke2 => chirp(
                rodio::cpal::SampleRate(3000),
                2000.0,
                2000.0,
                Duration::from_millis(10),
            ),
            Self::Enter => chirp(
                rodio::cpal::SampleRate(44100),
                1000.0,
                1000.0,
                Duration::from_millis(100),
            ),
            Self::Backspace => PinkNoise::new(rodio::cpal::SampleRate(10))
                .take_duration(Duration::from_millis(20))
                .amplify(0.1),
            Self::ModeNormal => SineWave::new(800.0)
                .take_duration(Duration::from_millis(50))
                .amplify(0.15),
            Self::ModeInsert => SineWave::new(600.0)
                .take_duration(Duration::from_millis(80))
                .amplify(0.15),
            Self::MessageSent => SineWave::new(1200.0)
                .take_duration(Duration::from_millis(150))
                .amplify(0.2),
            Self::MessageReceived => SineWave::new(800.0)
                .take_duration(Duration::from_millis(100))
                .fade_in(Duration::from_millis(20))
                .amplify(0.15),
            Self::Error => SineWave::new(1000.0)
                .take_duration(Duration::from_millis(200))
                .amplify(0.25),
            Self::ConnectionChange => SineWave::new(600.0)
                .take_duration(Duration::from_millis(30))
                .amplify(0.15),
        }
    }
}
