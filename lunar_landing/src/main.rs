mod constants;
mod entity;
mod game;
mod setup;
mod ui;
mod utility;

use crate::constants::*;
use crate::entity::*;
use crate::game::Game;
use crate::setup::setup_commands;
use crate::ui::hud::Hud;
use crate::ui::{GameOverMenu, MainMenu};
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let (play_commands, menu_commands) = setup_commands().unwrap();

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
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                MainMenu::draw(&mut canvas)?;

                for event in event_pump.poll_iter() {
                    if let Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } = event
                    {
                        if let Some(command) = menu_commands.get(&keycode) {
                            game.state = command.execute().unwrap();
                            continue 'game_loop;
                        }
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
            GameState::Playing => loop {
                let frame_duration = Duration::new(0, 1_000_000_000u32 / 60);
                let now = Instant::now();
                game.delta_second = now.duration_since(last_update);
                last_update = now;
                if frame_duration > game.delta_second {
                    std::thread::sleep(frame_duration - game.delta_second);
                }

                canvas.set_draw_color(Color::BLACK);
                canvas.clear();

                for event in event_pump.poll_iter() {
                    if let Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } = event
                    {
                        if let Some(command) = play_commands.get(&keycode) {
                            command.execute(&mut shuttle, game.delta_second.as_secs_f32());
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
                }

                canvas.set_draw_color(Color::BLACK);
                if game.update(&mut shuttle).is_some() {
                    continue 'game_loop;
                }

                game.draw(&mut canvas)?;
                shuttle.draw(&mut canvas, Color::YELLOW)?;
                hud.draw(&shuttle, &mut canvas)?;
                canvas.present();
            },
            GameState::OutOfFuel | GameState::JobsDone | GameState::MeteorHit => {
                game.meteors.clear();
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                GameOverMenu::draw(&game.state, &mut canvas)?;
                for event in event_pump.poll_iter() {
                    if let Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } = event
                    {
                        if let Some(command) = menu_commands.get(&keycode) {
                            game.state = command.execute().unwrap();
                            if game.state == GameState::NewGame {
                                game.state = GameState::Menu;
                            }
                            continue 'game_loop;
                        }
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
