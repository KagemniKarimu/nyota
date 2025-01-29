use clap::{Arg, ArgAction, ArgGroup, Command};

#[derive(Debug)]
pub enum Mode {
    Interactive,
    Development,
    Task,
    Menu, // default mode
}

pub struct ModeSettings {
    pub mode: Mode,
}

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
