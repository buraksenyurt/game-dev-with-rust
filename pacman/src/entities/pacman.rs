use crate::common::direction::Direction;
use crate::common::position::Position;
use crate::entities::map::Map;

pub struct Pacman {
    map: Map,
    pos: Position,
    dir: Direction,
    target_dir: Direction,
    ticks: u32,
    score: u32,
    lives: u8,
}

impl Pacman {
    pub fn get_map(&self) -> &Map {
        &self.map
    }
}

impl Default for Pacman {
    fn default() -> Self {
        Self {
            map: Map::default(),
            pos: Position::default(),
            dir: Direction::Left,
            target_dir: Direction::Left,
            ticks: 0,
            lives: 3,
            score: 0,
        }
    }
}
