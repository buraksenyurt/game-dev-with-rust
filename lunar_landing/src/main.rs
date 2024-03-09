mod constants;
mod entity;
mod game;
mod input;
mod utility;

use crate::constants::*;
use crate::entity::{GameState, Hud, Shuttle, Vector};
use crate::game::Game;
use crate::input::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let play_commands = get_play_commands();
    let menu_commands = get_menu_commands();

    let window = video_subsystem
        .window("Lunar Landing 2049", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = Game::new();
    let mut shuttle = Shuttle::new();
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
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {
                            if let Some(command) = menu_commands.get(&keycode) {
                                game.state = command.execute().unwrap();
                                continue 'game_loop;
                            }
                        }
                        _ => {}
                    }
                }

                canvas.present();
            }
            GameState::NewGame => {
                game = Game::new();
                shuttle = Shuttle::new();
                game.state = GameState::Playing;
                continue 'game_loop;
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
                            Event::KeyDown {
                                keycode: Some(keycode),
                                ..
                            } => {
                                if let Some(command) = play_commands.get(&keycode) {
                                    command.execute(&mut shuttle, delta_seconds);
                                }
                                if let Some(m_command) = menu_commands.get(&keycode) {
                                    if let Some(new_state) = m_command.execute() {
                                        game.state = new_state;
                                        if game.state == GameState::Menu {
                                            continue 'game_loop;
                                        }
                                    }
                                }
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
                game.meteors.clear();
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                game.draw_end_menu(&mut canvas)?;
                for event in event_pump.poll_iter() {
                    match event {
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {
                            if let Some(command) = menu_commands.get(&keycode) {
                                game.state = command.execute().unwrap();
                                if game.state == GameState::NewGame {
                                    game.state = GameState::Menu;
                                }
                                continue 'game_loop;
                            }
                        }
                        _ => {}
                    }
                }
                canvas.present();
            }
            GameState::ExitGame => {
                break 'game_loop;
            }
        }
    }

    Ok(())
}

fn get_play_commands() -> HashMap<Keycode, Box<dyn DirectionCommand>> {
    let mut command_map: HashMap<Keycode, Box<dyn DirectionCommand>> = HashMap::new();
    command_map.insert(Keycode::Left, Box::new(MoveLeftCommand));
    command_map.insert(Keycode::Right, Box::new(MoveRightCommand));
    command_map.insert(Keycode::Space, Box::new(MoveUpCommand));
    command_map.insert(Keycode::Down, Box::new(MoveDownCommand));
    command_map
}

fn get_menu_commands() -> HashMap<Keycode, Box<dyn MenuCommand>> {
    let mut command_map: HashMap<Keycode, Box<dyn MenuCommand>> = HashMap::new();
    command_map.insert(Keycode::Backspace, Box::new(ReturnToMenuCommand));
    command_map.insert(Keycode::Return, Box::new(StartNewGameCommand));
    command_map.insert(Keycode::Escape, Box::new(ExitGameCommand));
    command_map
}
