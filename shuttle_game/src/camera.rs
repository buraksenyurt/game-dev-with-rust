use bevy::prelude::*;

const DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Top-Down Perspective
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., DISTANCE, 0.).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
