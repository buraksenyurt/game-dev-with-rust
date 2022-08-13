mod main_state;

use crate::main_state::MainState;
use ggez;
use ggez::graphics::set_window_title;
use ggez::{event, graphics, Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("Ping_Pong_0", "BurakS")
        .build()
        .expect("There is a problem for creating context");
    set_window_title(&context, "PING_PONG");
    let mut state = MainState::new(&mut context);
    event::run(context, event_loop, state);
}
