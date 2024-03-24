use crate::constants::*;
use crate::game::{Game, GameState};
use crate::ui::MainMenu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};

pub struct Engine {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    game: Game,
    fps: u32,
}

impl Engine {
    pub fn new(game: Game, fps: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Monochrome Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            sdl_context,
            canvas,
            event_pump,
            game,
            fps,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);

        loop {
            match self.game.state {
                GameState::Crashed => {}
                GameState::ExitGame => break,
                GameState::MainMenu => {
                    MainMenu::draw(&mut self.canvas)?;
                    for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => {
                                self.game.state = GameState::ExitGame;
                                break;
                            }
                            Event::KeyDown {
                                keycode: Some(Keycode::Return),
                                ..
                            } => {
                                self.game.state = GameState::Playing;
                                break;
                            }
                            _ => {}
                        }
                    }

                    self.canvas.present();
                }
                GameState::NewGame => {
                    self.game = Game::new();
                    self.game.state = GameState::Playing;
                    continue;
                }
                GameState::Playing => {
                    self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.clear();

                    for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => {
                                self.game.state = GameState::MainMenu;
                                break;
                            }
                            _ => {}
                        }
                    }

                    self.canvas.present();
                }
            }

            let now = Instant::now();
            self.game.delta_second = now.duration_since(last_update);
            last_update = now;
            if frame_duration > self.game.delta_second {
                std::thread::sleep(frame_duration - self.game.delta_second);
            }
        }

        Ok(())
    }
}
