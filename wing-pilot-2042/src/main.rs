mod common;
mod entity;
mod game;
mod menu;

use crate::common::constants::{
    CLOUD_SPEED_FACTOR, ENEMY_BOMBER_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR,
    EXTRA_AMMO_SPEED_FACTOR, FIGHTER_BULLET_SPEED_FACTOR,
};
use crate::entity::asset_builder::{create_clouds, create_extra_ammo};
use crate::entity::enemy::Enemy;
use crate::entity::enemy_type::EnemyType;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::game::Game;
use crate::game::state::State;
use crate::menu::builder::draw_info_bar;
use game::conf::window_conf;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut game = Game::new(State::Playing).await;
    let mut extra_ammo_tick = 0;
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {}
            State::Playing => {
                if game.clouds.is_empty() {
                    game.clouds = create_clouds(3).await;
                }
                if game.enemy_fighters.actors.is_empty() && game.enemy_fighters.bullets.is_empty() {
                    if game.enemy_fighters.lift_off_time == 0 {
                        game.enemy_fighters = Fleet::new(4, EnemyType::Fighter).await;
                        info!(
                            "Fleet(F) lift of time {}",
                            game.enemy_fighters.lift_off_time
                        );
                    } else {
                        game.enemy_fighters.lift_off_time -= 1;
                    }
                }

                if game.enemy_bombers.actors.is_empty() && game.enemy_bombers.bullets.is_empty() {
                    if game.enemy_bombers.lift_off_time == 0 {
                        game.enemy_bombers = Fleet::new(3, EnemyType::Bomber).await;
                        info!("Fleet(B) lift of time {}", game.enemy_bombers.lift_off_time);
                    } else {
                        game.enemy_bombers.lift_off_time -= 1;
                    }
                }

                if game.fighter.out_of_ammo().await && game.extra_ammo == None {
                    let ammo = create_extra_ammo().await;
                    game.extra_ammo = Some(ammo);
                    //info!("Extra ammo created");
                }
                draw_fighter_fleet(&mut game).await;
                draw_bomber_fleet(&mut game).await;
                shift_fighter(&mut game.fighter).await;
                shoot(&mut game).await;
                shoot_e(&mut game).await;
                shoot_b(&mut game).await;
                draw_fighter_bullets(&mut game).await;
                draw_enemy_fighter_bullets(&mut game).await;
                draw_enemy_bomber_bullets(&mut game).await;
                draw_clouds(&mut game).await;

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
                game.enemy_fighters.actors.retain(|f| f.on_stage);
                game.enemy_bombers.actors.retain(|b| b.on_stage);
                game.fighter.bullets.retain(|b| b.is_alive);
                game.enemy_fighters.bullets.retain(|f| f.is_alive);
                game.enemy_bombers.bullets.retain(|b| b.is_alive);

                game.fighter.draw().await;
                draw_info_bar(&game).await;
            }
            State::Dead => {}
            State::End => {}
        }
        next_frame().await
    }
}

async fn draw_fighter_fleet(game: &mut Game) {
    for e in game.enemy_fighters.actors.iter_mut() {
        e.position += e.velocity * ENEMY_FIGHTER_SPEED_FACTOR;
        if !e.is_formation_on && e.position.y >= e.formation.start_y {
            e.velocity = e.formation.velocity;
            e.is_formation_on = true;
            e.fire_at_will = true;
            //println!("Formation changed");
        }

        check_borders(e).await;
        e.draw();
    }
}

async fn draw_bomber_fleet(game: &mut Game) {
    for e in game.enemy_bombers.actors.iter_mut() {
        e.position += e.velocity * ENEMY_BOMBER_SPEED_FACTOR;
        if !e.is_formation_on && e.position.y >= e.formation.start_y {
            e.velocity = e.formation.velocity;
            e.is_formation_on = true;
            e.fire_at_will = true;
            //println!("Formation changed");
        }

        check_borders(e).await;
        e.draw();
    }
}

async fn draw_fighter_bullets(game: &mut Game) {
    for b in game.fighter.bullets.iter_mut() {
        b.location += Vec2::new(0., -1.) * FIGHTER_BULLET_SPEED_FACTOR;
        b.draw().await;
        if b.location.x < 0. {
            b.is_alive = false;
        }
    }
}

async fn draw_enemy_fighter_bullets(game: &mut Game) {
    for b in game.enemy_fighters.bullets.iter_mut() {
        b.location += b.velocity * ENEMY_FIGHTER_SPEED_FACTOR;
        b.draw().await;
        if b.location.y > screen_height() {
            b.is_alive = false;
        }
    }
}

async fn draw_enemy_bomber_bullets(game: &mut Game) {
    for b in game.enemy_bombers.bullets.iter_mut() {
        b.location += b.velocity * ENEMY_BOMBER_SPEED_FACTOR;
        b.draw().await;
        if b.location.y > screen_height() {
            b.is_alive = false;
        }
    }
}

async fn draw_clouds(game: &mut Game) {
    for c in game.clouds.iter_mut() {
        c.location += c.velocity * CLOUD_SPEED_FACTOR;
        if c.location.y - c.texture.height() > screen_height() {
            c.on_stage = false;
        }
        c.draw();
    }
}

async fn check_borders(e: &mut Enemy) {
    if (e.velocity.y < 0. && e.position.y + e.texture.height() < 0.)
        || (e.velocity.x < 0. && e.position.x + e.texture.width() < 0.)
        || (e.position.x > screen_width() + e.texture.width()
            || e.position.y > screen_height() + e.texture.height())
    {
        e.on_stage = false;
    }
}

async fn shoot(game: &mut Game) {
    if game.fighter.ammo_count == 0 {
        //println!("Out of ammo");
        return;
    }
    if is_key_down(KeyCode::S) {
        let bullets = game.fighter.spawn_bullets().await;
        if let Some(mut b) = bullets {
            game.fighter.bullets.append(&mut b);
            game.fighter.ammo_count -= 2;
        }
    }
}

async fn shoot_e(game: &mut Game) {
    for enemy in game.enemy_fighters.actors.iter_mut() {
        if enemy.fire_at_will {
            let bullets = enemy.spawn_bullets(Vec2::new(0., 1.)).await;
            if let Some(mut b) = bullets {
                game.enemy_fighters.bullets.append(&mut b);
            }
        }
    }
}

async fn shoot_b(game: &mut Game) {
    for enemy in game.enemy_bombers.actors.iter_mut() {
        if enemy.fire_at_will {
            let v = (game.fighter.get_muzzle_point() - enemy.get_muzzle_point()).normalize();
            let angle = 2. * PI - v.angle_between(Vec2::new(1., 0.));
            let vel = Vec2::new(angle.cos(), angle.sin());
            // println!(
            //     "Bomber Vector {}, Fighter Vector {}, Distance Vector {}, Angle {} {}, Calculated Velocity {}",
            //     enemy.position,game.fighter.position,v, angle.to_degrees(),angle, vel
            // );
            let bullets = enemy.spawn_bullets(vel).await;
            if let Some(mut b) = bullets {
                game.enemy_bombers.bullets.append(&mut b);
            }
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
