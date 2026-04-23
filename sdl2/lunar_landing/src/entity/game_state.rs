use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    OutOfFuel,
    MeteorHit,
    JobsDone,
    ExitGame,
    NewGame,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Playing => {
                write!(f, "Playing")
            }
            GameState::OutOfFuel => {
                write!(f, "Out of fuel. Game is over!")
            }
            GameState::MeteorHit => {
                write!(f, "Meteor hit. Game is over!")
            }
            GameState::JobsDone => {
                write!(f, "Job's done... Well done!")
            }
            _ => write!(f, ""),
        }
    }
}
