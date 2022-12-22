mod common;
mod entity;
mod game;
mod menu;

use crate::common::constants::EXTRA_AMMO_SPEED_FACTOR;
use crate::entity::asset_builder::{create_clouds, create_extra_ammo};
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::entity::fleet::Fleet;
use crate::game::collider::{
    check_enmy_b_coll, check_enmy_f_coll, check_enmy_ws_coll, check_fighter_with_ammo,
};
use crate::game::game::Game;
use crate::game::state::State;
use crate::menu::builder::{draw_dead_menu, draw_end_menu, draw_main_menu};
use game::conf::window_conf;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut game = Game::new(State::Main).await;
    let mut extra_ammo_tick = 0;
    let mut warship_direction = WarshipDirection::Right;
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {
                draw_main_menu();
                if is_key_pressed(KeyCode::Space) {
                    game.state = State::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            State::Playing => {
                if game.fighter.shield == 0 {
                    game.state = State::Dead;
                    continue;
                }
                game.fighter.shift().await;

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
                if game.enemy_warships.actors.is_empty() && game.enemy_warships.bullets.is_empty() {
                    let left_or_right = rand::gen_range(0, 5);
                    warship_direction = match left_or_right % 3 {
                        0 => WarshipDirection::Right,
                        _ => WarshipDirection::Left,
                    };
                    if game.enemy_warships.lift_off_time == 0 {
                        game.enemy_warships =
                            Fleet::new(1, EnemyType::Warship(warship_direction)).await;
                        info!(
                            "Fleet(WS) lift of time {}",
                            game.enemy_warships.lift_off_time
                        );
                    } else {
                        game.enemy_warships.lift_off_time -= 1;
                    }
                }

                if game.fighter.out_of_ammo().await && game.extra_ammo_box == None {
                    let ammo = create_extra_ammo().await;
                    game.extra_ammo_box = Some(ammo);
                    //info!("Extra ammo created");
                }
                shoot(&mut game).await;
                shoot_e(&mut game).await;
                shoot_b(&mut game).await;
                shoot_ws(&mut game).await;

                game.draw_fleet(EnemyType::Warship(WarshipDirection::Right))
                    .await;
                game.draw_fleet(EnemyType::Fighter).await;
                game.draw_fleet(EnemyType::Bomber).await;
                game.draw_fighter_bullets().await;
                game.draw_bullets(EnemyType::Fighter).await;
                game.draw_bullets(EnemyType::Bomber).await;
                game.draw_bullets(EnemyType::Warship(warship_direction))
                    .await;
                game.draw_clouds().await;

                match &game.extra_ammo_box {
                    Some(mut ammo) => {
                        if extra_ammo_tick == ammo.lift_of_time.unwrap() {
                            ammo.location += ammo.velocity * EXTRA_AMMO_SPEED_FACTOR;

                            if ammo.location.y > screen_height() + ammo.texture.height() {
                                game.extra_ammo_box = None;
                                extra_ammo_tick = 0;
                                continue;
                            }

                            game.extra_ammo_box = Some(ammo);
                            ammo.draw();
                        } else {
                            extra_ammo_tick += 1;
                        }
                    }
                    None => {}
                }

                if check_fighter_with_ammo(&mut game).await {
                    game.fighter.ammo_count += 2;
                }
                check_enmy_f_coll(&mut game).await;
                check_enmy_b_coll(&mut game).await;
                check_enmy_ws_coll(&mut game).await;

                game.clouds.retain(|c| c.on_stage);
                game.enemy_fighters.actors.retain(|f| f.on_stage);
                game.enemy_bombers.actors.retain(|b| b.on_stage);
                game.enemy_warships.actors.retain(|b| b.on_stage);
                game.fighter.bullets.retain(|b| b.is_alive);
                game.enemy_fighters.bullets.retain(|f| f.is_alive);
                game.enemy_bombers.bullets.retain(|b| b.is_alive);
                game.enemy_warships.bullets.retain(|b| b.is_alive);

                game.fighter.draw().await;
                game.draw_info_bar().await;
            }
            State::Dead => {
                draw_dead_menu(&game);
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new(State::Playing).await;
                    extra_ammo_tick = 0;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            State::End => {
                draw_end_menu(&game);
                if is_key_pressed(KeyCode::Enter) {
                    //todo: Credits
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await
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
            let bullets = enemy.spawn_bullets(vel).await;
            if let Some(mut b) = bullets {
                game.enemy_bombers.bullets.append(&mut b);
            }
        }
    }
}

async fn shoot_ws(game: &mut Game) {
    for enemy in game.enemy_warships.actors.iter_mut() {
        if enemy.fire_at_will {
            let v = (game.fighter.get_muzzle_point() - enemy.get_muzzle_point()).normalize();
            let angle = 2. * PI - v.angle_between(Vec2::new(1., 0.));
            let vel = Vec2::new(angle.cos(), angle.sin());
            let bullets = enemy.spawn_bullets(vel).await;
            if let Some(mut b) = bullets {
                game.enemy_warships.bullets.append(&mut b);
            }
        }
    }
}
