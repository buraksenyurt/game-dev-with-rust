use crate::ui::draw_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct MainMenu;

impl MainMenu {
    pub fn draw(canvas: &mut WindowCanvas) -> Result<(), String> {
        let info = String::from("Start Game (Enter)");
        draw_center_text(canvas, info, 48, Color::RGBA(0, 125, 255, 255))?;
        Ok(())
    }
}
