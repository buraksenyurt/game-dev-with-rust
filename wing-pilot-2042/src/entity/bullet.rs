use crate::entity::bullet_type::BulletType;
use crate::entity::owner::Owner;
use macroquad::math::Rect;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Vec2, RED, WHITE};
use macroquad::shapes::draw_rectangle;

pub struct Bullet {
    owner: Owner,
    pub location: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub is_alive: bool,
    pub bullet_type: BulletType,
}

impl Bullet {
    pub async fn spawn(owner: Owner, location: Vec2) -> Self {
        Self {
            owner,
            location,
            rotation: 0.,
            velocity: Vec2::default(),
            is_alive: true,
            bullet_type: BulletType::Any,
        }
    }

    pub async fn draw(&self) {
        match self.owner {
            Owner::Fighter | Owner::EnemyFighter => {
                draw_rectangle(self.location.x, self.location.y, 3., 3., WHITE);
            }
            Owner::EnemyBomber => {
                draw_rectangle(self.location.x, self.location.y, 6., 6., RED);
            }
            Owner::Warship => {
                let texture = load_texture("resources/ws_missile.png").await.unwrap();
                let params = DrawTextureParams {
                    rotation: self.rotation,
                    ..Default::default()
                };
                draw_texture_ex(texture, self.location.x, self.location.y, WHITE, params);
            }
        }
    }

    pub async fn get_rect(&self) -> Rect {
        let mut r = Rect::new(self.location.x, self.location.y, 0., 0.);
        match self.owner {
            Owner::Fighter | Owner::EnemyFighter => {
                r.w = 3.;
                r.h = 3.;
            }
            Owner::EnemyBomber => {
                r.w = 6.;
                r.h = 6.;
            }
            Owner::Warship => {
                r.w = 5.;
                r.h = 5.;
            }
        }
        r
    }
}
