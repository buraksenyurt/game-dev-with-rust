use crate::assets_manager::AssetsResource;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::*;

const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct ShuttlePlugin;
impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_shuttle);
    }
}
fn spawn_shuttle(mut commands: Commands, assets_resource: Res<AssetsResource>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: assets_resource.shuttle.clone(),
            transform: Transform::from_translation(INITIAL_POSITION),
            ..default()
        },
    });
}
