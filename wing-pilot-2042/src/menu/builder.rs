use crate::common::constants::INFO_BAR_MARGIN;
use crate::game::game::Game;
use macroquad::prelude::{draw_text, measure_text, screen_height, screen_width, WHITE};

pub fn draw_info_bar(game: &Game) {
    let info = format!("Bullets {}", game.fighter_amount_count);
    let size = measure_text(info.as_str(), None, 32, 1.);
    draw_text(
        info.as_str(),
        INFO_BAR_MARGIN,
        size.height + INFO_BAR_MARGIN,
        32.,
        WHITE,
    );
}
