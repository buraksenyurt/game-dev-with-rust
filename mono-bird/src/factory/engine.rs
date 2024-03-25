use crate::game::{Game, GameState};
use crate::ui::MainMenu;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};

pub struct Engine {
    pub game: Game,
    pub fps: u32,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);

        loop {
            self.game.update(&mut self.event_pump, &mut self.canvas);

            match self.game.state {
                GameState::Crashed => {}
                GameState::ExitGame => break,
                GameState::MainMenu => {
                    MainMenu::draw(&mut self.canvas)?;
                    self.canvas.present();
                }
                GameState::NewGame => {
                    self.game.state = GameState::Playing;
                    continue;
                }
                GameState::Playing => {}
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
