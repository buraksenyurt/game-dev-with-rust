mod lib;

use crate::lib::bullet::Bullet;
use crate::lib::explosion::Explosion;
use crate::lib::game::Game;
use crate::lib::game_state::{GameState, Level};
use crate::lib::menu::{draw_dead_menu, draw_main_menu, draw_win_menu};
use crate::lib::missile::Missile;
use crate::lib::turret::Turret;
use crate::lib::{
    create_buildings, create_missiles, draw_buildings, draw_cursor, get_max, get_min, window_conf,
};
use lib::building::*;
use lib::constant::*;
use macroquad::audio;
use macroquad::audio::Sound;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);

    let hit_sound = audio::load_sound("resource/cannon_hit.ogg").await.unwrap();
    let turret_fire_sound = audio::load_sound("resource/rlauncher.ogg").await.unwrap();
    let explosion_sound = audio::load_sound("resource/explosion.ogg").await.unwrap();

    let mut game = Game::new();
    let buildings = create_buildings();
    let mut missiles: Vec<Missile> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();
    let mut mini_gunner = Turret::new();
    let rookie_level = Level::new(100, MAX_MISSILE_COUNT_SAME_TIME, 10, MISSILE_SPEED_FACTOR);
    clear_background(Color::default());

    loop {
        match game.game_state {
            GameState::Main => {
                draw_main_menu(&rookie_level);
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new();
                    game.game_state = GameState::Playing(rookie_level);
                    missiles = create_missiles(rookie_level.max_missile_count);
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            GameState::Playing(level) => {
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
                if game.city_health == 0 {
                    game.game_state = GameState::Dead;
                }
                if game.player_hit == level.total_missile_count {
                    println!(
                        "Level {} complete. Total hit {}",
                        level.difficulty, game.player_hit
                    );
                    game.game_state = GameState::Win;
                }

                draw_buildings(&buildings);
                draw_cursor();
                mini_gunner.draw();
                game.draw();

                if is_mouse_button_pressed(MouseButton::Left)
                    && bullets.len() < MAX_BULLET_ON_GAME
                    && mini_gunner.is_fire_suitable()
                {
                    audio::play_sound_once(turret_fire_sound);
                    let bullet = Bullet::spawn(mini_gunner.muzzle_point);
                    bullets.push(bullet);
                }

                for e in explosions.iter_mut() {
                    for m in missiles.iter_mut() {
                        check_clamping(explosion_sound, &mut game, e, m);
                    }
                }

                for b in bullets.iter_mut() {
                    if b.target.distance(b.position) < 1. {
                        b.is_alive = false;
                        let expl = Explosion::spawn(b.target);
                        explosions.push(expl);
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
                        m.position += m.velocity * level.missile_speed_factor;
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

                if missiles.len() <= level.max_missile_count as usize {
                    let mut new_missiles =
                        create_missiles(level.max_missile_count - missiles.len() as i32);
                    missiles.append(&mut new_missiles);
                }
            }
            GameState::Dead => {
                // println!("Commander! City has fatal damage.");
                draw_dead_menu();
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new();
                    explosions=Vec::new();
                    bullets=Vec::new();
                    game.game_state = GameState::Playing(rookie_level);
                    missiles = create_missiles(rookie_level.max_missile_count);
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            GameState::Win => {
                draw_win_menu();
                if is_key_pressed(KeyCode::Space) {
                    game = Game::new();
                    explosions=Vec::new();
                    bullets=Vec::new();
                    game.game_state = GameState::Playing(rookie_level);
                    missiles = create_missiles(rookie_level.max_missile_count);
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        next_frame().await
    }
}

fn check_clamping(explosion_sound: Sound, game: &mut Game, e: &mut Explosion, m: &mut Missile) {
    let mut nearest_point = Vec2::default();
    nearest_point.x = get_max(
        m.position.x,
        get_min(m.position.x + MISSILE_LENGTH, e.location.x),
    );
    nearest_point.y = get_max(
        m.position.y,
        get_min(m.position.y + MISSILE_LENGTH, e.location.y),
    );
    let distance = Vec2::new(
        e.location.x - nearest_point.x,
        e.location.y - nearest_point.y,
    );
    if distance.length() <= e.radius {
        //e.is_alive = false;
        audio::play_sound_once(explosion_sound);
        game.player_hit += 1;
        game.player_point += 10;
        m.is_alive = false;
    }
}
