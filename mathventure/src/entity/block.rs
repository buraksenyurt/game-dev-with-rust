use crate::entity::*;
use crate::resources::*;
use crate::utility::get_position;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum BlockType {
    Wall,
    Tile,
    StoneTile,
    ExitDoor,
    QuestionTower,
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

impl Drawable for Block {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let texture = match self.block_type {
            BlockType::Wall => texture_creator.load_texture("assets/wall.png").unwrap(),
            BlockType::Tile => texture_creator.load_texture("assets/tile.png").unwrap(),
            BlockType::ExitDoor => texture_creator
                .load_texture("assets/exit_door.png")
                .unwrap(),
            BlockType::QuestionTower => texture_creator
                .load_texture("assets/question_tower.png")
                .unwrap(),
            BlockType::StoneTile => texture_creator
                .load_texture("assets/stone_tile.png")
                .unwrap(),
        };

        let (x, y) = get_position(self.idx, STANDARD_COLUMN_COUNT, BLOCK_HEIGHT, BLOCK_WIDTH);
        let rect = Rect::new(x as i32, y as i32, BLOCK_WIDTH, BLOCK_HEIGHT);

        canvas.copy(&texture, None, Some(rect)).unwrap();
    }
}
