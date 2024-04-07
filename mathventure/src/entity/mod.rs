mod exit_door;
mod map;
mod question_tower;
mod tile;
mod wall;

pub use exit_door::ExitDoor;
pub use map::Map;
pub use question_tower::QuestionTower;
use std::any::Any;
pub use tile::Tile;
pub use wall::Wall;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn update(&mut self, delta_time: f32);
    fn as_any(&self) -> &dyn Any;
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
