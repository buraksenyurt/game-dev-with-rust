use crate::constant::ROBOT_CIRCLE_RADIUS;
use crate::KeyCode;
use macroquad::color::GOLD;
use macroquad::input::is_key_down;
use macroquad::math::Circle;
use macroquad::shapes::draw_circle;
use macroquad::text::{draw_text_ex, TextParams};
use std::f32::consts::PI;

pub struct Robot {
    pub circle: Circle,
    state: String,
}

impl Robot {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            circle: Circle::new(x, y, ROBOT_CIRCLE_RADIUS),
            state: String::new(),
        }
    }

    pub fn update(&mut self) {
        let radian = ((self.circle.x as f32) / 180.) * PI;
        let new_y = radian.sin();
        if is_key_down(KeyCode::Right) {
            self.circle.x += 1.;
            self.circle.y += new_y;
        } else if is_key_down(KeyCode::Left) {
            self.circle.x -= 1.;
            self.circle.y -= new_y;
        }
        self.state = format!(
            "x = {}, radyan = {}, sin x= {}",
            self.circle.x, radian, new_y
        );
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, GOLD);

        draw_text_ex(
            &self.state,
            0.,
            20.,
            TextParams {
                font_size: 24,
                color: GOLD,
                ..Default::default()
            },
        );
    }
}
