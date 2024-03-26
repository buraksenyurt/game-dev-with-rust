use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::factory::*;
use crate::game::Game;
use crate::screen::Screen;

mod constants;
mod entity;
mod factory;
mod game;
mod screen;
mod ui;

fn main() -> Result<(), String> {
    let game = Game::default();
    let screen = Screen::new("Monochrome Bird".to_string(), SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut engine = EngineBuilder::new()?
        .setup_screen(screen)?
        .add_game(game)
        .change_fps(30)
        .build()?;
    engine.run()
}
