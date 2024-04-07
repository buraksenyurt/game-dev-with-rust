use crate::entity::{Drawable, Map};
use crate::factory::{GameObject, MainState};
use crate::resources::INIT_LEVEL;
use crate::ui::{GameOverMenu, MainMenu};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub enum GameState {
    Failed,
    ExitGame,
    MainMenu,
    NewGame,
    Playing(u8),
}

pub struct Game {
    pub current_map: Map,
    pub state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_map: Map::new(0, 9, 5),
            state: GameState::MainMenu,
        }
    }
}

impl GameObject for Game {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        _delta_time: Duration,
    ) -> MainState {
        match self.state {
            GameState::Failed => {
                GameOverMenu::draw(canvas, 0, 1.).unwrap();
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.state = GameState::MainMenu;
                            //self.restart();
                            break;
                        }
                        _ => {}
                    }
                }
                canvas.present();
            }
            GameState::ExitGame => self.state = GameState::ExitGame,
            GameState::MainMenu => {
                MainMenu::draw(canvas).unwrap();
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.state = GameState::ExitGame;
                            return MainState::Exit;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Return),
                            ..
                        } => {
                            self.state = GameState::NewGame;
                            break;
                        }
                        _ => {}
                    }
                }
                canvas.present();
            }
            GameState::NewGame => {
                let mut map = Map::new(0, 9, 5);
                map.load(INIT_LEVEL);
                self.current_map = map;
                self.state = GameState::Playing(0);
                //self.player.draw(canvas);
            }
            GameState::Playing(_level) => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                self.current_map.draw(canvas);

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.state = GameState::MainMenu;
                            break;
                        }
                        _ => {}
                    }
                }

                canvas.present();
            }
        }
        MainState::Running
    }
}
