pub mod engine;
pub mod engine_builder;
pub mod screen;

pub use engine::Engine;
pub use engine_builder::EngineBuilder;
pub use screen::Screen;

use crate::resources::AssetManager;
use rand::rngs::ThreadRng;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub trait GameObject {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        texture_manager: &AssetManager,
        randomizer: &mut ThreadRng,
        delta_time: Duration,
    ) -> MainState;
}

pub enum MainState {
    Exit,
    Running,
}
