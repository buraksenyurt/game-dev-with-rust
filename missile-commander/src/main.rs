mod constant;
mod missile;
mod painter;

use crate::constant::{MAX_MISSILE_COUNT, MISSILE_LENGTH, MISSILE_SPEED_FACTOR};
use crate::missile::Missile;
use crate::painter::{draw_base, draw_cursor};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Missile Command".to_owned(),
        fullscreen: false,
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(miniquad::date::now() as _);

    let mut missiles = Vec::new();
    for _ in 0..MAX_MISSILE_COUNT {
        let missile = Missile::produce();
        println!("{}", &missile);
        missiles.push(missile);
    }
    clear_background(Color::default());
    show_mouse(false);
    loop {
        draw_base();
        draw_cursor();
        for m in missiles.iter_mut() {
            m.position += m.direction * MISSILE_SPEED_FACTOR;
            m.draw();
        }

        next_frame().await
    }
}
