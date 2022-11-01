mod bullet;
mod constant;
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
use crate::game_state::GameState;
use crate::garrison::Garrison;
use crate::helper::border_check;
use crate::menu::draw_menu;
use crate::resource::{BULLET_TEXTURE, GARRISON_TEXTURE, TANK_TEXTURE};
use crate::tank::Tank;
use macroquad::prelude::*;
use std::process::exit;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut bullets = Vec::new();
    let mut army = Vec::new();
    let tank_texture: Texture2D = load_texture(TANK_TEXTURE).await.unwrap();
    let bullet_texture: Texture2D = load_texture(BULLET_TEXTURE).await.unwrap();
    let garrison_texture: Texture2D = load_texture(GARRISON_TEXTURE).await.unwrap();
    let mut player = Tank::new(tank_texture);
    let mut last_shot = get_time();

    let mut counter = 0;
    loop {
        if counter == MAX_GARRISON_COUNT {
            break;
        }

        let p = vec2(
            rand::gen_range(
                DEFAULT_MARGIN,
                screen_width() - bullet_texture.width() + DEFAULT_MARGIN,
            ),
            rand::gen_range(
                DEFAULT_MARGIN,
                screen_height() - bullet_texture.height() + DEFAULT_MARGIN,
            ),
        );
        //println!("{}", p);
        let g = Garrison::new(counter, p, garrison_texture);
        if army
            .iter()
            .any(|s: &Garrison| (s.position - g.position).length() < g.texture.width())
        {
            continue;
        }
        army.push(g);
        counter += 1;
    }

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
                    && delta_time - last_shot > BULLET_RELOAD_TIME
                    && bullets.len() < MAX_BULLET_COUNT_IN_RANGE
                {
                    let (r, h) = (player.texture.width() * 0.5, player.texture.height() * 0.5);

                    let x1 = player.position.x + r + (direction.x * r);
                    let y1 = player.position.y + (h * 0.25) + (direction.y * r);

                    let bullet = Bullet {
                        position: vec2(x1, y1),
                        velocity: direction * delta_time as f32,
                        shoot_at: get_time(),
                        collided: false,
                        texture: bullet_texture,
                        rotation,
                    };
                    //println!("Tank\t{}\nBullet\t{}", player_tank, bullet);

                    bullets.push(bullet);
                    last_shot = delta_time;
                }

                for g in army.iter_mut() {
                    let d = g.position;
                    let p = border_check(&d, g.texture.width());
                    g.position = p - direction;
                }

                for s in army.iter_mut() {
                    for b in bullets.iter_mut() {
                        if (b.position - s.position).length() < s.texture.width() * 0.5 {
                            s.collided = true;
                            b.collided = true;
                        }
                    }
                }

                if is_key_down(KeyCode::Right) {
                    player.rotation += TANK_ROTATION_VALUE * get_frame_time();
                } else if is_key_down(KeyCode::Left) {
                    player.rotation -= TANK_ROTATION_VALUE * get_frame_time();
                }

                bullets.retain(|bullet| {
                    bullet.shoot_at + MAX_SHOOT_AT_TIME > delta_time && !bullet.collided
                });
                army.retain(|soldier| !soldier.collided);
                //println!("Total bullets in battlefield {}", bullets.len());
                if army.is_empty() {
                    game_state = GameState::PlayerWin
                }
            }
            _ => {}
        }
        clear_background(BLACK);

        match game_state {
            GameState::Menu => {
                draw_menu();
            }
            GameState::Playing => {
                player.draw();
                for b in bullets.iter_mut() {
                    let rotation = b.rotation;
                    let direction = Vec2::new(rotation.cos(), rotation.sin());
                    b.position += direction * BULLET_SPEED;
                    b.draw();
                }

                for g in army.iter() {
                    g.draw();
                }
            }
            GameState::PlayerDead => {}
            GameState::PlayerWin => {}
        }

        next_frame().await
    }
}
