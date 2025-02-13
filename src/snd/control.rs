//! Audio control module for nyota.
//! This module provides an interface for controlling the audio system in nyota.
//! It provides functions for playing sound effects and controlling the volume and mute state of the audio system.
//! The audio system is implemented using the `rodio` crate.
//! The `AudioControl` struct provides several convenience methods for controlling the audio system.
//! It follows the singleton pattern and can be accessed directly using the `AUDIO_CONTROL` static variable or indirectly using the `AudioControl::global()` method.

use anyhow::Error;
use once_cell::sync::OnceCell;
use rodio::{source::Source, OutputStream, Sink};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use super::constants::{MuteState, VolumeLevel, MAX_VOLUME, MIN_VOLUME};
use super::effects::SoundEffects;

/// The global audio control instance. This is a `OnceCell` that is used to store the `AudioControl` instance. It is initialized lazily and direct access from outside the module is discouraged.
static AUDIO_CONTROL: OnceCell<AudioControl> = OnceCell::new();

/// `AudioControl` is a struct for controlling the audio system in Nyota. It provides functions for playing sound effects and controlling the volume and mute state of the audio system.
#[derive(Debug)]
pub struct AudioControl {
    muted: AtomicBool,
    volume: AtomicU8,
}

/// Implementation of `AudioControl`. This implementation provides methods for controlling the audio system, such as playing sound effects, setting the volume, and muting the audio.
impl AudioControl {
    /// Creates a new `AudioControl` instance with the given mute state and volume level.
    /// This is the constructor for the `AudioControl` struct and should not be called directly.
    /// Use the `init()` method to initialize the audio system.
    fn new(muted: MuteState, volume: VolumeLevel) -> Self {
        Self {
            muted: AtomicBool::new(muted),
            volume: AtomicU8::new(volume.clamp(MIN_VOLUME, MAX_VOLUME)),
        }
    }

    /// Plays a sound effect with the given amplitude. The amplitude is a value between 0.0 and 1.0, where 0.0 is silent and 1.0 is full volume.
    /// The sound effect is played asynchronously using the `rodio` crate.
    /// This method is used internally by the other sound effect methods and should not be called directly.
    async fn play_sound(&self, effect: SoundEffects) -> Result<(), Error> {
        if !self.is_muted() {
            let play_volume = self.get_volume();
            let amplitude = play_volume as f32 / MAX_VOLUME as f32;

            tokio::spawn(async move {
                match OutputStream::try_default() {
                    Ok((_stream, stream_handle)) => {
                        if let Ok(sink) = Sink::try_new(&stream_handle) {
                            let source = effect.create_source().amplify(amplitude);
                            sink.append(source);
                            sink.sleep_until_end();
                        }
                    }
                    Err(e) => eprintln!("Audio error: {}", e),
                }
            });
        }
        Ok(())
    }

    /// Initializes the audio system with the given mute state and volume level.
    /// This method should be called once at the start of the program to initialize the audio system.
    /// It sets the global `AUDIO_CONTROL` OnceCell instance with the new `AudioControl` struct instance.
    /// If the audio system is already initialized, an error is returned.
    pub async fn init(muted: MuteState, volume: VolumeLevel) -> Result<(), Error> {
        println!("DEBUG: Initializing Audio: {}, {}", muted, volume);
        AUDIO_CONTROL
            .set(Self::new(muted, volume))
            .map_err(|audio| {
                Error::msg(format!(
                    "[AUDIO CONTROL] Error: AudioControl already initialized <muted ={}, volume={}>",
                    audio.is_muted(),
                    audio.get_volume()
                ))
            }).unwrap();
        if let Some(audio) = AUDIO_CONTROL.get() {
            println!("DEBUG: Audio initialized: {:#?}", audio,);
        }
        Ok(())
    }

    /// Returns a reference to the global `AudioControl` instance. This method is used to access the initialized `AudioControl` instance in other functions and if necessary, outside the module.
    pub fn global() -> Result<&'static Self, Error> {
        AUDIO_CONTROL
            .get()
            .ok_or_else(|| Error::msg("[AUDIO CONTROL] Error: AudioControl is unavailable."))
    }

    /// Returns the current mute state of the audio system. The `MuteState` is a type aliased boolean value indicating whether the audio is muted or not.
    pub fn is_muted(&self) -> MuteState {
        self.muted.load(Ordering::Relaxed)
    }

    /// Sets the mute state of the audio system to the given value. The `MuteState` is a type aliased boolean value indicating whether the audio is muted or not.
    /// Ordering is set to `Relaxed` for performance reasons such that the value is not synchronized across threads. This is acceptable for the audio system as it is not critical to have the most up-to-date value.
    pub fn set_muted(&self, muted: MuteState) {
        self.muted.store(muted, Ordering::Relaxed);
    }

    /// Returns the current volume level of the audio system. The `VolumeLevel` is a type aliased unsigned 8-bit integer value indicating the volume level of the audio system.
    pub fn get_volume(&self) -> VolumeLevel {
        self.volume.load(Ordering::Relaxed)
    }

    /// Sets the volume level of the audio system to the given value. The `VolumeLevel` is a type aliased unsigned 8-bit integer value indicating the volume level of the audio system.
    pub fn set_volume(&self, volume: VolumeLevel) {
        self.volume
            .store(volume.clamp(MIN_VOLUME, MAX_VOLUME), Ordering::Relaxed);
    }

    /// Plays the sound effect for a menu toggle action. This method plays the sound effect asynchronously using the `play_sound()` method.
    pub async fn play_menu_toggle() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::MenuToggle).await?;
        Ok(())
    }

    /// Plays the sound effect which turns on the program. This method plays the sound effect asynchronously using the `play_sound()` method.
    pub async fn play_welcome_chirp() -> Result<(), Error> {
        Self::global()?
            .play_sound(SoundEffects::WelcomeChirp)
            .await?;
        Ok(())
    }

    /// Plays the sound effect for a message sent action. This method plays the sound effect asynchronously using the `play_sound()` method.
    pub async fn play_message_sent_noise() -> Result<(), Error> {
        Self::global()?
            .play_sound(SoundEffects::MessageSent)
            .await?;
        Ok(())
    }

    pub async fn play_message_received_noise() -> Result<(), Error> {
        Self::global()?
            .play_sound(SoundEffects::MessageReceived)
            .await?;
        Ok(())
    }

    pub async fn play_error_sound() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::Error).await?;
        Ok(())
    }

    pub async fn play_connection_change_noise() -> Result<(), Error> {
        Self::global()?
            .play_sound(SoundEffects::ConnectionChange)
            .await?;
        Ok(())
    }

    pub async fn play_keystroke_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::Keystroke).await?;
        Ok(())
    }

    pub async fn play_keystroke2_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::Keystroke2).await?;
        Ok(())
    }

    pub async fn play_enter_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::Enter).await?;
        Ok(())
    }

    pub async fn play_backspace_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::Backspace).await?;
        Ok(())
    }

    pub async fn play_mode_normal_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::ModeNormal).await?;
        Ok(())
    }

    pub async fn play_mode_insert_noise() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::ModeInsert).await?;
        Ok(())
    }
}
