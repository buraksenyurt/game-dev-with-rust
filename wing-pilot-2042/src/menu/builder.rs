use crate::common::constants::INFO_BAR_MARGIN;
use crate::game::game::Game;
use macroquad::prelude::{draw_text, measure_text, GOLD};

pub async fn draw_info_bar(game: &Game) {
    let info = format!(
        "Bullets {} Enemy Fighters {}",
        game.fighter_amount_count,
        game.enemy_fleet.enemies.len()
    );
    let size = measure_text(info.as_str(), None, 24, 1.);
    draw_text(
        info.as_str(),
        INFO_BAR_MARGIN,
        size.height + INFO_BAR_MARGIN,
        24.,
        GOLD,
    );
}
