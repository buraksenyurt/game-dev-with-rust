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
    let mut fighter = Fighter::new().await;
    println!("{}", fighter.texture.width());
    loop {
        clear_background(DARKBLUE);
        shift_fighter(&mut fighter);

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

fn shift_fighter(fighter: &mut Fighter) {
    if is_key_down(KeyCode::Left) {
        if is_key_down(KeyCode::Up) {
            fighter.shift_left_up();
        } else if is_key_down(KeyCode::Down) {
            fighter.shift_left_down();
        } else {
            fighter.shift_left();
        }
    } else if is_key_down(KeyCode::Right) {
        if is_key_down(KeyCode::Up) {
            fighter.shift_right_up();
        } else if is_key_down(KeyCode::Down) {
            fighter.shift_right_down();
        } else {
            fighter.shift_right();
        }
    } else if is_key_down(KeyCode::Up) {
        fighter.shift_up();
    } else if is_key_down(KeyCode::Down) {
        fighter.shift_down();
    }
}
