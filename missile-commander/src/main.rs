mod lib;

use crate::lib::bullet::Bullet;
use crate::lib::game::Game;
use crate::lib::turret::Turret;
use crate::lib::{create_buildings, create_missiles, draw_buildings, draw_cursor, window_conf};
use lib::building::*;
use lib::constant::*;
use macroquad::audio;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);

    let hit_sound = audio::load_sound("resource/cannon_hit.ogg").await.unwrap();
    let mut game = Game::new();
    let buildings = create_buildings();
    let mut missiles = create_missiles(MAX_MISSILE_COUNT);
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut mini_gunner = Turret::new();
    clear_background(Color::default());

    loop {
        draw_buildings(&buildings);
        draw_cursor();
        mini_gunner.draw();
        game.draw();

        if is_mouse_button_pressed(MouseButton::Left)
            && bullets.len() < MAX_BULLET_ON_GAME
            && mini_gunner.muzzle_point.y < screen_height()-CITY_HEIGHT
        {
            let bullet = Bullet::spawn(mini_gunner.muzzle_point);
            bullets.push(bullet);
        }
        if game.city_health == 0 {
            println!("Commander! City has fatal damage.");
            break;
        }

        for b in bullets.iter_mut() {
            if b.location.x < 0. || b.location.x > screen_width() || b.location.y < 0. {
                b.is_alive = false;
            }
            b.location += b.velocity * BULLET_SPEED_FACTOR;
            b.draw();
        }

        bullets.retain(|b| b.is_alive);

        for m in missiles.iter_mut() {
            if m.lift_off_time == 0 {
                m.position += m.direction * MISSILE_SPEED_FACTOR;
                m.draw();

                // if m.position.x < 0. || m.position.x > screen_width() {
                //     m.is_alive = false;
                // }
                if m.position.y > screen_height() - CITY_HEIGHT {
                    m.is_alive = false;
                    game.city_health -= PENALTY_VALUE;
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
