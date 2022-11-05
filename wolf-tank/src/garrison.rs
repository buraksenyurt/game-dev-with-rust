use crate::Vec2;
use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Garrison {
    pub id: usize,
    pub position: Vec2,
    pub direction: Vec2,
    pub collided: bool,
    pub texture: Texture2D,
}

impl Garrison {
    pub fn new(id: usize, position: Vec2, texture: Texture2D) -> Self {
        Self {
            id,
            position,
            direction: vec2(1., 0.),
            collided: false,
            texture,
        }
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }
}
