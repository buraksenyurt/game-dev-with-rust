use macroquad::prelude::*;

pub struct Building {
    position: Vec2,
    size: Vec2,
}

impl Building {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            LIGHTGRAY,
        );
    }
}
