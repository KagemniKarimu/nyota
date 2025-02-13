//! This module contains functions that return decorative ASCII art banners and plaques for the nyota program.

use colored::*;

/// Returns the banner for nyota. A banner is a decorative image or sign used to display information.
/// The banner displays the name of the program in a decorative ASCII art format.
/// The ASCII art is colored using the `colored` crate to provide a rainbow effect.
pub fn get_banner() -> String {
    let banner = r#"
                                 █████
                                ░░███
 ████████   █████ ████  ██████  ███████    ██████
░░███░░███ ░░███ ░███  ███░░███░░░███░    ░░░░░███
 ░███ ░███  ░███ ░███ ░███ ░███  ░███      ███████
 ░███ ░███  ░███ ░███ ░███ ░███  ░███ ███ ███░░███
 ████ █████ ░░███████ ░░██████   ░░█████ ░░████████
░░░░ ░░░░░   ░░░░░███  ░░░░░░     ░░░░░   ░░░░░░░░
             ███ ░███
            ░░██████
             ░░░░░░
"#;

    // Create rainbow effect
    banner
        .lines()
        .enumerate()
        .map(|(i, line)| {
            match i % 6 {
                0 => line.bright_red(),
                1 => line.bright_yellow(),
                2 => line.bright_green(),
                3 => line.bright_cyan(),
                4 => line.bright_blue(),
                5 => line.bright_magenta(),
                _ => line.normal(),
            }
            .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Returns the version plaque for nyota. A plaque is a decorative image or sign used to display information.
/// The version plaque displays the version number and authors of the program. The version number is read from the Cargo.toml file.
pub fn get_version_plaque() -> String {
    let version = env!("CARGO_PKG_VERSION"); // Gets version from Cargo.toml
    let plaque = format!(
        r#"
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
    ┌╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶┐
    ╎                                 ╎
    ╎    version:  v{}             ╎
    ╎                                 ╎
    ╎    authors: DariaAG             ╎
    ╎             KagemniKarimu       ╎
    ╎                                 ╎
    ╎                                 ╎
    └╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶┘
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
"#,
        version
    );

    plaque.bright_cyan().to_string()
}
