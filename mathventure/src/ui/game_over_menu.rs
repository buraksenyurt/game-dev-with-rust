use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct GameOverMenu;

impl GameOverMenu {
    pub fn draw(canvas: &mut WindowCanvas, point: u64) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let info = String::from("Game Over !!!");
        draw_vertical_center_text(canvas, info, 36, Color::WHITE, 180)?;
        let score = format!("SCORE {point}");
        draw_vertical_center_text(canvas, score, 24, Color::WHITE, 240)?;

        Ok(())
    }
}
