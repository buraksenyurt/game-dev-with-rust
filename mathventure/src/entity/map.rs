use crate::entity::*;

pub struct Map {
    pub level: u32,
    pub column_count: u32,
    pub row_count: u32,
    pub entities: Vec<Box<dyn Drawable>>,
}

impl Map {
    pub fn new(level: u32, column_count: u32, row_count: u32) -> Self {
        Self {
            level,
            column_count,
            row_count,
            entities: vec![],
        }
    }

    pub fn load(&mut self, map_content: &str) {
        let rows = map_content.split('\n');
        let mut idx = 0;
        for row in rows {
            for entity in row.chars() {
                let e: Box<dyn Drawable> = match entity {
                    'w' => Box::new(Block::new(idx, BlockType::Wall)),
                    'e' => Box::new(Block::new(idx, BlockType::ExitDoor)),
                    'q' => Box::new(Block::new(idx, BlockType::QuestionTower)),
                    's' => Box::new(Block::new(idx, BlockType::StoneTile)),
                    _ => Box::new(Block::new(idx, BlockType::Tile)),
                };
                idx += 1;
                self.entities.push(e)
            }
        }
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for (idx, entity) in self.entities.iter().enumerate() {
            entity.draw(canvas);
        }
    }
}
