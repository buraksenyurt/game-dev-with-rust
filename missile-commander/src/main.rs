use macroquad::prelude::{
    clear_background, draw_rectangle, next_frame, screen_width, Color, Vec2, WHITE,
};
use rand::prelude::*;

#[macroquad::main("Missile Command")]
async fn main() {
    let mut rng = thread_rng();
    let x = rng.gen_range((0. + screen_width() * 0.25)..(screen_width() - screen_width() * 0.25));

    let mut missile = Missile {
        position: Vec2::new(x, 0.),
    };

    loop {
        clear_background(Color::default());
        missile.position += Vec2::new(0., 1.);

        draw_rectangle(missile.position.x, missile.position.y, 4., 4., WHITE);

        next_frame().await
    }
}

struct Missile {
    pub position: Vec2,
}
