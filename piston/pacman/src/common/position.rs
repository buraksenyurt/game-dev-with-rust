use crate::common::contants::PACMAN_START_POS;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: PACMAN_START_POS.0,
            y: PACMAN_START_POS.1,
        }
    }
}
