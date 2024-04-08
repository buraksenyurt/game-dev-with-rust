mod conversation_box;
mod game_over_menu;
mod main_menu;

pub use conversation_box::ConversationBox;
pub use game_over_menu::GameOverMenu;
pub use main_menu::MainMenu;

use crate::resources::{LINE_SPACING, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

fn draw_vertical_center_text(
    canvas: &mut Canvas<Window>,
    text: String,
    font_size: u16,
    color: Color,
    y: i32,
) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("fonts/OpenSans-Bold.ttf", font_size)?;
    let texture_creator = canvas.texture_creator();
    let mut current_y = y;

    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        let line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        let (width, _) = font.size_of(&line).unwrap();
        if width > SCREEN_WIDTH || current_line.is_empty() {
            if !current_line.is_empty() {
                render_text_line(
                    &font,
                    &texture_creator,
                    canvas,
                    &current_line,
                    color,
                    current_y,
                )?;
                current_y += font_size as i32 + LINE_SPACING;
            }
            current_line = word.to_string();
        } else {
            current_line = line;
        }
    }

    if !current_line.is_empty() {
        render_text_line(
            &font,
            &texture_creator,
            canvas,
            &current_line,
            color,
            current_y,
        )?;
    }

    Ok(())
}

fn render_text_line(
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    canvas: &mut Canvas<Window>,
    text: &str,
    color: Color,
    y: i32,
) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let x = (SCREEN_WIDTH - width) / 2;
    canvas.copy(&texture, None, Some(Rect::new(x as i32, y, width, height)))?;
    Ok(())
}
