use crate::constants::*;
use crate::ui::draw_vertical_center_text;
use crate::utility::hex_to_color;
use sdl2::render::WindowCanvas;

pub struct MainMenu;

impl MainMenu {
    pub fn draw(canvas: &mut WindowCanvas) -> Result<(), String> {
        let info = String::from("Lunar Landing 2049");
        draw_vertical_center_text(canvas, info, 48, hex_to_color(ROCKET_RED), 100)?;
        let info = String::from("Start Game (ENTER)");
        draw_vertical_center_text(canvas, info, 36, hex_to_color(ROCKET_RED), 200)?;
        let info = String::from("Developed with Rust and SDL2");
        draw_vertical_center_text(canvas, info, 16, hex_to_color(ROCKET_RED), 300)?;
        let info = String::from("BSŞ - 2024");
        draw_vertical_center_text(canvas, info, 16, hex_to_color(ROCKET_RED), 320)?;
        Ok(())
    }
}
