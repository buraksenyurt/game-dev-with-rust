use crate::Game;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery, WindowCanvas};
use sdl2::ttf;
use sdl2::video::Window;

pub fn draw_game_area(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for i in 0..game.mountain_points.len() - 1 {
        let start = game.mountain_points[i];
        let end = game.mountain_points[i + 1];
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(start, end)?;
    }

    Ok(())
}

pub fn draw_landing_platforms(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for p in &game.landing_platforms {
        p.draw(canvas)?;
    }

    Ok(())
}

pub fn draw_text(
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

// pub fn draw_strong_line(
//     canvas: &mut Canvas<Window>,
//     start: Point,
//     end: Point,
//     color: Color,
//     thickness: i32,
// ) -> Result<(), String> {
//     canvas.set_draw_color(color);
//     for i in 0..thickness {
//         let offset = i - thickness / 2;
//         canvas.draw_line(
//             Point::new(start.x + offset, start.y + offset),
//             Point::new(end.x + offset, end.y + offset),
//         )?;
//     }
//     Ok(())
// }
