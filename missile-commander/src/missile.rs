use crate::constant::TRACE_TICKNESS;
use crate::MISSILE_LENGTH;
use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

pub struct Missile {
    start_position: Vec2,
    pub position: Vec2,
    pub direction: Vec2,
    pub angle: f32,
}

impl Missile {
    pub fn produce() -> Self {
        let x = rand::gen_range(
            screen_width() * 0.25,
            screen_width() - screen_width() * 0.25,
        );
        let left_angle = (x / screen_height()).atan();
        let right_angle = (screen_width() - x / screen_height()).atan();
        let pos_neg = rand::gen_range(0, 5);
        let sign = match pos_neg {
            0 => -1.,
            _ => 1.,
        };
        let angle: f32 = rand::gen_range(left_angle, right_angle);

        Self {
            start_position: Vec2::new(x, 0.),
            position: Vec2::new(x, 0.),
            direction: Vec2::new(sign * angle.cos(), 1.),
            angle,
        }
    }

    pub fn draw(&self) {
        draw_line(
            self.start_position.x,
            self.start_position.y,
            self.position.x,
            self.position.y,
            TRACE_TICKNESS,
            WHITE,
        );
        draw_rectangle(
            self.position.x,
            self.position.y,
            MISSILE_LENGTH,
            MISSILE_LENGTH,
            WHITE,
        );
    }
}

impl Display for Missile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pos: {}, Dir: {}, Angle:{} rad,{} deg",
            self.position,
            self.direction,
            self.angle,
            self.angle.to_degrees()
        )
    }
}
