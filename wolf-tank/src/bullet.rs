use crate::Vec2;
use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub shoot_at: f32,
    pub collided: bool,
    pub texture: Texture2D,
}

impl Bullet {
    pub fn draw(&self) {
        let mut params = DrawTextureParams::default();
        params.rotation = self.rotation;
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            params,
        );
    }
}
