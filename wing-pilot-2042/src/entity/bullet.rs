use crate::entity::owner::Owner;
use macroquad::prelude::{Vec2, BLACK, RED, WHITE};
use macroquad::shapes::draw_rectangle;

pub struct Bullet {
    owner: Owner,
    pub location: Vec2,
    pub is_alive: bool,
}

impl Bullet {
    pub async fn spawn(owner: Owner, location: Vec2) -> Self {
        Self {
            owner,
            location,
            is_alive: true,
        }
    }

    pub async fn draw(&self) {
        match self.owner {
            Owner::Fighter => {
                draw_rectangle(self.location.x, self.location.y, 3., 3., WHITE);
            }
            Owner::EnemyFighter => {
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
}
