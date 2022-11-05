mod bullet;
mod constant;
mod game;
mod game_state;
mod garrison;
mod helper;
mod menu;
mod resource;
mod tank;

use crate::bullet::Bullet;
use crate::constant::{
    BULLET_RELOAD_TIME, BULLET_SPEED, DEFAULT_MARGIN, MAX_BULLET_COUNT_IN_RANGE,
    MAX_GARRISON_COUNT, MAX_SHOOT_AT_TIME, TANK_ROTATION_VALUE,
};
use crate::game::Game;
use crate::game_state::GameState;
use crate::garrison::Garrison;
use crate::helper::border_check;
use crate::menu::draw_menu;
use crate::resource::{get_texture, TextureType};
use crate::tank::Tank;
use macroquad::prelude::*;
use std::process::exit;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let mut game = Game::init().await;

    loop {
        match game.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    exit(0);
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    exit(0);
                }

                let delta_time = get_time();
                let mut player = &mut game.player;
                let rotation = player.rotation;
                let direction = Vec2::new(rotation.cos(), rotation.sin());
                let position = border_check(&player.position, player.texture.width());
                player.position = position;
                if is_key_down(KeyCode::Up) {
                    player.position += direction;
                } else if is_key_down(KeyCode::Down) {
                    player.position -= direction;
                }

                // BULLET_RELOAD_TIME değeri ile oynayarak
                // bir atıştan ne kadar sonra ateş edebileceğimizi belirtebiliriz.
                if is_key_down(KeyCode::S)
                    && delta_time - game.last_shot > BULLET_RELOAD_TIME
                    && game.bullets.len() < MAX_BULLET_COUNT_IN_RANGE
                {
                    let (r, h) = (player.texture.width() * 0.5, player.texture.height() * 0.5);

                    let x1 = player.position.x + r + (direction.x * r);
                    let y1 = player.position.y + (h * 0.25) + (direction.y * r);

                    let bullet = Bullet {
                        position: vec2(x1, y1),
                        velocity: direction * delta_time as f32,
                        shoot_at: get_time(),
                        collided: false,
                        texture: get_texture(TextureType::Bullet).await,
                        rotation,
                    };
                    //println!("Tank\t{}\nBullet\t{}", player_tank, bullet);

                    game.bullets.push(bullet);
                    game.last_shot = delta_time;
                }

                for g in game.army.iter_mut() {
                    let d = g.position;
                    let p = border_check(&d, g.texture.width());
                    g.position = p - direction;
                }

                for s in game.army.iter_mut() {
                    for b in game.bullets.iter_mut() {
                        if (b.position - s.position).length() < s.texture.width() * 0.5 {
                            s.collided = true;
                            b.collided = true;
                            game.score += 10;
                        }
                    }
                }

                for s in game.army.iter_mut() {
                    if (player.position - s.position).length() < player.texture.width() * 0.75 {
                        game.state = GameState::PlayerDead;
                    }
                }

                if is_key_down(KeyCode::Right) {
                    player.rotation += TANK_ROTATION_VALUE * get_frame_time();
                } else if is_key_down(KeyCode::Left) {
                    player.rotation -= TANK_ROTATION_VALUE * get_frame_time();
                }

                game.bullets.retain(|bullet| {
                    bullet.shoot_at + MAX_SHOOT_AT_TIME > delta_time && !bullet.collided
                });
                game.army.retain(|soldier| !soldier.collided);
                //println!("Total bullets in battlefield {}", bullets.len());
                if game.army.is_empty() {
                    game.state = GameState::PlayerWin
                }
            }
            GameState::PlayerWin | GameState::PlayerDead => {
                if is_key_pressed(KeyCode::Space) {
                    game = Game::init().await;
                    game.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    exit(0);
                }
            }
        }
        clear_background(BLACK);

        match game.state {
            GameState::Menu => {
                let main_menu = vec!["Press Space to Start", "Press ESC to Exit"];
                draw_menu(main_menu);
            }
            GameState::Playing => {
                game.player.draw();
                for b in game.bullets.iter_mut() {
                    let rotation = b.rotation;
                    let direction = Vec2::new(rotation.cos(), rotation.sin());
                    b.position += direction * BULLET_SPEED;
                    b.draw();
                }

                for g in game.army.iter() {
                    g.draw();
                }
            }
            GameState::PlayerDead => {
                let info = format!("Game Over!{}", game.score);
                let end_menu = vec![info.as_str(), "Press SPACE to replay or ESC to Exit"];
                draw_menu(end_menu);
            }
            GameState::PlayerWin => {
                let info = format!("You win! Your score is {}", game.score);
                let win_menu = vec![info.as_str(), "Press SPACE to replay or ESC to Exit"];
                draw_menu(win_menu);
            }
        }
        next_frame().await
    }
}
