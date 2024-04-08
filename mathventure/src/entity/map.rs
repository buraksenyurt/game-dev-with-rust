use crate::entity::*;

pub struct Map {
    pub level: u32,
    pub column_count: u32,
    pub row_count: u32,
    pub tiles: Vec<Box<dyn DrawableEntity>>,
    pub player_idx: u32,
}

impl Map {
    pub fn new(level: u32, column_count: u32, row_count: u32, player_idx: u32) -> Self {
        Self {
            level,
            column_count,
            row_count,
            tiles: vec![],
            player_idx,
        }
    }

    pub fn load(&mut self, map_content: &str) {
        let rows = map_content.split('\n');
        let mut idx = 0;
        for row in rows {
            for c in row.chars() {
                let e: Box<dyn DrawableEntity> = match c {
                    'w' => Box::new(Block::new(idx, BlockType::Wall)),
                    'e' => Box::new(Block::new(idx, BlockType::ExitDoor)),
                    'q' => Box::new(Block::new(idx, BlockType::QuestionTower)),
                    's' => Box::new(Block::new(idx, BlockType::StoneTile)),
                    _ => Box::new(Block::new(idx, BlockType::Tile)),
                };
                idx += 1;
                self.tiles.push(e);
                if c == 'p' {
                    self.player_idx = idx - 1;
                }
            }
        }
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for entity in self.tiles.iter() {
            entity.draw(canvas);
        }
    }
}
