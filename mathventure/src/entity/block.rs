use crate::entity::*;
use crate::resources::*;
use crate::utility::get_position;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum BlockType {
    Wall,
    Tile,
    StoneTile,
    ExitDoor,
    QuestionTower,
    Ghost,
    Player,
    Ufo,
}
pub struct Block {
    pub idx: u32,
    pub width: u32,
    pub height: u32,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(idx: u32, block_type: BlockType) -> Self {
        Self {
            idx,
            width: BLOCK_WIDTH,
            height: BLOCK_HEIGHT,
            block_type,
        }
    }
}

impl Entity for Block {
    fn get_type(&self) -> BlockType {
        self.block_type
    }
    fn get_idx(&self) -> u32 {
        self.idx
    }
}

impl Drawable for Block {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        let texture = texture_manager.get_texture(&self.block_type);

        let (x, y) = get_position(self.idx);
        let rect = Rect::new(x as i32, y as i32, BLOCK_WIDTH, BLOCK_HEIGHT);

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
