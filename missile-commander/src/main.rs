mod lib;

use crate::lib::bullet::Bullet;
use crate::lib::explosion::Explosion;
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
    let mut explosions: Vec<Explosion> = Vec::new();
    let mut mini_gunner = Turret::new();
    clear_background(Color::default());

    loop {
        if game.city_health == 0 {
            println!("Commander! City has fatal damage.");
            break;
        }

        draw_buildings(&buildings);
        draw_cursor();
        mini_gunner.draw();
        game.draw();

        if is_mouse_button_pressed(MouseButton::Left)
            && bullets.len() < MAX_BULLET_ON_GAME
            && mini_gunner.is_fire_suitable()
        {
            let bullet = Bullet::spawn(mini_gunner.muzzle_point);
            bullets.push(bullet);
        }

        for b in bullets.iter_mut() {
            if b.target.distance(b.position) < 1. {
                b.is_alive = false;
                let expl = Explosion::spawn(b.target);
                explosions.push(expl);
                //println!("On target point {}", b.target);
            }
            b.position += b.velocity * BULLET_SPEED_FACTOR;
            b.draw();
        }

        for e in explosions.iter_mut() {
            e.draw();
            if e.life_time == 0 {
                e.is_alive = false;
            } else {
                e.radius += EXPLOSION_RADIUS_RATE;
                e.life_time -= 1;
            }
        }

        for m in missiles.iter_mut() {
            if m.lift_off_time == 0 {
                m.position += m.velocity * MISSILE_SPEED_FACTOR;
                m.draw();

                if m.position.y > screen_height() - CITY_HEIGHT {
                    m.is_alive = false;
                    game.city_health -= PENALTY_VALUE;
                    audio::play_sound_once(hit_sound);
                }
            } else {
                m.lift_off_time -= 1;
            }
        }

        explosions.retain(|e| e.is_alive);
        bullets.retain(|b| b.is_alive);
        missiles.retain(|m| m.is_alive);

        let mut new_missiles = create_missiles(MAX_MISSILE_COUNT - missiles.len() as i32);
        missiles.append(&mut new_missiles);
        next_frame().await
    }
}
