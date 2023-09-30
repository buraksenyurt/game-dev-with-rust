use crate::components::Vector;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub value: Vector,
}
#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Part {
    pub kind: String,
}
