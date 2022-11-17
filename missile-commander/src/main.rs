mod lib;

use crate::lib::game::Game;
use crate::lib::{create_buildings, create_missiles, draw_buildings, draw_cursor, window_conf};
use lib::building::*;
use lib::constant::*;
use macroquad::audio;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);

    let hit_sound = audio::load_sound("resource/cannon_hit.ogg")
        .await
        .unwrap();
    let mut game = Game::new();
    let buildings = create_buildings();
    let mut missiles = create_missiles(MAX_MISSILE_COUNT);
    clear_background(Color::default());

    loop {
        draw_buildings(&buildings);
        draw_cursor();
        game.draw();

        if game.city_health == 0 {
            println!("Commander! City has fatal damage.");
            break;
        }
        for m in missiles.iter_mut() {
            if m.lift_off_time == 0 {
                m.position += m.direction * MISSILE_SPEED_FACTOR;
                m.draw();

                if m.position.x < 0. || m.position.x > screen_width() {
                    m.is_alive = false;
                }
                if m.position.y > screen_height() - 100. {
                    m.is_alive = false;
                    game.city_health -= 100;
                    audio::play_sound_once(hit_sound);
                }
            } else {
                m.lift_off_time -= 1;
            }
        }

        missiles.retain(|m| m.is_alive);
        let mut new_missiles = create_missiles(MAX_MISSILE_COUNT - missiles.len() as i32);
        missiles.append(&mut new_missiles);
        next_frame().await
    }
}
