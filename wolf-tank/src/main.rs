mod constant;
mod game_state;
mod helper;
mod menu;
mod resource;
mod tank;

use crate::constant::TANK_ROTATION_VALUE;
use crate::game_state::GameState;
use crate::helper::border_check;
use crate::menu::draw_menu;
use crate::resource::TANK_TEXTURE;
use crate::tank::Tank;
use macroquad::prelude::*;
use std::process::exit;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let mut game_state = GameState::Menu;
    let tank_texture: Texture2D = load_texture(TANK_TEXTURE).await.unwrap();
    let mut player_tank = Tank::new(tank_texture);
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

                let delta_time = get_frame_time();
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

                if is_key_down(KeyCode::Right) {
                    player_tank.rotation += TANK_ROTATION_VALUE * delta_time;
                } else if is_key_down(KeyCode::Left) {
                    player_tank.rotation -= TANK_ROTATION_VALUE * delta_time;
                }
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
            }
            GameState::PlayerDead => {}
        }

        next_frame().await
    }
}
