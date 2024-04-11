use crate::entity::{BlockType, Drawable};
use crate::resources::{TextureManager, BLOCK_HEIGHT, BLOCK_WIDTH};
use crate::utility::get_position;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Player {
    pub idx: u32,
}

impl Player {
    pub fn new(idx: u32) -> Self {
        Self { idx }
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        let texture = texture_manager.get("player");

        let (x, y) = get_position(self.idx);
        let rect = Rect::new(x as i32, y as i32, BLOCK_WIDTH, BLOCK_HEIGHT);

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
