use crate::factory::{GameObject, MainState};
use crate::resources::TextureManager;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};

pub struct Engine {
    pub game: Box<dyn GameObject>,
    pub fps: u32,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);
        texture_manager.add("player", "assets/player.png");
        texture_manager.add("wall", "assets/wall.png");
        texture_manager.add("tile", "assets/tile.png");
        texture_manager.add("exit_door", "assets/exit_door.png");
        texture_manager.add("question_tower", "assets/question_tower.png");
        texture_manager.add("ghost", "assets/snake.png");
        texture_manager.add("stone_tile", "assets/stone_tile.png");
        texture_manager.add("ufo_1", "assets/owl.png");
        texture_manager.add("ufo_2", "assets/hippo.png");
        texture_manager.add("ufo_3", "assets/giraffe.png");

        let mut rng = rand::thread_rng();

        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_update);

            let state = self.game.update(
                &mut self.event_pump,
                &mut self.canvas,
                &texture_manager,
                &mut rng,
                delta,
            );

            match state {
                MainState::Exit => break,
                MainState::Running => {}
            }

            last_update = now;
            if frame_duration > delta {
                std::thread::sleep(frame_duration - delta);
            }
        }

        Ok(())
    }
}
