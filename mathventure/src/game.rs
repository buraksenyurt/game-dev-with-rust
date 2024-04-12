use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::libc::tcsendbreak;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::entity::*;
use crate::factory::*;
use crate::resources::*;
use crate::ui::*;

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

    fn spawn_ufo(&mut self, rng: &mut ThreadRng) {
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);
        let (mut x, mut y) = Math::get_position(
            self.current_map.tower_idx,
            dimension.clone(),
            STANDARD_COLUMN_COUNT,
        );
        x += (BLOCK_WIDTH / 2) - UFO_WIDTH / 2;
        y += (BLOCK_HEIGHT / 2) - UFO_HEIGHT / 2;

        // let (player_x, player_y) = get_position(self.player.idx);
        // let direction = get_unit_vector(x as f32, y as f32, player_x as f32, player_y as f32);
        // let velocity = Velocity::new((direction.0 * 200.0) as i32, (direction.1 * 200.0) as i32);

        let names = ["owl", "hippo", "giraffe"];
        let name = names[rng.gen_range(0..names.len())];
        let directions = [
            (-200, -200),
            (0, -200),
            (0, 200),
            (200, 0),
            (-200, 0),
            (200, 200),
        ];
        let direction = directions[rng.gen_range(0..directions.len())];
        let velocity = Vector::new(direction.0 as f32, direction.1 as f32);
        let ufo = Ufo::new(
            Location::new(x as i32, y as i32),
            velocity,
            Dimension::new(UFO_WIDTH, UFO_HEIGHT),
            name.to_string(),
        );
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
        asset_manager: &AssetManager,
        randomizer: &mut ThreadRng,
        delta_time: Duration,
    ) -> MainState {
        match self.state {
            GameState::Failed => {
                GameOverMenu::draw(canvas, 0).unwrap();
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
                if self.player.live == 0 {
                    self.state = GameState::Failed;
                } else {
                    let question = &self.current_level.question.description;
                    self.current_map.draw(canvas, asset_manager);
                    self.player.draw(canvas, asset_manager);
                    ConversationBox::draw(canvas, question, self.player.live);

                    for ufo in &mut self.ufo_list {
                        ufo.update(delta_time.as_secs_f32());
                    }
                    for ufo in &self.ufo_list {
                        ufo.draw(canvas, asset_manager);
                    }
                    for ufo in &mut self.ufo_list {
                        let r1 = self.player.get_rect();
                        let r2 = ufo.get_rect();
                        if !ufo.hit && Math::check_collision(r1, r2) {
                            ufo.hit = true;
                            self.player.live -= 1;
                        }
                    }

                    let now = Instant::now();
                    if now.duration_since(self.last_ufo_time) >= self.next_ufo_delay
                        && self.ufo_list.len() < MAX_UFO_COUNT
                    {
                        self.spawn_ufo(randomizer);
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

                    self.ufo_list.retain(|u| {
                        (u.location.x + u.dimension.width as i32) > 0
                            && u.location.x < SCREEN_WIDTH as i32
                            && (u.location.y + u.dimension.height as i32) > 0
                            && u.location.y < SCREEN_HEIGHT as i32
                            && !u.hit
                    });
                    //println!("Current ufo count {}", self.ufo_list.len());
                }

                canvas.present();
            }
        }
        MainState::Running
    }
}
