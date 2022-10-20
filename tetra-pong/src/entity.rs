use crate::constant::PADDLE_SPEED;
use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;
use crate::SCREEN_WIDTH;

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

    pub fn go_left(&mut self) {
        if self.position.x>0. {
            self.position.x -= PADDLE_SPEED;
        }
    }

    pub fn go_right(&mut self) {
        if self.position.x < SCREEN_WIDTH - self.sprite.width() as f32 {
            self.position.x += PADDLE_SPEED;
        }
    }
}
