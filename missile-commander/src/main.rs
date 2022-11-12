use macroquad::prelude::{
    clear_background, draw_rectangle, next_frame, screen_width, Color, Vec2, WHITE,
};
use macroquad::window::screen_height;
use rand::prelude::*;

#[macroquad::main("Missile Command")]
async fn main() {
    let rng = thread_rng();
    let mut missile = Missile::produce(rng);

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
    pub fn produce(mut rng: ThreadRng) -> Self {
        let x =
            rng.gen_range((0. + screen_width() * 0.25)..(screen_width() - screen_width() * 0.25));
        let left_angle = (x / screen_height()).atan();
        let right_angle = ((screen_width() - x) / screen_height()).atan();
        println!("x value {}", x);
        println!(
            "Left angle(max) - {} Radian \t Right angle(max) - {} Radian\nLeft angle(max) - {} Degree \t Right angle(max) - {} Degree",
            left_angle, right_angle,left_angle.to_degrees(),
            right_angle.to_degrees()
        );
        let pos_neg = rng.gen::<bool>();
        let v = match pos_neg {
            true => 1.,
            false => -1.,
        };
        let angle: f32 = rng.gen_range(left_angle..right_angle) * v;
        println!("Angle {}", angle);

        Self {
            position: Vec2::new(x, 0.),
            direction: Vec2::new(angle.sin(), 1.),
        }
    }
}
