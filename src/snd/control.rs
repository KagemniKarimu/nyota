use anyhow::Error;
use once_cell::sync::OnceCell;
use rodio::{source::Source, OutputStream, Sink};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use super::constants::{MuteState, VolumeLevel, MAX_VOLUME, MIN_VOLUME};
use super::effects::SoundEffects;

pub static AUDIO_CONTROL: OnceCell<AudioControl> = OnceCell::new();

#[derive(Debug)]
pub struct AudioControl {
    muted: AtomicBool,
    volume: AtomicU8,
}

impl AudioControl {
    fn new(muted: MuteState, volume: VolumeLevel) -> Self {
        Self {
            muted: AtomicBool::new(muted),
            volume: AtomicU8::new(volume.clamp(MIN_VOLUME, MAX_VOLUME)),
        }
    }

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

    pub fn global() -> Result<&'static Self, Error> {
        AUDIO_CONTROL
            .get()
            .ok_or_else(|| Error::msg("[AUDIO CONTROL] Error: AudioControl is unavailable."))
    }

    pub fn is_muted(&self) -> MuteState {
        self.muted.load(Ordering::Relaxed)
    }

    pub fn set_muted(&self, muted: MuteState) {
        self.muted.store(muted, Ordering::Relaxed);
    }

    pub fn get_volume(&self) -> VolumeLevel {
        self.volume.load(Ordering::Relaxed)
    }

    pub fn set_volume(&self, volume: VolumeLevel) {
        self.volume
            .store(volume.clamp(MIN_VOLUME, MAX_VOLUME), Ordering::Relaxed);
    }

    pub async fn play_menu_toggle() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffects::MenuToggle).await?;
        Ok(())
    }

    pub async fn play_welcome_chirp() -> Result<(), Error> {
        Self::global()?
            .play_sound(SoundEffects::WelcomeChirp)
            .await?;
        Ok(())
    }

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
