use crate::factory::screen::Screen;
use crate::factory::EngineBuilder;
use crate::game::Game;

mod entity;
mod factory;
mod game;
mod tests;
mod ui;
mod utility;

fn main() -> Result<(), String> {
    let game = Game::default();
    let screen = Screen::new("Mathventure with Burak".to_string(), 640, 480);
    let mut engine = EngineBuilder::new()?
        .setup_screen(screen)?
        .add_game(Box::new(game))
        .change_fps(60)
        .build()?;
    engine.run()
}
