use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct GameOverMenu;

impl GameOverMenu {
    pub fn draw(canvas: &mut WindowCanvas, point: u64, total_time: f32) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let info = String::from("Game Over !!!");
        draw_vertical_center_text(canvas, info, 36, Color::WHITE, 180)?;
        let score = format!("SCORE {point}");
        draw_vertical_center_text(canvas, score, 24, Color::WHITE, 240)?;
        let duration = format!("TIME {total_time:#.2} secs");
        draw_vertical_center_text(canvas, duration, 24, Color::WHITE, 270)?;

        Ok(())
    }
}
