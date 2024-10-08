use crate::graphics::resources::TILE_SIZE;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4. * TILE_SIZE,
        4. * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
