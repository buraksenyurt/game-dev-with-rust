use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::time::Duration;

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

    pub fn update(&mut self, event_pump: &mut EventPump) {
        match self.state {
            GameState::Crashed => self.state = GameState::Crashed,
            GameState::ExitGame => self.state = GameState::ExitGame,
            GameState::MainMenu => {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.state = GameState::ExitGame;
                            break;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Return),
                            ..
                        } => {
                            self.state = GameState::Playing;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            GameState::NewGame => {
                self.state = GameState::Playing;
            }
            GameState::Playing => {
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
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
