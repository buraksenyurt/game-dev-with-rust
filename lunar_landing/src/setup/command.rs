use crate::entity::GameState;
use crate::game::Game;

pub trait DirectionCommand {
    fn execute(&self, game: &mut Game);
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
    fn execute(&self, game: &mut Game) {
        game.shuttle.velocity.x -= 30. * game.delta_second.as_secs_f32();
    }
}

impl DirectionCommand for MoveRightCommand {
    fn execute(&self, game: &mut Game) {
        game.shuttle.velocity.x += 30. * game.delta_second.as_secs_f32();
    }
}

impl DirectionCommand for MoveUpCommand {
    fn execute(&self, game: &mut Game) {
        game.shuttle.velocity.y -= 50. * game.delta_second.as_secs_f32();
        game.shuttle.fuel_level -= 10;
    }
}

impl DirectionCommand for MoveDownCommand {
    fn execute(&self, game: &mut Game) {
        game.shuttle.velocity.y += 75. * game.delta_second.as_secs_f32();
        game.shuttle.fuel_level -= 2;
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
