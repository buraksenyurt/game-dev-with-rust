use crate::{Vec2, CITY_HEIGHT};
use macroquad::prelude::{mouse_position, DARKGREEN};
use macroquad::shapes::draw_line;
use macroquad::window::{screen_height, screen_width};

pub struct Turret {
    location: Vec2,
}

impl Turret {
    pub fn new() -> Self {
        Self {
            location: Vec2::new(screen_width() * 0.5, screen_height() - CITY_HEIGHT),
        }
    }

    pub fn draw(&self) {
        let m = mouse_position();
        let v = Vec2::new(m.0 - self.location.x, m.1 - self.location.y);
        let unit_v = v.normalize();
        draw_line(
            self.location.x,
            self.location.y,
            self.location.x + (unit_v.x * 30.),
            self.location.y + (unit_v.y * 30.),
            3.,
            DARKGREEN,
        )
    }
}
