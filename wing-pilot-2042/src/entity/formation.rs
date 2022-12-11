use macroquad::prelude::{rand, Vec2};
use macroquad::window::screen_height;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Formation {
    pub velocity: Vec2,
    pub start_y: f32,
}

impl Formation {
    pub fn new(velocity: Vec2, start_y: f32) -> Self {
        Self { velocity, start_y }
    }
}

pub enum Direction {
    NorthEast,
    NortWest,
    West,
    East,
}

async fn get_random_direction() -> Direction {
    match rand::gen_range(0, 4) {
        0 => Direction::NorthEast,
        1 => Direction::NortWest,
        2 => Direction::West,
        _ => Direction::East,
    }
}

pub async fn get_formation() -> Formation {
    let d = get_random_direction().await;
    match d {
        Direction::NortWest => Formation::new(
            Vec2::new(-(PI / 4.).cos(), -(PI / 4.).sin()),
            screen_height() * 0.5,
        ),
        Direction::NorthEast => Formation::new(
            Vec2::new((PI / 4.).cos(), -(PI / 4.).sin()),
            screen_height() * 0.5,
        ),
        Direction::East => Formation::new(Vec2::new(-1., 0.), screen_height() * 0.40),
        Direction::West => Formation::new(Vec2::new(1., 0.), screen_height() * 0.25),
    }
}
