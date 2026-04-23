pub mod block;
pub mod flappy;

pub use block::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn update(&mut self, delta_time: f32);
    fn position(&self) -> (i32, i32);
    fn set_position(&mut self, x: i32, y: i32);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
