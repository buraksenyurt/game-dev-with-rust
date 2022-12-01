use crate::lib::score::Score;
use crate::{Bullet, Explosion, GameState, Missile, MAX_CITY_HEALTH};
use macroquad::prelude::*;

pub struct Game {
    pub city_health: i32,
    pub score: Score,
    pub state: GameState,
    pub missiles: Vec<Missile>,
    pub bullets: Vec<Bullet>,
    pub explosions: Vec<Explosion>,
    pub current_stage: usize,
}

impl Game {
    pub fn new() -> Self {
        clear_background(BLACK);
        Game {
            city_health: MAX_CITY_HEALTH,
            score: Score::default(),
            state: GameState::Main,
            missiles: Vec::new(),
            bullets: Vec::new(),
            explosions: Vec::new(),
            current_stage: 0,
        }
    }
    pub fn draw(&self) {
        let text = format!("{}", self.score);
        let size = measure_text(text.as_str(), None, 20, 1.);
        draw_text(
            text.as_str(),
            screen_width() * 0.5 - size.width * 0.5,
            screen_height() - size.height + 10.,
            20.,
            RED,
        );
    }
}
