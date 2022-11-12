use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

#[macroquad::main("Missile Command")]
async fn main() {
    let mut missile = Missile::produce();
    println!("{}", missile);
    loop {
        clear_background(Color::default());
        missile.position += missile.direction;

        draw_rectangle(missile.position.x, missile.position.y, 4., 4., WHITE);

        next_frame().await
    }
}

struct Missile {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Missile {
    pub fn produce() -> Self {
        let x = rand::gen_range(
            screen_width() * 0.25,
            screen_width() - screen_width() * 0.25,
        );
        let left_angle = (x / screen_height()).atan();
        let right_angle = ((screen_width() - x) / screen_height()).atan();
        let pos_neg = rand::gen_range(0, 5);
        let v = match pos_neg {
            0 => 1.,
            _ => -1.,
        };
        let angle: f32 = rand::gen_range(left_angle, right_angle) * v;

        Self {
            position: Vec2::new(x, 0.),
            direction: Vec2::new(angle.cos(), 1.),
        }
    }
}

impl Display for Missile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos: {}, Dir: {}", self.position, self.direction)
    }
}
