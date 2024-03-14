use crate::ui::draw_vertical_center_text;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct MainMenu;

impl MainMenu {
    pub fn draw(canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let bg_texture = texture_creator.load_texture("assets/llbackground.png")?;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.copy(&bg_texture, None, Some(Rect::new(0, 0, 800, 600)))?;

        let info = String::from("Lunar Landing 2049");
        draw_vertical_center_text(canvas, info, 48, Color::GRAY, 300)?;
        let info = String::from("Start Game (ENTER)");
        draw_vertical_center_text(canvas, info, 36, Color::GRAY, 360)?;
        let info = String::from("Developed with Rust and SDL2");
        draw_vertical_center_text(canvas, info, 16, Color::GRAY, 550)?;
        let info = String::from("BSŞ - 2024");
        draw_vertical_center_text(canvas, info, 16, Color::GRAY, 570)?;

        Ok(())
    }
}
