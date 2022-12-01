use crate::game::score::Score;
use crate::{Bullet, Explosion, Missile, State};
use macroquad::prelude::*;

pub struct Game {
    pub score: Score,
    pub state: State,
    pub missiles: Vec<Missile>,
    pub bullets: Vec<Bullet>,
    pub explosions: Vec<Explosion>,
    pub current_stage: usize,
}

impl Game {
    pub fn new(current_stage: usize) -> Self {
        clear_background(BLACK);
        Game {
            score: Score::default(),
            state: State::Main,
            missiles: Vec::new(),
            bullets: Vec::new(),
            explosions: Vec::new(),
            current_stage,
        }
    }
    pub fn draw(&self) {
        let stage_name = match self.current_stage {
            0 => "Rookie",
            1 => "Specialist",
            2 => "Veteran",
            _ => "",
        };
        let text = format!("{} STG {}", self.score, stage_name);
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
