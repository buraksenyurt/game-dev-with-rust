use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf;
use sdl2::video::Window;

pub fn find_ground_height(points: &[Point], x: i32) -> i32 {
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if x >= p1.x && x <= p2.x {
            return interpolate_height(&[p1, p2], x);
        }
    }
    0
}

fn interpolate_height(points: &[Point], x: i32) -> i32 {
    if x <= points.first().unwrap().x {
        return points.first().unwrap().y;
    }
    if x >= points.last().unwrap().x {
        return points.last().unwrap().y;
    }

    for window in points.windows(2) {
        let (p1, p2) = (window[0], window[1]);
        if x >= p1.x && x <= p2.x {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            return p1.y + (x - p1.x) * dy / dx;
        }
    }
    points.first().unwrap().y
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
