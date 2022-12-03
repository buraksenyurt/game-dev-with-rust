mod entity;
mod game;

use crate::entity::bullet::Bullet;
use crate::entity::fighter::Fighter;
use crate::entity::owner::Owner;
use crate::game::game::Game;
use crate::game::state::State;
use game::conf::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut game = Game::new(State::Playing);
    let mut fighter = Fighter::new().await;
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {}
            State::Playing => {
                shift_fighter(&mut fighter);
                shoot(&mut game, &mut fighter);
                fighter.draw();

                for b in game.bullets.iter_mut() {
                    b.location += Vec2::new(0., -1.) * 5.;
                    b.draw();
                    if b.location.x < 0. {
                        b.is_alive = false;
                    }
                }

                game.bullets.retain(|b| b.is_alive);
            }
            State::Dead => {}
            State::End => {}
        }
        next_frame().await
    }
}

fn shoot(game: &mut Game, fighter: &mut Fighter) {
    if is_key_pressed(KeyCode::S) {
        let lm = fighter.get_left_muzzle();
        let rm = fighter.get_right_muzzle();
        let bullet_1 = Bullet::spawn(Owner::Fighter, lm);
        let bullet_2 = Bullet::spawn(Owner::Fighter, rm);
        game.bullets.push(bullet_1);
        game.bullets.push(bullet_2);
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
        if is_key_down(KeyCode::Left) {
            fighter.shift_left_up();
        } else if is_key_down(KeyCode::Right) {
            fighter.shift_right_up();
        } else {
            fighter.shift_up();
        }
    } else if is_key_down(KeyCode::Down) {
        if is_key_down(KeyCode::Left) {
            fighter.shift_left_down();
        } else if is_key_down(KeyCode::Right) {
            fighter.shift_right_down();
        } else {
            fighter.shift_down();
        }
    }
}
