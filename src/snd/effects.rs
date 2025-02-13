//! Sound effects for the audio system.
//! The sound effects are used to provide audio feedback to the user during program execution.
//! The general pattern is to use `rodio` interfaces to generate chirps and noises.
//! An enum (`SoundEffects`) is used to represent the different sound effects that can be played.
//! Variants are defined for each sound effect, and each variant has a corresponding method that creates a `Source` for the effect.
use rodio::cpal::SampleRate;
use rodio::source::chirp;
use rodio::source::{PinkNoise, SineWave, Source};
use std::time::Duration;

/// The different sound effects that can be played by the audio system.
/// Each sound effect needs a corresponding method that creates a `Source` for the effect.
/// The `Source` can be played asynchronously using the `rodio` crate.
/// The sound effects are used to provide audio feedback to the user during program execution.
pub enum SoundEffects {
    /// A welcome chirp sound effect, played when the program starts.
    WelcomeChirp,
    /// A menu toggle sound effect, played when a menu is toggled.
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

/// Implementation of the `SoundEffects` enum.
/// Each variant of the enum has a corresponding method that creates a `Source` for the effect.
impl SoundEffects {
    /// Creates a `rodio::source::Source` for the sound effect. The `Source` can be played asynchronously using the `rodio` crate.
    /// The `Source` generates a waveform that corresponds to the sound effect.
    /// The waveform is then played through the audio system to provide audio feedback to the user.
    /// The `Source` is created based on the parameters of the sound effect.
    /// The `Source` is returned as a boxed trait object to allow for dynamic dispatch.
    /// The `Source` is also marked as `Send` to allow for asynchronous playback.
    /// The `Source` is also marked as `'static` to allow for static dispatch.
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
