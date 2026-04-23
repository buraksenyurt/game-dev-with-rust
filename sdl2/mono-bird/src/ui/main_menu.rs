use crate::ui::draw_vertical_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct MainMenu;

impl MainMenu {
    pub fn draw(canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let info = String::from("Monochrome Bird");
        draw_vertical_center_text(canvas, info, 36, Color::GRAY, 100)?;
        let info = String::from("Start Game (ENTER)");
        draw_vertical_center_text(canvas, info, 24, Color::GRAY, 160)?;
        let info = String::from("Developed with Rust and SDL2");
        draw_vertical_center_text(canvas, info, 12, Color::GRAY, 250)?;
        let info = String::from("BSŞ - 2024");
        draw_vertical_center_text(canvas, info, 12, Color::GRAY, 370)?;

        Ok(())
    }
}
