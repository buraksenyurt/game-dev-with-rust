use crate::entity::{GameState, Shuttle};
use crate::game::Game;

pub trait Command {
    fn execute(&self, shuttle: &mut Shuttle, game: &mut Game, delta_time: f32)
        -> Option<GameState>;
}

pub struct MoveLeftCommand;
pub struct MoveRightCommand;
pub struct MoveUpCommand;
pub struct MoveDownCommand;
pub struct ExitGameCommand;

impl Command for MoveLeftCommand {
    fn execute(
        &self,
        shuttle: &mut Shuttle,
        _game: &mut Game,
        delta_seconds: f32,
    ) -> Option<GameState> {
        shuttle.velocity.x -= 30. * delta_seconds;
        None
    }
}

impl Command for MoveRightCommand {
    fn execute(
        &self,
        shuttle: &mut Shuttle,
        _game: &mut Game,
        delta_seconds: f32,
    ) -> Option<GameState> {
        shuttle.velocity.x += 30. * delta_seconds;
        None
    }
}

impl Command for MoveUpCommand {
    fn execute(
        &self,
        shuttle: &mut Shuttle,
        _game: &mut Game,
        delta_seconds: f32,
    ) -> Option<GameState> {
        shuttle.velocity.y -= 50. * delta_seconds;
        shuttle.fuel_level -= 10;
        None
    }
}

impl Command for MoveDownCommand {
    fn execute(
        &self,
        shuttle: &mut Shuttle,
        _game: &mut Game,
        delta_seconds: f32,
    ) -> Option<GameState> {
        shuttle.velocity.y += 75. * delta_seconds;
        shuttle.fuel_level -= 2;
        None
    }
}

impl Command for ExitGameCommand {
    fn execute(
        &self,
        _shuttle: &mut Shuttle,
        _game: &mut Game,
        _delta_seconds: f32,
    ) -> Option<GameState> {
        Some(GameState::Menu)
    }
}
