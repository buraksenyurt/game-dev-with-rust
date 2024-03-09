mod constants;
mod entity;
mod game;
mod utility;

use crate::constants::*;
use crate::entity::{GameState, Hud, Shuttle, Vector};
use crate::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut shuttle = Shuttle::new();

    let window = video_subsystem
        .window("Lunar Landing 2049", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = Game::new();
    let mut last_update = Instant::now();
    let mut hud = Hud::new();

    'game_loop: loop {
        match game.state {
            GameState::Menu => {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                game.draw_main_menu(&mut canvas)?;

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            break 'game_loop;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Return),
                            ..
                        } => {
                            game = Game::new();
                            game.state = GameState::Playing;
                            shuttle = Shuttle::new();
                            continue 'game_loop;
                        }
                        _ => {}
                    }
                }

                canvas.present();
            }
            GameState::Playing => {
                loop {
                    let frame_duration = Duration::new(0, 1_000_000_000u32 / 60);
                    let now = Instant::now();
                    let delta = now.duration_since(last_update);
                    last_update = now;
                    let delta_seconds = delta.as_secs_f32();
                    if frame_duration > delta {
                        std::thread::sleep(frame_duration - delta);
                    }

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();

                    for event in event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => {
                                break 'game_loop;
                            }
                            Event::KeyDown {
                                keycode: Some(Keycode::Left),
                                ..
                            } => {
                                shuttle.velocity.x -= 30. * delta_seconds;
                            }
                            Event::KeyDown {
                                keycode: Some(Keycode::Right),
                                ..
                            } => {
                                shuttle.velocity.x += 30. * delta_seconds;
                            }
                            Event::KeyDown {
                                keycode: Some(Keycode::Down),
                                ..
                            } => {
                                shuttle.velocity.y += 75. * delta_seconds;
                                shuttle.fuel_level -= 2;
                            }
                            Event::KeyDown {
                                keycode: Some(Keycode::Space),
                                ..
                            } => {
                                shuttle.velocity.y -= 50. * delta_seconds;
                                shuttle.fuel_level -= 10;
                            }
                            _ => {}
                        }
                    }

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    game.move_meteors(delta_seconds);
                    game.check_out_of_ranges();
                    game.respawn_meteors();
                    game.draw(&mut canvas)?;
                    //println!("Current meteor count is {}", game.meteors.iter().count());
                    if shuttle.fuel_level <= 0 {
                        game.state = GameState::OutOfFuel;
                        continue 'game_loop;
                    }
                    if game.check_meteor_shuttle_collisions(&shuttle) {
                        game.state = GameState::MeteorHit;
                        continue 'game_loop;
                    }
                    if !shuttle.is_landed(&game) {
                        shuttle.toss_randomly(Vector { x: 40., y: 80. }, delta_seconds);
                        shuttle.velocity.y += 2.5 * delta_seconds;
                        shuttle.fuel_level -= 1;
                    } else {
                        game.state = GameState::JobsDone;
                        continue 'game_loop;
                    }
                    shuttle.draw(&mut canvas, Color::RGB(255, 255, 0))?;
                    hud.draw(&shuttle, &mut canvas)?;
                    canvas.present();
                }
            }
            GameState::OutOfFuel | GameState::JobsDone | GameState::MeteorHit => {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                game.draw_end_menu(&mut canvas)?;
                if handle_inputs(&mut event_pump, &mut game) {
                    break 'game_loop;
                }
                canvas.present();
            }
        }
    }

    Ok(())
}

fn handle_inputs(event_pump: &mut sdl2::EventPump, game: &mut Game) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                game.state = GameState::Menu;
            }
            _ => {}
        }
    }
    false
}
