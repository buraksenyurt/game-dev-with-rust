use crate::board::components::Position;
use crate::graphics::resources::TILE_SIZE;
use bevy::prelude::Vec3;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}
impl Location {
    pub const UP: Location = Location { x: 0, y: 1 };
    pub const DOWN: Location = Location { x: 0, y: -1 };
    pub const LEFT: Location = Location { x: -1, y: 0 };
    pub const RIGHT: Location = Location { x: 1, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.value.x as f32,
        TILE_SIZE * position.value.y as f32,
        z,
    )
}
