use crate::components::Vector;
use bevy::prelude::KeyCode;

pub const WIN_WIDTH: f32 = 1280.;
pub const WIN_HEIGHT: f32 = 960.;
pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PART_Z: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;
pub const PART_SPEED: f32 = 15.;
pub const KEY_DIRECTION_MAP: [(KeyCode, Vector); 4] = [
    (KeyCode::Up, Vector::UP),
    (KeyCode::Down, Vector::DOWN),
    (KeyCode::Left, Vector::LEFT),
    (KeyCode::Right, Vector::RIGHT),
];
