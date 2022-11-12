use macroquad::prelude::*;

pub fn draw_base() {
    draw_line(
        0.,
        screen_height() - screen_height() * 0.20,
        screen_width(),
        screen_height() - screen_height() * 0.20,
        3.,
        GOLD,
    );
}
