use crate::components::Vector;
use bevy::prelude::KeyCode;

pub const WIN_WIDTH: u32 = 1280;
pub const WIN_HEIGHT: u32 = 960;
pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;
pub const PART_SPEED: f32 = 15.;
pub const KEY_DIRECTION_MAP: [(KeyCode, Vector); 4] = [
    (KeyCode::ArrowUp, Vector::UP),
    (KeyCode::ArrowDown, Vector::DOWN),
    (KeyCode::ArrowLeft, Vector::LEFT),
    (KeyCode::ArrowRight, Vector::RIGHT),
];
