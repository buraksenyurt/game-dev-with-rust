use crate::MAX_CITY_HEALTH;
use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

pub struct Game {
    pub city_health: i32,
    pub player_hit: i32,
    pub player_point: i32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            city_health: MAX_CITY_HEALTH,
            player_hit: 0,
            player_point: 0,
        }
    }
    pub fn draw(&self) {
        let text = format!("{}", self);
        let size = measure_text(text.as_str(), None, 20, 1.);
        draw_text(
            text.as_str(),
            screen_width() * 0.5 - size.width * 0.5,
            screen_height() - size.height +10.,
            20.,
            RED,
        );
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "City Health {}, Hit {}, Point {}",
            self.city_health, self.player_hit, self.player_point
        )
    }
}