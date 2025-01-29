use colored::*;

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
    banner.lines()
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
            }.to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn get_version_plaque() -> String {
    let version = env!("CARGO_PKG_VERSION");  // Gets version from Cargo.toml
    let plaque = format!(r#"
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
"#, version);

    plaque.bright_cyan().to_string()
}
