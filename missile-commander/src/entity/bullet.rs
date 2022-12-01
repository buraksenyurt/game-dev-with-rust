use crate::BULLET_WIDTH;
use macroquad::input::mouse_position;
use macroquad::prelude::{Vec2, SKYBLUE};
use macroquad::shapes::draw_rectangle;

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub target: Vec2,
    pub is_alive: bool,
}

impl Bullet {
    pub fn spawn(location: Vec2) -> Self {
        let mp = mouse_position();
        let dv = Vec2::new(mp.0 - location.x, mp.1 - location.y);
        Self {
            position: location,
            target: Vec2::new(mp.0, mp.1),
            velocity: dv.normalize(),
            is_alive: true,
        }
    }

    pub fn draw(&self) {
        let p = self.position - BULLET_WIDTH * 0.5;
        draw_rectangle(p.x, p.y, BULLET_WIDTH, BULLET_WIDTH, SKYBLUE);
    }
}
