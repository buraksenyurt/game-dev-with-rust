mod building;
mod constant;
mod cursor;
mod missile;

use crate::building::{create_buildings, draw_buildings};
use crate::constant::{
    MAX_MISSILE_COUNT, MISSILE_LENGTH, MISSILE_SPEED_FACTOR, WINDOW_HEIGHT, WINDOW_WITH,
};
use crate::cursor::draw_cursor;
use crate::missile::create_missiles;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Missile Command".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WITH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    rand::srand(miniquad::date::now() as _);

    let buildings = create_buildings();
    let mut missiles = create_missiles(MAX_MISSILE_COUNT);
    clear_background(Color::default());

    loop {
        draw_buildings(&buildings);
        draw_cursor();
        for m in missiles.iter_mut() {
            if m.lift_off_time == 0 {
                m.position += m.direction * MISSILE_SPEED_FACTOR;
                m.draw();

                if m.position.y > screen_height() - 100. {
                    m.is_alive = false;
                }
            } else {
                m.lift_off_time -= 1;
            }
        }

        missiles.retain(|m| m.is_alive);
        let mut new_missiles = create_missiles(MAX_MISSILE_COUNT - missiles.len() as i32);
        missiles.append(&mut new_missiles);
        next_frame().await
    }
}
