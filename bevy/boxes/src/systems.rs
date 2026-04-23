use crate::globals::TILE_SIZE;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(8. * TILE_SIZE, 8. * TILE_SIZE, 999.9),
    ));
}
