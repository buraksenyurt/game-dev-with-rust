use crate::sprite_type::SpriteType;
use ggez::graphics;
use ggez::mint::{Point2, Vector2};

// Oyundaki hareket eden nesneleri ifade eden veri yapısı. Bu veri yapısı ile oyuncuyu,
// kayaları ve atışları ifade edebiliriz.
// Her birinin ortak özellikleri bu veri yapısında toplanıyor.
// Nesnenin ekrandaki konumu, türü, hızı, açısal hızı, ölçeği gibi...
#[derive(Debug)]
pub struct Sprite {
    pub kind: SpriteType,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub angle_velocity: f32,
    pub size: f32,
    pub life: f32,
    pub facing: f32, //Henüz anlamadım ama sanırım nesnenin dönüş değeri için kullanılacak
}

impl Sprite {
    pub fn new(
        kind: SpriteType,
        position: Point2<f32>,
        velocity: Vector2<f32>,
        angle_velocity: f32,
        size: f32,
        life: f32,
        facing: f32,
    ) -> Self {
        Self {
            kind,
            position,
            velocity,
            angle_velocity,
            size,
            life,
            facing,
        }
    }
}
