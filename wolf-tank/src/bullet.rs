use crate::Vec2;
use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub shoot_at: f64,
    pub collided: bool,
    pub texture: Texture2D,
}

impl Bullet {
    pub fn draw(&self) {
        let params = DrawTextureParams {
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            params,
        );
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position ({}), Rotation {}",
            self.position, self.rotation
        )
    }
}
