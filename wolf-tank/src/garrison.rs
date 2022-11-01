use crate::Vec2;
use macroquad::prelude::*;

pub struct Garrison {
    pub id: usize,
    pub position: Vec2,
    pub collided: bool,
    pub texture: Texture2D,
}

impl Garrison {
    pub fn new(id: usize, position: Vec2, texture: Texture2D) -> Self {
        Self {
            id,
            position,
            collided: false,
            texture,
        }
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }
}
