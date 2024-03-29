use crate::constants::*;
use crate::entity::flappy::Flappy;
use crate::entity::{Block, BlockDirection, Drawable, Entity};
use crate::ui::Hud;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};

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
    pub blocks: Vec<Block>,
    pub player: Flappy,
    last_block_time: Instant,
    next_block_delay: Duration,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            point: 0,
            state: GameState::MainMenu,
            delta_second: Duration::default(),
            player: Flappy::default(),
            blocks: Vec::new(),
            last_block_time: Instant::now(),
            next_block_delay: Duration::from_secs(rng.gen_range(3..=5)),
        }
    }

    pub fn update(&mut self, event_pump: &mut EventPump, canvas: &mut Canvas<Window>) {
        match self.state {
            GameState::Crashed => {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.state = GameState::MainMenu;
                            self.restart();
                            break;
                        }
                        _ => {}
                    }
                }
            }
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
                self.player.draw(canvas);
            }
            GameState::Playing => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();

                self.player.y += 1;
                if self.check_collision() || self.player.is_out_of_border() {
                    self.state = GameState::Crashed;
                    return;
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
                            keycode: Some(Keycode::Space),
                            ..
                        } => {
                            self.player.y -= 10;
                        }
                        _ => {}
                    }
                }

                self.player.update(self.delta_second.as_secs_f32());
                self.player.draw(canvas);

                let now = Instant::now();
                if now.duration_since(self.last_block_time) >= self.next_block_delay
                    && self.blocks.len() < MAX_BLOCK_COUNT
                {
                    self.spawn_block();
                    self.last_block_time = now;
                    self.next_block_delay =
                        Duration::from_secs(rand::thread_rng().gen_range(3..=5));
                }

                for block in &mut self.blocks {
                    block.update(self.delta_second.as_secs_f32());
                }

                self.blocks
                    .retain(|block| block.x + block.width as i32 + 10 > 0);

                for block in &self.blocks {
                    block.draw(canvas);
                }
                self.count_point();
                Hud::draw(canvas, self.point).unwrap();

                canvas.present();
            }
        }
    }

    fn spawn_block(&mut self) {
        let mut rng = rand::thread_rng();
        let heights = [80, 180, 240, 300];
        let widths = [40, 45, 50];

        let height = heights[rng.gen_range(0..heights.len())];
        let width = widths[rng.gen_range(0..widths.len())];
        let direction: BlockDirection;
        let y = match rng.gen_range(0..100) % 3 == 0 {
            false => {
                direction = BlockDirection::BottomToUp;
                SCREEN_HEIGHT as i32 - height as i32
            }
            true => {
                direction = BlockDirection::TopToBottom;
                0
            }
        };

        let block = Block {
            x: SCREEN_WIDTH as i32,
            y,
            width,
            height,
            x_velocity: -100.,
            direction,
            counted: false,
        };
        self.blocks.push(block);
    }

    fn check_collision(&mut self) -> bool {
        for b in self.blocks.iter() {
            if b.intersects(&self.player) {
                return true;
            }
        }
        false
    }

    fn count_point(&mut self) {
        for block in self.blocks.iter_mut() {
            if !block.counted && block.x < -(block.width as i32) {
                block.counted = true;
                self.point += 10;
            }
        }
    }
    fn restart(&mut self) {
        self.blocks.clear();
        self.point = 0;
        self.player = Flappy::default();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
