use crate::constant::PADDLE_SPEED;
use crate::SCREEN_WIDTH;
use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

// Oyundaki nesneleri Entity olarak tutabiliriz.
pub struct Entity {
    pub sprite: Texture,
    pub position: Vec2<f32>,
}

impl Entity {
    pub fn new(sprite: Texture, position: Vec2<f32>) -> Self {
        Self { sprite, position }
    }
    pub fn draw(&self, context: &mut Context) {
        self.sprite.draw(context, self.position);
    }
    // Axis-Aligned Bounding Boxes (AABB) yöntemine göre çarpışma kontrolü yapılacak
    // Dolayısıyla sprite'ın dörtgen sınırlarının bilmemiz lazım.
    // Aşağıdaki fonksiyon bunun için bir yardımcı
    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.sprite.width() as f32,
            self.sprite.height() as f32,
        )
    }
    // Nesnenin merkez koordinatlarını bulan fonksiyon
    pub fn get_centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.sprite.width() as f32 * 0.5),
            self.position.y + (self.sprite.height() as f32 * 0.5),
        )
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

pub struct Ball {
    pub core: Entity,
    pub velocity: Vec2<f32>,
}

impl Ball {
    pub fn new(core: Entity, velocity: Vec2<f32>) -> Self {
        Self { core, velocity }
    }
}
