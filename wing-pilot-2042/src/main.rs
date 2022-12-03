mod entity;
mod game;

use crate::entity::fighter::Fighter;
use crate::game::game::Game;
use crate::game::state::State;
use game::conf::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let _game = Game::new(State::Main);
    let fighter = Fighter::new().await;
    loop {
        clear_background(DARKBLUE);
        fighter.draw();
        // match game.state {
        //     State::Main => {}
        //     State::Playing => {}
        //     State::Dead => {}
        //     State::End => {}
        // }
        next_frame().await
    }
}
