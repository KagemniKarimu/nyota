//! # Textual User Interface Module (`nyota::tui`)
//! This module is the textual user interface module for `nyota`. This module contains the complete implementation of the textual user interface.
//! It provides functions for displaying banners, menus, and interactive interfaces.
//! The textual user interface is used by `nyota` to provide a user-friendly and interactive experience.
//! It can be used to display information, receive user input, and provide feedback to the user.
//!
//! ## Implementation Details
//! The textual user interface is implemented with `ratatui` crate as its base. This module provides modular and extensible interfaces for building text-based user interfaces.
//! The textual user interface is divided into several submodules, each of which provides a different aspect of the user interface.
//! `ratatui` provides the building blocks for creating interactive interfaces, such as windows, buttons, text input fields, and more.
//!  The `tui` is encapsulated and can be avoided if a user prefers a command-line interface.

//! ## Example
//! ```rust
//! use nyota::tui::banner::get_banner;
//! use nyota::tui::splash::Splash;
//!
//! // Display a banner
//! println!("{}", get_banner());
//!
//! // Display a splash screen
//! SplashScreen::new().show();
//!```
//!
//! ## Modules
//! - `banner`: Banner module for Nyota. This module provides functions that return decorative ASCII art banners and plaques for the Nyota program.
//! - `interactive`: Interactive module for Nyota. This module provides functions for creating interactive interfaces in Nyota.
//! - `menu`: Menu module for Nyota. This module provides functions for creating menus in Nyota.
//! - `splash`: Splash module for Nyota. This module provides functions for displaying splash screens in Nyota.

pub mod banner;
pub mod interactive;
pub mod menu;
pub mod splash;
