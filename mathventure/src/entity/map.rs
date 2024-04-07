use crate::entity::question_tower::QuestionTower;
use crate::entity::*;

pub struct Map {
    pub level: u32,
    pub column_count: u32,
    pub row_count: u32,
    pub entities: Vec<Box<dyn Entity>>,
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
                let e: Box<dyn Entity> = match entity {
                    'w' => Box::new(Wall::new(idx, 32, 32)),
                    'e' => Box::new(ExitDoor::new(idx, 32, 32)),
                    'q' => Box::new(QuestionTower::new(idx, 32, 32)),
                    _ => Box::new(Tile::new(idx, 32, 32)),
                };
                idx += 1;
                self.entities.push(e)
            }
        }
    }
}
