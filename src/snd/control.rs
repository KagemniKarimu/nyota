use anyhow::Error;
use once_cell::sync::OnceCell;
use rodio::{source::Source, OutputStream, Sink};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use super::constants::{MuteState, VolumeLevel, MAX_VOLUME, MIN_VOLUME};
use super::effects::SoundEffect;

static AUDIO_CONTROL: OnceCell<AudioControl> = OnceCell::new();

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

    fn play_sound(&self, effect: SoundEffect) -> Result<(), Error> {
        if !self.is_muted() {
            let volume = self.get_volume();
            let amplitude = volume as f32 / 100.0;

            tokio::spawn(async move {
                let (_stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle)?;

                let source = effect.create_source().amplify(amplitude);
                sink.append(source);
                sink.sleep_until_end();
            });
        }
        Ok(())
    }

    pub fn init(muted: MuteState, volume: VolumeLevel) -> Result<(), Error> {
        AUDIO_CONTROL
            .set(Self::new(muted, volume))
            .map_err(|audio| {
                Error::msg(format!(
                    "[AUDIO CONTROL] Error: AudioControl already initialized <muted ={}, volume={}>",
                    audio.is_muted(),
                    audio.get_volume()
                ))
            })
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

    pub fn play_menu_toggle() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffect::MenuToggle);
        Ok(())
    }

    pub fn play_welcome() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffect::Welcome);
        Ok(())
    }

    pub fn play_message_sent() -> Result<(), Error> {
        Self::global()?.play_sound(SoundEffect::MessageSent);
        Ok(())
    }
}
