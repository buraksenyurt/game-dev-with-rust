mod entity;
mod game;
mod menu;
mod stage;

use crate::entity::missile::create_missiles;
use crate::game::utility::window_conf;
use crate::game::utility::{draw_cursor, get_max, get_min};
use crate::stage::stage::Stage;
use entity::bullet::Bullet;
use entity::city::*;
use entity::explosion::Explosion;
use entity::missile::Missile;
use entity::turret::Turret;
use game::constant::*;
use game::game::Game;
use game::state::State;
use macroquad::audio;
use macroquad::prelude::*;
use menu::menu::{draw_dead_menu, draw_end_menu, draw_main_menu, draw_win_menu};
use stage::builder::load_stages;

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);

    let hit_sound = audio::load_sound("resource/cannon_hit.ogg").await.unwrap();
    let turret_fire_sound = audio::load_sound("resource/rlauncher.ogg").await.unwrap();
    let explosion_sound = audio::load_sound("resource/explosion.ogg").await.unwrap();

    let mut game = Game::new();
    let buildings = create_buildings();
    let mut mini_gunner = Turret::new();
    let stages = load_stages();
    clear_background(Color::default());

    loop {
        match game.state {
            State::Main => {
                draw_main_menu(&stages[0]);
                if is_key_pressed(KeyCode::Space) {
                    game = init_game(stages[0]);
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            State::Playing(stage) => {
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
                if game.score.city_health == 0 {
                    game.state = State::Dead;
                }
                if game.score.total_hit == stage.total_missile_count {
                    game.state = State::Win;
                }

                draw(&buildings);
                draw_cursor();
                mini_gunner.draw();
                game.draw();

                if is_mouse_button_pressed(MouseButton::Left)
                    && game.bullets.len() < MAX_BULLET_ON_GAME
                    && mini_gunner.is_fire_suitable()
                {
                    audio::play_sound_once(turret_fire_sound);
                    let bullet = Bullet::spawn(mini_gunner.muzzle_point);
                    game.bullets.push(bullet);
                }

                for e in game.explosions.iter_mut() {
                    for m in game.missiles.iter_mut() {
                        let nearest_point = Vec2::new(
                            get_max(
                                m.position.x,
                                get_min(m.position.x + MISSILE_LENGTH, e.location.x),
                            ),
                            get_max(
                                m.position.y,
                                get_min(m.position.y + MISSILE_LENGTH, e.location.y),
                            ),
                        );
                        let distance = Vec2::new(
                            e.location.x - nearest_point.x,
                            e.location.y - nearest_point.y,
                        );
                        if distance.length() <= e.radius {
                            //e.is_alive = false;
                            audio::play_sound_once(explosion_sound);
                            game.score.total_hit += 1;
                            game.score.total_point += 10;
                            m.is_alive = false;
                            continue;
                        }
                    }
                }

                for b in game.bullets.iter_mut() {
                    if b.target.distance(b.position) < 1. {
                        b.is_alive = false;
                        let expl = Explosion::spawn(b.target);
                        game.explosions.push(expl);
                    }
                    b.position += b.velocity * BULLET_SPEED_FACTOR;
                    b.draw();
                }

                for e in game.explosions.iter_mut() {
                    e.draw();
                    if e.life_time == 0 {
                        e.is_alive = false;
                    } else {
                        e.radius += EXPLOSION_RADIUS_RATE;
                        e.life_time -= 1;
                    }
                }

                for m in game.missiles.iter_mut() {
                    if m.lift_off_time == 0 {
                        m.position += m.velocity * stage.missile_speed_factor;
                        m.draw();

                        if m.position.y > screen_height() - CITY_HEIGHT {
                            m.is_alive = false;
                            game.score.city_health -= PENALTY_VALUE;
                            audio::play_sound_once(hit_sound);
                        }
                    } else {
                        m.lift_off_time -= 1;
                    }
                }

                game.explosions.retain(|e| e.is_alive);
                game.bullets.retain(|b| b.is_alive);
                game.missiles.retain(|m| m.is_alive);

                if game.missiles.len() <= stage.max_missile_count as usize {
                    let mut new_missiles =
                        create_missiles(stage.max_missile_count - game.missiles.len() as i32);
                    game.missiles.append(&mut new_missiles);
                }
            }
            State::Dead => {
                // println!("Commander! City has fatal damage.");
                draw_dead_menu(&game);
                if is_key_pressed(KeyCode::Space) {
                    game = init_game(stages[0]);
                } else if is_key_pressed(KeyCode::Escape) {
                    game.state = State::Main;
                }
            }
            State::Win => {
                draw_win_menu(&game);
                if game.current_stage == stages.len() {
                    game.state = State::End;
                }
                if is_key_pressed(KeyCode::Space) {
                    game.current_stage += 1;
                    game = init_game(stages[game.current_stage]);
                } else if is_key_pressed(KeyCode::Escape) {
                    game.state = State::Main;
                }
            }
            State::End => {
                draw_end_menu(&game);
                if is_key_pressed(KeyCode::Enter) {
                    println!("Developer Burak Selim Şenyurt");
                } else if is_key_pressed(KeyCode::Escape) {
                    game.state = State::Main;
                }
            }
        }

        next_frame().await
    }
}

fn init_game(stage: Stage) -> Game {
    let mut game = Game::new();
    game.state = State::Playing(stage);
    game.missiles = create_missiles(stage.max_missile_count);
    game
}
