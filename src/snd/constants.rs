//! This module contains default values for the sound system.
//! These values are used to initialize the sound system and are used as defaults when the user does not specify any values.

/// Type alias for the sound system's mute state.
pub type MuteState = bool;
/// Type alias for the sound system's volume level.
pub type VolumeLevel = u8;

/// Default volume level for the sound system. Used when the user does not specify a volume level during initialization.
pub const DEFAULT_VOLUME: u8 = 100;
/// Minimum volume level for the sound system - values below this are clamped to this value.
pub const MIN_VOLUME: u8 = 0;
/// Maximum volume level for the sound system - values above this are clamped to this value.
pub const MAX_VOLUME: u8 = 100;
/// Default mute state for the sound system. Used when the user does not specify a mute state during initialization.
pub const DEFAULT_MUTE: bool = false;
