use crate::entity::GameState;
use crate::ui::draw_center_text;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct GameOverMenu;

impl GameOverMenu {
    pub fn draw(state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let info = state.to_string();
        match state {
            GameState::Playing => {}
            GameState::OutOfFuel => {
                draw_center_text(canvas, info, 48, Color::RGBA(255, 0, 0, 255))?;
            }
            GameState::MeteorHit => {
                draw_center_text(canvas, info, 48, Color::RGBA(125, 125, 125, 255))?;
            }
            GameState::JobsDone => {
                draw_center_text(canvas, info, 48, Color::RGBA(0, 255, 0, 255))?;
            }
            _ => {}
        }
        Ok(())
    }
}
