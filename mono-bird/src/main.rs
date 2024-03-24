use crate::engine::Engine;
use crate::game::Game;

mod constants;
mod engine;
mod game;
mod ui;

fn main() -> Result<(), String> {
    let game = Game::default();
    let mut engine = Engine::new(game, 60)?;
    engine.run()
}
