mod game_over_menu;
pub mod hud;
pub mod main_menu;

pub use game_over_menu::GameOverMenu;
pub use main_menu::MainMenu;

use crate::constants::WIDTH;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

fn draw_text(
    canvas: &mut Canvas<Window>,
    text: &str,
    font_size: u16,
    color: Color,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("fonts/OpenSans-Bold.ttf", font_size)?;
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();
    canvas.copy(&texture, None, Some(Rect::new(x, y, width, height)))?;
    Ok(())
}
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
    let x = (WIDTH - text_size.0 as i32) / 2;

    let TextureQuery { width, height, .. } = texture.query();
    canvas.copy(&texture, None, Some(Rect::new(x, y, width, height)))?;
    Ok(())
}
