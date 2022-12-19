use crate::entity::owner::Owner;
use macroquad::math::Rect;
use macroquad::prelude::{Vec2, BLACK, RED, WHITE};
use macroquad::shapes::draw_rectangle;

pub struct Bullet {
    owner: Owner,
    pub location: Vec2,
    pub velocity: Vec2,
    pub is_alive: bool,
}

impl Bullet {
    pub async fn spawn(owner: Owner, location: Vec2) -> Self {
        Self {
            owner,
            location,
            velocity: Vec2::default(),
            is_alive: true,
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
                draw_rectangle(self.location.x, self.location.y, 5., 5., BLACK);
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
