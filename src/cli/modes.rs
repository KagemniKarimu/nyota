use clap::{Arg, ArgAction, ArgGroup, Command};

/// The different modes that nyota can be started in.
/// This is used to determine the behavior of the program.
/// The mode is set by the user using command-line flags.
/// If no flags are provided, the default mode is `Menu`.
#[derive(Debug)]
pub enum Mode {
    /// Interactive mode begins the program with a Read-Eval-Print Loop (REPL).
    Interactive,
    /// Development mode is like interactive mode, but with raw outputs for debugging & development.
    Development,
    /// Task mode gives users the opportunity to execute a single task and quit upon completion.
    Task,
    /// Menu mode gives users access to nyota from the main menu. It is the default mode.
    Menu, // default mode
}

/// The settings for the current mode of nyota.
pub struct ModeSettings {
    pub mode: Mode,
}

/// Parses the user input flags to determine the mode to start nyota in. If no flags are provided, the default mode is `Menu`.
pub fn get_mode_input() -> ModeSettings {
    // Parse User Input Flags
    let flag_input = Command::new("nyota")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .help("Start in interactive REPL mode"),
        )
        .arg(
            Arg::new("development")
                .short('d')
                .long("dev")
                .action(ArgAction::SetTrue)
                .help("Start in development mode with raw outputs"),
        )
        .arg(
            Arg::new("task")
                .short('t')
                .long("task")
                .action(ArgAction::SetTrue)
                .help("Execute a single task"),
        )
        .group(
            ArgGroup::new("modes")
                .args(["interactive", "development", "task"])
                .required(false) // set to false because we need to send to menu
                .multiple(false),
        )
        .get_matches();

    // Debug Print of Flag Input
    // println!("{:?}",flag_input);

    // Set and Return Mode Settings
    ModeSettings {
        mode: if flag_input.get_flag("interactive") {
            Mode::Interactive
        } else if flag_input.get_flag("development") {
            Mode::Development
        } else if flag_input.get_flag("task") {
            Mode::Task
        } else {
            //if no Flag detected, default to Menu
            Mode::Menu
        },
    }
}
