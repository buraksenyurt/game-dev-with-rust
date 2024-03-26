use crate::constants::*;
use crate::entity::flappy::Flappy;
use crate::entity::{Block, Drawable, Entity};
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
                // canvas.set_draw_color(Color::BLACK);
                // canvas.clear();
                self.player.draw(canvas);
                // canvas.present();
            }
            GameState::Playing => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();

                self.player.y += 1;

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

                self.blocks.retain(|block| block.x + block.width as i32 > 0);

                for block in &self.blocks {
                    block.draw(canvas);
                }

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
        let y = match rng.gen_range(0..100) % 3 == 0 {
            false => SCREEN_HEIGHT as i32 - height as i32,
            true => 0,
        };

        let block = Block {
            x: SCREEN_WIDTH as i32,
            y,
            width,
            height,
            x_velocity: -100.,
        };
        self.blocks.push(block);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
