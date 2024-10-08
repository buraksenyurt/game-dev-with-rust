use crate::factory::screen::Screen;
use crate::factory::{Dimension, EngineBuilder};
use crate::game::Game;
use crate::resources::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod entity;
mod factory;
mod game;
mod resources;
mod tests;
mod ui;

fn main() -> Result<(), String> {
    let game = Game::default();
    let screen = Screen::new(
        "Mathventure with Burak".to_string(),
        Dimension::new(SCREEN_WIDTH, SCREEN_HEIGHT),
    );
    let mut engine = EngineBuilder::new()?
        .setup_screen(screen)?
        .add_game(Box::new(game))
        .change_fps(60)
        .build()?;
    engine.run()
}
