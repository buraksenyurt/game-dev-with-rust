mod block;
mod map;
mod player;

pub use block::*;
pub use map::Map;
pub use player::*;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn get_type(&self) -> BlockType;
    fn get_idx(&self) -> u32;
}

pub trait Updatable {
    fn update(&mut self, delta_time: f32);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub trait DrawableEntity: Entity + Drawable {}
impl<T: Entity + Drawable + ?Sized> DrawableEntity for T {}
