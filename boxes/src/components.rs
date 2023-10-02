use crate::board::components::Position;
use crate::globals::TILE_SIZE;
use bevy::prelude::Vec3;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}
impl Vector {
    pub const UP: Vector = Vector { x: 0, y: 1 };
    pub const DOWN: Vector = Vector { x: 0, y: -1 };
    pub const LEFT: Vector = Vector { x: -1, y: 0 };
    pub const RIGHT: Vector = Vector { x: 1, y: 0 };
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

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
