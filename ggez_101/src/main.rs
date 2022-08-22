mod game;

use crate::game::Game;
use ggez::graphics::set_window_title;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("ggez_101", "BurakS")
        .build()
        .expect("Bir sorun oldu");
    set_window_title(&context, "Temeller");
    let game_state = Game::new(&mut context);
    event::run(context, event_loop, game_state);
}
