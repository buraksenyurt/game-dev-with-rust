use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Hud;

impl Hud {
    pub fn draw(canvas: &mut WindowCanvas, point: u64, total_time: f32) -> Result<(), String> {
        let info = format!("{point} , {total_time:#.2}");
        draw_vertical_center_text(canvas, info, 16, Color::GRAY, 10)?;

        Ok(())
    }
}
