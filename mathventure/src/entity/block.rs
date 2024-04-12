use crate::entity::*;
use crate::factory::{Dimension, Math};
use crate::resources::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum BlockType {
    Wall,
    Tile,
    StoneTile,
    ExitDoor,
    QuestionTower,
    Ghost,
}
pub struct Block {
    pub idx: u32,
    pub dimension: Dimension,
    pub block_type: BlockType,
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BlockType::Wall => "wall",
            BlockType::Tile => "tile",
            BlockType::StoneTile => "stone_tile",
            BlockType::ExitDoor => "exit_door",
            BlockType::QuestionTower => "question_tower",
            BlockType::Ghost => "snake",
        };
        write!(f, "{}", value)
    }
}

impl Block {
    pub fn new(idx: u32, block_type: BlockType) -> Self {
        Self {
            idx,
            dimension: Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT),
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
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: &AssetManager) {
        let texture = texture_manager.get(&self.block_type.to_string());
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);

        let (x, y) = Math::get_position(self.idx, dimension.clone(), STANDARD_COLUMN_COUNT);
        let rect = Rect::new(x as i32, y as i32, dimension.width, dimension.height);

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
