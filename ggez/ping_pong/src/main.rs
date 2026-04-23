mod ball;
mod constants;
mod main_state;
mod player;
mod racket;

use crate::main_state::MainState;
use ggez;
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

    let (mut context, event_loop) = ContextBuilder::new("Ping_Pong_0", "BurakS")
        .add_resource_path(resource_dir)
        .build()
        .expect("There is a problem for creating context");
    set_window_title(&context, "PING_PONG");
    let state = MainState::new(&mut context);
    event::run(context, event_loop, state);
}
