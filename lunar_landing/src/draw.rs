use crate::constants::WIDTH;
use crate::entity::Shuttle;
use crate::Game;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery, WindowCanvas};
use sdl2::ttf;
use sdl2::video::Window;

pub fn draw_landing_platforms(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for p in &game.landing_platforms {
        p.draw(canvas)?;
    }

    Ok(())
}
pub fn draw_hud(shuttle: &Shuttle, canvas: &mut Canvas<Window>) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let v_point = shuttle.velocity.to_point();
    draw_text(
        canvas,
        &ttf_context,
        &format!("Fuel: {}", shuttle.fuel_level),
        14,
        Color::RGBA(255, 255, 255, 255),
        WIDTH - 100,
        10,
    )?;
    draw_text(
        canvas,
        &ttf_context,
        &format!(
            "({}:{})",
            shuttle.position.x + v_point.x,
            shuttle.position.y + v_point.y
        ),
        14,
        Color::RGBA(255, 255, 255, 255),
        WIDTH - 100,
        30,
    )?;

    Ok(())
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    ttf_context: &ttf::Sdl2TtfContext,
    text: &str,
    font_size: u16,
    color: Color,
    x: i32,
    y: i32,
) -> Result<(), String> {
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
