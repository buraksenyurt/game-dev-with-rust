mod common;
mod entity;
mod game;
mod menu;

use crate::common::constants::{EXTRA_AMMO_SPEED_FACTOR, MAX_AMMO};
use crate::entity::asset_builder::{create_assets, create_extra_ammo};
use crate::entity::asset_type::AssetType;
use crate::entity::enemy_type::EnemyType;
use crate::game::collider::{
    check_fighter_with_ammo, fighter_vs_bomber, fighter_vs_fighter, fighter_vs_warship,
    fighter_vs_warship_missile, warship_vs_ground,
};
use crate::game::game::Game;
use crate::game::state::State;
use crate::menu::builder::{draw_dead_menu, draw_main_menu};
use game::conf::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);
    let mut game = Game::new(State::Main).await;
    let mut extra_ammo_tick = 0;
    loop {
        clear_background(DARKBLUE);

        match game.state {
            State::Main => {
                draw_main_menu();
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new(State::Playing).await;
                    extra_ammo_tick = 0;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            State::Playing => {
                if game.fighter.is_got_shot {
                    game.fighter.draw_on_shot().await;
                    game.fighter.is_got_shot = false;
                }
                if game.fighter.shield <= 0 {
                    game.state = State::Dead;
                    continue;
                }
                game.fighter.shift().await;

                if game.grounds.is_empty() {
                    game.grounds = create_assets(1, AssetType::Ground).await;
                }
                if game.clouds.is_empty() {
                    game.clouds = create_assets(3, AssetType::Cloud).await;
                }

                game.spawn_enemy_fighters().await;
                game.spawn_enemy_bombers().await;
                game.spawn_enemy_warships().await;

                if game.fighter.out_of_ammo().await && game.extra_ammo_box == None {
                    let ammo = create_extra_ammo().await;
                    game.extra_ammo_box = Some(ammo);
                    //info!("Extra ammo created");
                }
                game.fighter.shoot().await;
                game.enemy_shot().await;
                game.bomber_shot().await;
                game.warship_shot().await;
                if rand::gen_range(0, 25) == 0 {
                    game.recalc_distance().await;
                }

                game.draw_grounds().await;
                game.draw_fleet(EnemyType::Warship(None)).await;
                game.draw_fleet(EnemyType::Fighter).await;
                game.draw_fleet(EnemyType::Bomber).await;
                game.draw_fighter_bullets().await;
                game.draw_bullets(EnemyType::Fighter).await;
                game.draw_bullets(EnemyType::Bomber).await;
                game.draw_bullets(EnemyType::Warship(None)).await;
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

                if check_fighter_with_ammo(&mut game).await && game.fighter.ammo_count <= MAX_AMMO {
                    game.fighter.ammo_count += 2;
                }
                fighter_vs_fighter(&mut game).await;
                fighter_vs_bomber(&mut game).await;
                fighter_vs_warship(&mut game).await;
                fighter_vs_warship_missile(&mut game).await;
                warship_vs_ground(&mut game).await;

                game.clouds.retain(|c| c.on_stage);
                game.enemy_fighters.actors.retain(|f| f.on_stage);
                game.enemy_bombers.actors.retain(|b| b.on_stage);
                game.enemy_warships.actors.retain(|ws| ws.on_stage);
                game.fighter.bullets.retain(|b| b.is_alive);
                game.enemy_fighters.bullets.retain(|f| f.is_alive);
                game.enemy_bombers.bullets.retain(|b| b.is_alive);
                game.enemy_warships.bullets.retain(|ws| ws.is_alive);

                game.fighter.draw().await;
                game.draw_info_bar().await;
            }
            State::Dead => {
                draw_dead_menu(&game);
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new(State::Playing).await;
                    extra_ammo_tick = 0;
                } else if is_key_pressed(KeyCode::Enter) {
                    game.state = State::Main;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await
    }
}
