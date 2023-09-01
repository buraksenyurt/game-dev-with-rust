use crate::utility::Location;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub value: Location,
}
#[derive(Component)]
pub struct Tile;
