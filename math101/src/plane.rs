use crate::constant::ROBOT_CIRCLE_RADIUS;
use crate::{KeyCode, Vec2};
use macroquad::color::BROWN;
use macroquad::input::is_key_down;
use macroquad::math::Circle;
use macroquad::shapes::draw_circle;
use std::f32::consts::PI;

pub struct Plane {
    pub circle: Circle,
    pub angle: f32,
    pub start: Vec2,
    pub radius: f32,
}

impl Plane {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            circle: Circle::new(x, y, ROBOT_CIRCLE_RADIUS),
            angle: 0.,
            start: Vec2::new(x, y),
            radius,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Space) {
            let radian = ((self.angle as f32) / 180.) * PI;
            self.circle.x = self.start.x + (self.radius * self.angle.cos());
            self.circle.y = self.start.y + (self.radius * self.angle.sin());
            self.angle += 0.1;
            if radian > 2. * PI {
                self.angle = 0.;
            }

            println!(
                "Angle {} Radian {} x {} y {}",
                self.angle, radian, self.circle.x, self.circle.y
            );
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, BROWN);
    }
}
