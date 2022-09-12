use crate::constant::MAX_PHYSICS_VELOCITY;
use crate::sprite_type::SpriteType;
use ggez::graphics;
use ggez::mint::Point2;
type Vector2 = glam::Vec2;

// Oyundaki hareket eden nesneleri ifade eden veri yapısı. Bu veri yapısı ile oyuncuyu,
// kayaları ve atışları ifade edebiliriz.
// Her birinin ortak özellikleri bu veri yapısında toplanıyor.
// Nesnenin ekrandaki konumu, türü, hızı, açısal hızı, ölçeği gibi...
#[derive(Debug)]
pub struct Sprite {
    pub kind: SpriteType,
    pub position: Point2<f32>,
    pub velocity: Vector2,
    pub angle_velocity: f32,
    pub size: f32,
    pub life: f32,
    pub facing: f32, //Henüz anlamadım ama sanırım nesnenin dönüş değeri için kullanılacak
}

impl Sprite {
    pub fn new(
        kind: SpriteType,
        position: Point2<f32>,
        velocity: Vector2,
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

    // Aktörün konumunu yeniden belirleyen fonksiyon.
    pub fn update_position(&mut self, delta_time: f32) {
        let square_velocity = self.velocity.length_squared();
        if square_velocity > MAX_PHYSICS_VELOCITY.powi(2) {
            self.velocity.x = self.velocity.x / square_velocity.sqrt() * MAX_PHYSICS_VELOCITY;
            self.velocity.y = self.velocity.y / square_velocity.sqrt() * MAX_PHYSICS_VELOCITY;
        }
        let direction_velocity = Vector2 {
            x: self.velocity.x * delta_time,
            y: self.velocity.y * delta_time,
        };
        self.position.x += direction_velocity.x;
        self.position.y += direction_velocity.y;
        self.facing += self.angle_velocity;
    }

    // Karakter ekranın herhangibir noktasında çıkarsa ters taraftan tekrar girmesini
    // sağlayan fonksiyon
    pub fn wrap_position(&mut self, x: f32, y: f32) {
        let (x_bounds, y_bounds) = (x / 2., y / 2.);
        if self.position.x > x_bounds {
            self.position.x -= x;
        } else if self.position.x < -x_bounds {
            self.position.x += x;
        };
        if self.position.y > y_bounds {
            self.position.y -= y;
        } else if self.position.y < -y_bounds {
            self.position.y += y;
        }
    }
}
