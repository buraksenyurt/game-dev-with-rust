use crate::globals::TILE_SIZE;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        8. * TILE_SIZE,
        8. * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
