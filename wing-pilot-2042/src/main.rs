mod game;

use macroquad::prelude::*;
use game::conf::window_conf;
use crate::game::game::Game;
use crate::game::state::State;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let game=Game::new(State::Main);
    loop {
        match game.state{
            State::Main=>{

            }
            State::Playing=>{

            }
            State::Dead=>{

            }
            State::End=>{

            }
        }
        next_frame().await
    }
}
