mod block;
mod map;

pub use block::*;
pub use map::Map;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Updatable {
    fn update(&mut self, delta_time: f32);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

// pub trait DrawableEntity: Entity + Drawable {}
// impl<T: Entity + Drawable + ?Sized> DrawableEntity for T {}
