use crate::entity::Drawable;
use crate::factory::{AssetManager, Dimension, Location, Math, Rectangle};
use crate::resources::{BLOCK_HEIGHT, BLOCK_WIDTH, STANDARD_COLUMN_COUNT};
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

    pub fn get_rect(&self) -> Rectangle {
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);
        let player_position =
            Math::get_position(self.idx, dimension.clone(), STANDARD_COLUMN_COUNT);
        let location = Location::new(player_position.0 as i32, player_position.1 as i32);
        Rectangle::new(location, dimension)
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas<Window>, asset_manager: &AssetManager) {
        let texture = asset_manager.get("player");
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);

        let (x, y) = Math::get_position(self.idx, dimension.clone(), STANDARD_COLUMN_COUNT);
        let rect = Rect::new(x as i32, y as i32, dimension.width, dimension.height);

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
