mod common;
mod entity;
mod game;
mod menu;

use crate::common::constants::{
    BULLET_SPEED_FACTOR, CLOUD_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR, EXTRA_AMMO_SPEED_FACTOR,
};
use crate::entity::asset::Asset;
use crate::entity::asset_builder::{create_clouds, create_extra_ammo};
use crate::entity::enemy::Enemy;
use crate::entity::enemy_type::EnemyType;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::game::Game;
use crate::game::state::State;
use crate::menu::builder::draw_info_bar;
use game::conf::window_conf;
use macroquad::experimental::collections::storage::get;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut fighter = Fighter::new().await;
    let mut game = Game::new(State::Playing);
    let mut extra_ammo_tick = 0;
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {}
            State::Playing => {
                if game.clouds.is_empty() {
                    game.clouds = create_clouds(3).await;
                }
                if game.enemy_fleet.enemies.is_empty() {
                    if game.enemy_fleet.lift_off_time == 0 {
                        game.enemy_fleet = Fleet::new(4, EnemyType::Fighter).await;
                        info!("Fleet lift of time {}", game.enemy_fleet.lift_off_time);
                    } else {
                        game.enemy_fleet.lift_off_time -= 1;
                    }
                }
                if fighter.out_of_ammo().await && game.extra_ammo == None {
                    let ammo = create_extra_ammo().await;
                    game.extra_ammo = Some(ammo);
                    //info!("Extra ammo created");
                }
                draw_fleet(&mut game).await;
                shift_fighter(&mut fighter).await;
                shoot(&mut game, &mut fighter).await;
                draw_fighter_bullets(&mut game).await;
                draw_clouds(&mut game);

                match &game.extra_ammo {
                    Some(mut ammo) => {
                        if extra_ammo_tick == ammo.lift_of_time.unwrap() {
                            ammo.location += ammo.velocity * EXTRA_AMMO_SPEED_FACTOR;

                            if ammo.location.y > screen_height() + ammo.texture.height() {
                                game.extra_ammo = None;
                                extra_ammo_tick = 0;
                                continue;
                            }

                            game.extra_ammo = Some(ammo);
                            ammo.draw();
                        } else {
                            extra_ammo_tick += 1;
                        }
                    }
                    None => {}
                }

                game.clouds.retain(|c| c.on_stage);
                game.enemy_fleet.enemies.retain(|e| e.on_stage);
                game.bullets.retain(|b| b.is_alive);

                fighter.draw().await;
                draw_info_bar(&game).await;
            }
            State::Dead => {}
            State::End => {}
        }
        next_frame().await
    }
}

async fn draw_fleet(game: &mut Game) {
    for e in game.enemy_fleet.enemies.iter_mut() {
        e.location += e.velocity * ENEMY_FIGHTER_SPEED_FACTOR;
        if !e.is_formation_on && e.location.y >= e.formation.start_y {
            e.velocity = e.formation.velocity;
            e.is_formation_on = true;
            //println!("Formation changed");
        }

        check_borders(e).await;
        e.draw();
    }
}

async fn draw_fighter_bullets(game: &mut Game) {
    for b in game.bullets.iter_mut() {
        b.location += Vec2::new(0., -1.) * BULLET_SPEED_FACTOR;
        b.draw().await;
        if b.location.x < 0. {
            b.is_alive = false;
        }
    }
}

fn draw_clouds(game: &mut Game) {
    for c in game.clouds.iter_mut() {
        c.location += c.velocity * CLOUD_SPEED_FACTOR;
        if c.location.y - c.texture.height() > screen_height() {
            c.on_stage = false;
        }
        c.draw();
    }
}

async fn check_borders(e: &mut Enemy) {
    if e.velocity.y < 0. && e.location.y + e.texture.height() < 0. {
        e.on_stage = false;
    } else if e.velocity.x < 0. && e.location.x + e.texture.width() < 0. {
        e.on_stage = false;
    } else {
        if e.location.x > screen_width() + e.texture.width()
            || e.location.y > screen_height() + e.texture.height()
        {
            e.on_stage = false;
        }
    }
}

async fn shoot(game: &mut Game, fighter: &mut Fighter) {
    if fighter.ammo_count == 0 {
        //println!("Out of ammo");
        return;
    }
    if is_key_down(KeyCode::S) {
        let bullets = fighter.spawn_bullets().await;
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

async fn shift_fighter(fighter: &mut Fighter) {
    if is_key_down(KeyCode::Left) {
        if is_key_down(KeyCode::Up) {
            fighter.shift_left_up().await;
        } else if is_key_down(KeyCode::Down) {
            fighter.shift_left_down().await;
        } else {
            fighter.shift_left().await;
        }
    } else if is_key_down(KeyCode::Right) {
        if is_key_down(KeyCode::Up) {
            fighter.shift_right_up().await;
        } else if is_key_down(KeyCode::Down) {
            fighter.shift_right_down().await;
        } else {
            fighter.shift_right().await;
        }
    } else if is_key_down(KeyCode::Up) {
        if is_key_down(KeyCode::Left) {
            fighter.shift_left_up().await;
        } else if is_key_down(KeyCode::Right) {
            fighter.shift_right_up().await;
        } else {
            fighter.shift_up().await;
        }
    } else if is_key_down(KeyCode::Down) {
        if is_key_down(KeyCode::Left) {
            fighter.shift_left_down().await;
        } else if is_key_down(KeyCode::Right) {
            fighter.shift_right_down().await;
        } else {
            fighter.shift_down().await;
        }
    }
}
