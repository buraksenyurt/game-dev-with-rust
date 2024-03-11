use crate::constants::ROCKET_RED;
use crate::entity::GameState;
use crate::ui::draw_vertical_center_text;
use crate::utility::hex_to_color;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct GameOverMenu;

impl GameOverMenu {
    pub fn draw(state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let info = state.to_string();
        match state {
            GameState::Playing => {}
            GameState::OutOfFuel => {
                draw_vertical_center_text(canvas, info, 48, Color::RED, 200)?;
            }
            GameState::MeteorHit => {
                draw_vertical_center_text(canvas, info, 48, Color::RED, 200)?;
            }
            GameState::JobsDone => {
                draw_vertical_center_text(canvas, info, 48, hex_to_color(ROCKET_RED), 200)?;
            }
            _ => {}
        }
        Ok(())
    }
}
