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
