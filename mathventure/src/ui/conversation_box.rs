use crate::resources::{BLOCK_HEIGHT, SCREEN_WIDTH, STANDARD_ROW_COUNT};
use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct ConversationBox;

impl ConversationBox {
    pub fn draw(canvas: &mut WindowCanvas, question: String) {
        let map_height = STANDARD_ROW_COUNT * BLOCK_HEIGHT;
        let y = (map_height + (SCREEN_WIDTH - map_height) / 2) - BLOCK_HEIGHT + 10;
        draw_vertical_center_text(canvas, question, 24, Color::WHITE, y as i32)
            .expect("Question draw error");
    }
}