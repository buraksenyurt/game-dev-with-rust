use crate::constant::MENU_SEPERATOR_HEIGHT;
use macroquad::prelude::*;

pub fn draw_menu() {
    let menus = ["Press Space to Start", "Press ESC to Exit"];
    for (i, m) in (0_u32..).zip(menus.into_iter()) {
        let dimensions = measure_text(m, None, 48, 1.);
        draw_text_ex(
            m,
            screen_width() * 0.5 - dimensions.width * 0.5,
            (screen_height() * 0.5 - dimensions.height * 0.5)
                + (i as f32 * dimensions.height)
                + (i as f32 * MENU_SEPERATOR_HEIGHT),
            TextParams {
                font_size: 48,
                color: GREEN,
                ..Default::default()
            },
        );
    }
}
