use crate::constants::{DEFAULT_FUEL_LEVEL, ROCKET_RED};
use crate::entity::GameState;
use crate::game::Game;
use crate::ui::draw_vertical_center_text;
use crate::utility::hex_to_color;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct GameOverMenu;

impl GameOverMenu {
    pub fn draw(game: &Game, canvas: &mut WindowCanvas) -> Result<(), String> {
        let info = game.state.to_string();
        match game.state {
            GameState::Playing => {}
            GameState::OutOfFuel => {
                draw_vertical_center_text(canvas, info, 48, Color::RED, 200)?;
            }
            GameState::MeteorHit => {
                draw_vertical_center_text(canvas, info, 48, Color::RED, 200)?;
                draw_vertical_center_text(
                    canvas,
                    format!("Fuel Level is {}", game.shuttle.fuel_level),
                    24,
                    Color::RED,
                    300,
                )?;
            }
            GameState::JobsDone => {
                let point = DEFAULT_FUEL_LEVEL - game.shuttle.fuel_level;
                draw_vertical_center_text(canvas, info, 48, hex_to_color(ROCKET_RED), 200)?;
                draw_vertical_center_text(
                    canvas,
                    format!("Fuel Level is {}", game.shuttle.fuel_level),
                    24,
                    Color::GREEN,
                    300,
                )?;
                draw_vertical_center_text(
                    canvas,
                    format!("Total Point is {}", point),
                    24,
                    Color::GREEN,
                    350,
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}
