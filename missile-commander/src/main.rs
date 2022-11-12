mod constant;
mod missile;
mod painter;

use crate::constant::{MAX_MISSILE_COUNT, MISSILE_LENGTH, MISSILE_SPEED_FACTOR};
use crate::missile::Missile;
use crate::painter::draw_base;
use macroquad::prelude::*;

#[macroquad::main("Missile Command")]
async fn main() {
    rand::srand(miniquad::date::now() as _);

    let mut missiles = Vec::new();
    for _ in 0..MAX_MISSILE_COUNT {
        let missile = Missile::produce();
        println!("{}", &missile);
        missiles.push(missile);
    }
    clear_background(Color::default());

    loop {
        draw_base();

        for m in missiles.iter_mut() {
            m.position += m.direction * MISSILE_SPEED_FACTOR;
            m.draw();
        }

        next_frame().await
    }
}
