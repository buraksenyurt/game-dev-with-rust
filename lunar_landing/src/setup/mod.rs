pub mod command;
pub mod config;

pub use command::*;
pub use config::*;

use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::error::Error;
use std::{fs, path::Path};

pub type DirectionCommandsType = HashMap<Keycode, Box<dyn DirectionCommand>>;
pub type MenuCommandsType = HashMap<Keycode, Box<dyn MenuCommand>>;
pub type CommandSetupResultType = Result<(DirectionCommandsType, MenuCommandsType), Box<dyn Error>>;

fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(path)?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}

fn keycode_from_string(key: &str) -> Option<Keycode> {
    match key {
        "Left" => Some(Keycode::Left),
        "A" => Some(Keycode::A),
        "Right" => Some(Keycode::Right),
        "D" => Some(Keycode::D),
        "Down" => Some(Keycode::Down),
        "S" => Some(Keycode::S),
        "Space" => Some(Keycode::Space),
        "W" => Some(Keycode::W),
        "Return" => Some(Keycode::Return),
        "Backspace" => Some(Keycode::Backspace),
        "Escape" => Some(Keycode::Escape),
        _ => None,
    }
}

fn create_direction_command(command_name: &str) -> Option<Box<dyn DirectionCommand>> {
    match command_name {
        "MoveLeftCommand" => Some(Box::new(MoveLeftCommand)),
        "MoveRightCommand" => Some(Box::new(MoveRightCommand)),
        "MoveUpCommand" => Some(Box::new(MoveUpCommand)),
        "MoveDownCommand" => Some(Box::new(MoveDownCommand)),
        _ => None,
    }
}

fn create_menu_command(command_name: &str) -> Option<Box<dyn MenuCommand>> {
    match command_name {
        "StartNewGameCommand" => Some(Box::new(StartNewGameCommand)),
        "ReturnToMenuCommand" => Some(Box::new(ReturnToMenuCommand)),
        "ExitGameCommand" => Some(Box::new(ExitGameCommand)),
        _ => None,
    }
}

pub fn setup_commands() -> CommandSetupResultType {
    let config = load_config("config.toml")?;

    let mut play_commands: HashMap<Keycode, Box<dyn DirectionCommand>> = HashMap::new();
    let mut menu_commands: HashMap<Keycode, Box<dyn MenuCommand>> = HashMap::new();

    for cmd in config.play_commands {
        if let (Some(keycode), Some(command)) = (
            keycode_from_string(&cmd.key),
            create_direction_command(&cmd.command),
        ) {
            play_commands.insert(keycode, command);
        }
    }

    for cmd in config.menu_commands {
        if let (Some(keycode), Some(command)) = (
            keycode_from_string(&cmd.key),
            create_menu_command(&cmd.command),
        ) {
            menu_commands.insert(keycode, command);
        }
    }

    Ok((play_commands, menu_commands))
}
