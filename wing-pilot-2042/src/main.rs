mod common;
mod entity;
mod game;
mod menu;

use crate::common::constants::{
    BULLET_SPEED_FACTOR, CLOUD_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR,
};
use crate::entity::asset_builder::create_clouds;
use crate::entity::enemy_type::EnemyType;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::game::Game;
use crate::game::state::State;
use crate::menu::builder::draw_info_bar;
use game::conf::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut fighter = Fighter::new().await;
    let mut game = Game::new(State::Playing);
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {}
            State::Playing => {
                if game.clouds.is_empty() {
                    game.clouds = create_clouds(3).await;
                }

                if game.enemy_fleet.enemies.is_empty() && game.enemy_fleet.lift_off_time == 0 {
                    game.enemy_fleet = Fleet::new(3, EnemyType::Fighter).await;
                } else {
                    game.enemy_fleet.lift_off_time -= 1;
                }
                for e in game.enemy_fleet.enemies.iter_mut() {
                    e.location += e.velocity * ENEMY_FIGHTER_SPEED_FACTOR;
                    e.draw();
                }
                shift_fighter(&mut fighter);
                shoot(&mut game, &mut fighter);
                fighter.draw();

                for b in game.bullets.iter_mut() {
                    b.location += Vec2::new(0., -1.) * BULLET_SPEED_FACTOR;
                    b.draw();
                    if b.location.x < 0. {
                        b.is_alive = false;
                    }
                }

                game.bullets.retain(|b| b.is_alive);
                draw_info_bar(&game);

                for c in game.clouds.iter_mut() {
                    c.location += c.velocity * CLOUD_SPEED_FACTOR;
                    if c.location.y - c.texture.height() > screen_height() {
                        c.on_stage = false;
                    }
                    c.draw();
                }
                game.clouds.retain(|c| c.on_stage);
            }
            State::Dead => {}
            State::End => {}
        }
        next_frame().await
    }
}

fn shoot(game: &mut Game, fighter: &mut Fighter) {
    if fighter.ammo_count == 0 {
        //println!("Out of ammo");
        return;
    }
    if is_key_down(KeyCode::S) {
        let bullets = fighter.spawn_bullets();
        match bullets {
            Some(mut b) => {
                game.bullets.append(&mut b);
                fighter.ammo_count -= 2;
                game.fighter_amount_count = fighter.ammo_count;
            }
            None => {}
        }
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
