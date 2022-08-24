mod color_factory;
mod constant;
mod game;

use crate::game::Game;
use ggez::graphics::set_window_title;
use ggez::{event, ContextBuilder, GameResult};
use std::{env, path};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut context, event_loop) = ContextBuilder::new("ggez_101", "BurakS")
        .add_resource_path(resource_dir)
        .build()
        .expect("Bir sorun oldu");
    set_window_title(&context, "Temeller");
    let game_state = Game::new(&mut context);
    event::run(context, event_loop, game_state);
}
