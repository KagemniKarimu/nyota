//! # Sound Module (`nyota::snd`)
//! This module is the sound module for `nyota`. This module contains the complete implementation of the sound system.
//! It provides functions for global initialization, audio control, and playing sound effects.
//! The sound system uses the `rodio` crate. This module provides modular and extensible interfaces for sound management.
//! Sound is used by nyota to communicate program state and important changes in the interface.
//! It can also be disabled if a user would like a silent experience.
//!
//!  ## Implementation Details
//! The sound system is initialized globally and can be accessed from any part of the program.
//! The sound system is controlled using the `AudioControl` struct, which provides methods for playing sound effects, controlling the volume, and muting the audio.
//! The sound effects are defined in the `SoundEffects` enum, which contains different sound effects that can be played by the audio system.
//! Each sound effect has a corresponding method that creates a `Source` for the effect.
//! The `Source` is played asynchronously using the `rodio` crate.
//!
//! ## Example
//! ```rust
//! use nyota::snd::control::AudioControl;
//! use nyota::snd::constants::{DEFAULT_MUTE, DEFAULT_VOLUME};
//!
//! // Initialize the audio system with sane defaults
//! // It is a global singleton and should be initialized only once at the start of the program.
//! AudioControl::init(DEFAULT_MUTE, DEFAULT_VOLUME);
//!
//! // Play a sound effect
//! AudioControl.play_welcome_chirp().await;
//!
//! // Increase the volume
//! let current_volume = AudioControl.get_volume();
//! AudioControl.set_volume(current_volume + 10);
//!
//! // Mute the audio
//! AudioControl.set_mute(true);
//!
//! ```
//!
//! ## Modules
//! - `control`: Audio control module for Nyota. This module provides an interface for controlling the audio system in Nyota.
//! - `effects`: Sound effects module for Nyota. This module provides an interface for playing sound effects in Nyota.
//! - `constants`: Default values for the sound system. These values are used to initialize the sound system and are used as defaults when the user does not specify any values.

pub mod constants;
pub mod control;
pub mod effects;
