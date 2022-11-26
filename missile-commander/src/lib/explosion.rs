use crate::{EXPLOSION_LIFE_TIME, EXPLOSION_THICKNESS};
use macroquad::prelude::{Vec2, SKYBLUE};
use macroquad::shapes::draw_circle_lines;

pub struct Explosion {
    location: Vec2,
    pub life_time: usize,
    pub radius: f32,
    pub is_alive: bool,
}

impl Explosion {
    pub fn spawn(location: Vec2) -> Self {
        Self {
            location,
            life_time: EXPLOSION_LIFE_TIME,
            radius: 0.,
            is_alive: true,
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(
            self.location.x,
            self.location.y,
            self.radius,
            EXPLOSION_THICKNESS,
            SKYBLUE,
        );
    }
}
