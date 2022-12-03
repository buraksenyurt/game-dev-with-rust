mod entity;
mod game;

use crate::game::game::Game;
use crate::game::state::State;
use game::conf::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let game = Game::new(State::Main);

    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {}
            State::Playing => {}
            State::Dead => {}
            State::End => {}
        }
        next_frame().await
    }
}
