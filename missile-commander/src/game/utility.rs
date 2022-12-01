use crate::{CURSOR_LENGTH, WINDOW_HEIGHT, WINDOW_WITH};
use macroquad::prelude::*;

pub fn draw_cursor() {
    let (x, y) = mouse_position();
    draw_line(x - CURSOR_LENGTH, y, x + CURSOR_LENGTH, y, 1., RED);
    draw_line(x, y - CURSOR_LENGTH, x, y + CURSOR_LENGTH, 1., RED);

    draw_text(
        format!("{:?}", mouse_position()).as_str(),
        0.,
        screen_height() - 5.,
        20.,
        RED,
    );
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Missile Command".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WITH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn get_max(v1: f32, v2: f32) -> f32 {
    if v1 > v2 {
        return v1;
    }
    v2
}

pub fn get_min(v1: f32, v2: f32) -> f32 {
    if v1 < v2 {
        return v1;
    }
    v2
}
