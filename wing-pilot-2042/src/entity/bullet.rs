use crate::entity::owner::Owner;
use macroquad::prelude::{Vec2, WHITE};
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
        draw_rectangle(self.location.x, self.location.y, 3., 3., WHITE);
    }
}
