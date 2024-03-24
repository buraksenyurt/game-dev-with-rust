mod constants;
mod ui;

use crate::constants::*;
use crate::ui::MainMenu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Monochrome Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = Game::default();
    let mut last_update = Instant::now();

    loop {
        match game.state {
            GameState::Crashed => {}
            GameState::ExitGame => break,
            GameState::MainMenu => {
                MainMenu::draw(&mut canvas)?;
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            game.state = GameState::ExitGame;
                            break;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Return),
                            ..
                        } => {
                            game.state = GameState::Playing;
                            break;
                        }
                        _ => {}
                    }
                }

                canvas.present();
            }
            GameState::NewGame => {
                game = Game::new();
                game.state = GameState::Playing;
                continue;
            }
            GameState::Playing => {
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
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            game.state = GameState::MainMenu;
                            break;
                        }
                        _ => {}
                    }
                }

                canvas.present();
            }
        }
    }

    Ok(())
}

#[derive(PartialEq)]
pub enum GameState {
    Crashed,
    ExitGame,
    MainMenu,
    NewGame,
    Playing,
}

pub struct Game {
    pub point: u64,
    pub state: GameState,
    pub delta_second: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self {
            point: 0,
            state: GameState::MainMenu,
            delta_second: Duration::default(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
