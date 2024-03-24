pub mod main_menu;

pub use main_menu::MainMenu;

use crate::constants::SCREEN_WIDTH;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

fn draw_vertical_center_text(
    canvas: &mut Canvas<Window>,
    text: String,
    font_size: u16,
    color: Color,
    y: i32,
) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("fonts/OpenSans-Bold.ttf", font_size)?;
    let surface = font
        .render(&text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let text_size = font.size_of(&text).unwrap();
    let x = (SCREEN_WIDTH - text_size.0) / 2;

    let TextureQuery { width, height, .. } = texture.query();
    canvas.copy(&texture, None, Some(Rect::new(x as i32, y, width, height)))?;
    Ok(())
}
