use rand::Rng;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::entity::{BlockType, Drawable, Map, Player, Ufo, Updatable};
use crate::factory::{GameObject, MainState};
use crate::resources::{
    Level, LevelManager, TextureManager, Velocity, BLOCK_HEIGHT, BLOCK_WIDTH, MAX_UFO_COUNT,
    STANDARD_COLUMN_COUNT, UFO_HEIGHT, UFO_WIDTH,
};
use crate::ui::{ConversationBox, GameOverMenu, MainMenu};
use crate::utility::get_position;

pub enum GameState {
    Failed,
    ExitGame,
    MainMenu,
    NewGame,
    Playing(u32),
}

pub struct Game {
    pub current_map: Map,
    pub current_level: Level,
    pub state: GameState,
    pub player: Player,
    pub max_level: u32,
    last_ufo_time: Instant,
    next_ufo_delay: Duration,
    pub ufo_list: Vec<Ufo>,
}

impl Default for Game {
    fn default() -> Self {
        let level_manager = LevelManager::init();
        let current_level = level_manager.get_level(0).unwrap();
        Self {
            current_map: Map::default(),
            state: GameState::MainMenu,
            player: Player::new(0),
            current_level,
            max_level: level_manager.max_level_count,
            last_ufo_time: Instant::now(),
            next_ufo_delay: Duration::default(),
            ufo_list: Vec::new(),
        }
    }
}

impl Game {
    pub fn init(&mut self, level_index: u32) {
        let mut rng = rand::thread_rng();
        let level_manager = LevelManager::init();
        let level = level_manager.get_level(level_index).unwrap();

        let mut map = Map::default();
        map.load(level.map.as_str());
        let player_idx = map.player_idx;
        self.player = Player::new(player_idx);
        self.current_map = map;
        self.current_level = level;
        self.state = GameState::Playing(level_index);
        self.last_ufo_time = Instant::now();
        self.next_ufo_delay = Duration::from_secs(rng.gen_range(2..=4))
    }

    pub fn move_player(&mut self, direction: Direction) {
        let target_index = match direction {
            Direction::Down => self.player.idx + STANDARD_COLUMN_COUNT,
            Direction::Left => self.player.idx - 1,
            Direction::Right => self.player.idx + 1,
            Direction::Up => self.player.idx - STANDARD_COLUMN_COUNT,
        };
        let target_type = self.current_map.tiles[target_index as usize].get_type();
        if target_type == BlockType::StoneTile || target_type == BlockType::Tile {
            self.player.idx = target_index
        }
    }

    fn spawn_ufo(&mut self) {
        let (mut x, mut y) = get_position(
            self.current_map.tower_idx,
            STANDARD_COLUMN_COUNT,
            BLOCK_HEIGHT,
            BLOCK_WIDTH,
        );
        x += (BLOCK_WIDTH / 2) - UFO_WIDTH / 2;
        y += (BLOCK_HEIGHT / 2) - UFO_HEIGHT / 2;
        let velocity = Velocity::new(-100.0, -100.0);
        let ufo = Ufo::new(x as i32, y as i32, velocity, UFO_WIDTH, UFO_HEIGHT);
        self.ufo_list.push(ufo);
    }
}
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl GameObject for Game {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        delta_time: Duration,
    ) -> MainState {
        let texture_creator = canvas.texture_creator();
        let texture_manager = TextureManager::new(&texture_creator);

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

                let question = &self.current_level.question.description;
                self.current_map.draw(canvas, &texture_manager);
                self.player.draw(canvas, &texture_manager);
                ConversationBox::draw(canvas, question);

                let now = Instant::now();
                if now.duration_since(self.last_ufo_time) >= self.next_ufo_delay
                    && self.ufo_list.len() < MAX_UFO_COUNT
                {
                    self.spawn_ufo();
                    self.last_ufo_time = now;
                }

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
                            self.move_player(Direction::Right);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Left),
                            ..
                        } => {
                            self.move_player(Direction::Left);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Up),
                            ..
                        } => {
                            self.move_player(Direction::Up);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Down),
                            ..
                        } => {
                            self.move_player(Direction::Down);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::N),
                            keymod,
                            ..
                        } => {
                            if keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD)
                                && self.current_level.id < self.max_level
                            {
                                let new_level = self.current_level.id + 1;
                                self.init(new_level);
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::P),
                            keymod,
                            ..
                        } => {
                            if keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD)
                                && self.current_level.id > 0
                            {
                                let new_level = self.current_level.id - 1;
                                self.init(new_level);
                            }
                        }
                        _ => {}
                    }
                }

                for ufo in &mut self.ufo_list {
                    ufo.update(delta_time.as_secs_f32());
                    ufo.draw(canvas, &texture_manager);
                }
                self.ufo_list.retain(|u| u.x + u.width as i32 - 10 > 0);
                //println!("Current ufo count {}", self.ufo_list.len());

                canvas.present();
            }
        }
        MainState::Running
    }
}
