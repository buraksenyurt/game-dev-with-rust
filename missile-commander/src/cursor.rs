use crate::constant::CURSOR_LENGTH;
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
