use crate::entity::{Drawable, Map, Player};
use crate::factory::{GameObject, MainState};
use crate::resources::{INIT_LEVEL, STANDARD_COLUMN_COUNT, STANDARD_ROW_COUNT};
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
    pub player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_map: Map::new(0, STANDARD_COLUMN_COUNT, STANDARD_ROW_COUNT, 0),
            state: GameState::MainMenu,
            player: Player::new(0),
        }
    }
}

impl Game {
    pub fn init(&mut self, level: u8) {
        //TODO@Burak Must implement level based map loading
        let mut map = Map::new(0, STANDARD_COLUMN_COUNT, STANDARD_ROW_COUNT, 0);
        map.load(INIT_LEVEL);
        let player_idx = map.player_idx;
        self.player = Player::new(player_idx);
        self.current_map = map;
        self.state = GameState::Playing(level);
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
                self.init(0);
            }
            GameState::Playing(_level) => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                self.current_map.draw(canvas);
                self.player.draw(canvas);

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
                        Event::KeyDown {
                            keycode: Some(Keycode::Right),
                            ..
                        } => {
                            println!("Player index is {}", self.player.idx + 1);
                            self.player.idx += 1;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Left),
                            ..
                        } => {
                            println!("Player index is {}", self.player.idx - 1);
                            self.player.idx -= 1;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Up),
                            ..
                        } => {
                            println!(
                                "Player index is {}",
                                self.player.idx - STANDARD_COLUMN_COUNT
                            );
                            self.player.idx -= STANDARD_COLUMN_COUNT;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Down),
                            ..
                        } => {
                            println!(
                                "Player index is {}",
                                self.player.idx + STANDARD_COLUMN_COUNT
                            );
                            self.player.idx += STANDARD_COLUMN_COUNT;
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
