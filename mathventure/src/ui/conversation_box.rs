use crate::resources::{BLOCK_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, STANDARD_ROW_COUNT};
use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct ConversationBox;

impl ConversationBox {
    pub fn draw(canvas: &mut WindowCanvas, question: &str, live: i8) {
        let map_height = STANDARD_ROW_COUNT * BLOCK_HEIGHT;
        let y = (map_height + (SCREEN_WIDTH - map_height) / 2) - BLOCK_HEIGHT + 10;
        draw_vertical_center_text(canvas, question.to_string(), 24, Color::WHITE, y as i32)
            .expect("Question draw error");

        draw_vertical_center_text(
            canvas,
            format!("Kalan Can {}", live),
            24,
            Color::RED,
            SCREEN_HEIGHT as i32 - 50,
        )
        .expect("Question draw error");
    }
}
