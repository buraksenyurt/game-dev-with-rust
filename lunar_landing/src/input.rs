use crate::entity::{GameState, Shuttle};
use crate::game::Game;

pub trait DirectionCommand {
    fn execute(&self, shuttle: &mut Shuttle, delta_time: f32);
}

pub trait MenuCommand {
    fn execute(&self) -> Option<GameState>;
}

pub struct MoveLeftCommand;
pub struct MoveRightCommand;
pub struct MoveUpCommand;
pub struct MoveDownCommand;
pub struct ReturnToMenuCommand;
pub struct StartNewGameCommand;
pub struct ExitGameCommand;

impl DirectionCommand for MoveLeftCommand {
    fn execute(&self, shuttle: &mut Shuttle, delta_seconds: f32) {
        shuttle.velocity.x -= 30. * delta_seconds;
    }
}

impl DirectionCommand for MoveRightCommand {
    fn execute(&self, shuttle: &mut Shuttle, delta_seconds: f32) {
        shuttle.velocity.x += 30. * delta_seconds;
    }
}

impl DirectionCommand for MoveUpCommand {
    fn execute(&self, shuttle: &mut Shuttle, delta_seconds: f32) {
        shuttle.velocity.y -= 50. * delta_seconds;
        shuttle.fuel_level -= 10;
    }
}

impl DirectionCommand for MoveDownCommand {
    fn execute(&self, shuttle: &mut Shuttle, delta_seconds: f32) {
        shuttle.velocity.y += 75. * delta_seconds;
        shuttle.fuel_level -= 2;
    }
}

impl MenuCommand for ReturnToMenuCommand {
    fn execute(&self) -> Option<GameState> {
        Some(GameState::Menu)
    }
}

impl MenuCommand for StartNewGameCommand {
    fn execute(&self) -> Option<GameState> {
        Some(GameState::NewGame)
    }
}

impl MenuCommand for ExitGameCommand {
    fn execute(&self) -> Option<GameState> {
        Some(GameState::ExitGame)
    }
}
