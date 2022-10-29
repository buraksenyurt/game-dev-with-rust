mod bullet;
mod constant;
mod game_state;
mod helper;
mod menu;
mod resource;
mod tank;

use crate::bullet::Bullet;
use crate::constant::TANK_ROTATION_VALUE;
use crate::game_state::GameState;
use crate::helper::border_check;
use crate::menu::draw_menu;
use crate::resource::{BULLET_TEXTURE, TANK_TEXTURE};
use crate::tank::Tank;
use macroquad::prelude::*;
use std::process::exit;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut bullets = Vec::new();
    let tank_texture: Texture2D = load_texture(TANK_TEXTURE).await.unwrap();
    let bullet_texture: Texture2D = load_texture(BULLET_TEXTURE).await.unwrap();
    let mut player_tank = Tank::new(tank_texture);
    let mut last_shot = get_time();

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
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
                let rotation = player_tank.rotation;
                let direction = Vec2::new(rotation.cos(), rotation.sin());
                let position = border_check(&player_tank.position, player_tank.texture.width());
                player_tank.position = position;
                if is_key_down(KeyCode::Up) {
                    //println!("{}", player_tank);
                    player_tank.position += direction;
                } else if is_key_down(KeyCode::Down) {
                    player_tank.position -= direction;
                }

                // 0.5 değeri ile oynayarak bir atıştan ne kadar sonra ateş edebileceğimizi belirtebiliriz.
                if is_key_down(KeyCode::S) && delta_time - last_shot > 0.5 {
                    let bullet = Bullet {
                        position: player_tank.position + rotation,
                        velocity: direction * 5. * delta_time as f32,
                        shoot_at: get_time(),
                        collided: false,
                        texture: bullet_texture,
                        rotation,
                    };
                    bullets.push(bullet);
                    last_shot = delta_time;
                }

                if is_key_down(KeyCode::Right) {
                    player_tank.rotation += TANK_ROTATION_VALUE * get_frame_time();
                } else if is_key_down(KeyCode::Left) {
                    player_tank.rotation -= TANK_ROTATION_VALUE * get_frame_time();
                }

                bullets.retain(|bullet| bullet.shoot_at + 6. > delta_time);
                println!("Total bullets in battlefield {}", bullets.len());
            }
            GameState::PlayerDead => {}
        }
        clear_background(BLACK);

        match game_state {
            GameState::Menu => {
                draw_menu();
            }
            GameState::Playing => {
                player_tank.draw();
                for b in bullets.iter_mut() {
                    let rotation = b.rotation;
                    let direction = Vec2::new(rotation.cos(), rotation.sin());
                    b.position += direction;
                    b.draw();
                }
            }
            GameState::PlayerDead => {}
        }

        next_frame().await
    }
}
