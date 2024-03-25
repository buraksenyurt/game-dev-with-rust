pub mod block;
pub mod flappy;

pub use block::*;
pub use flappy::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

trait Entity {
    fn update(&mut self, delta_time: f32);
    fn position(&self) -> (i32, i32);
    fn set_position(&mut self, x: i32, y: i32);
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
