use crate::constant::PADDLE_SPEED;
use crate::SCREEN_WIDTH;
use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;

// Oyundaki nesneleri Entity olarak tutabiliriz.
pub struct Entity {
    sprite: Texture,
    position: Vec2<f32>,
}

impl Entity {
    pub fn new(sprite: Texture, position: Vec2<f32>) -> Self {
        Self { sprite, position }
    }
    pub fn draw(&self, context: &mut Context) {
        self.sprite.draw(context, self.position);
    }
}

pub struct Player {
    pub core: Entity,
}

impl Player {
    pub fn go_left(&mut self) {
        if self.core.position.x > 0. {
            self.core.position.x -= PADDLE_SPEED;
        }
    }

    pub fn go_right(&mut self) {
        if self.core.position.x < SCREEN_WIDTH - self.core.sprite.width() as f32 {
            self.core.position.x += PADDLE_SPEED;
        }
    }
}
