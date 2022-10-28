use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

pub struct Tank {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub direction: Vec2,
    pub texture: Texture2D,
}

impl Tank {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            position: Vec2::new(
                screen_width() * 0.5 - texture.width() * 0.5,
                screen_height() * 0.5 - texture.height() * 0.5,
            ),
            texture: texture,
            velocity: Vec2::new(0., 0.),
            rotation: 0.,
            direction: Vec2::new(1., 0.),
        }
    }

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

impl Display for Tank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position ({}), Rotation {}, Velocity {}, Direction {}",
            self.position, self.rotation, self.velocity, self.direction
        )
    }
}
