use crate::constant::MENU_SEPERATOR_HEIGHT;
use macroquad::prelude::*;

pub fn draw_menu(titles: Vec<&str>) {
    for (i, m) in (0_u32..).zip(titles.into_iter()) {
        let dimensions = measure_text(m, None, 48, 1.);
        draw_text_ex(
            m,
            screen_width() * 0.5 - dimensions.width * 0.5,
            (screen_height() * 0.25 - dimensions.height * 0.5)
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
