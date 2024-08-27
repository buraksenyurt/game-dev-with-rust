use crate::assets_manager::AssetsResource;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::*;

const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPEED: f32 = 25.0;
const ROTATION_SPEED: f32 = 2.5;
const ROLL_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Shuttle;
pub struct ShuttlePlugin;
impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_shuttle)
            .add_systems(Update, move_shuttle);
    }
}
fn spawn_shuttle(mut commands: Commands, assets_resource: Res<AssetsResource>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: assets_resource.shuttle.clone(),
                transform: Transform::from_translation(INITIAL_POSITION),
                ..default()
            },
        },
        Shuttle,
    ));
}

fn move_shuttle(
    mut query: Query<(&mut Transform, &mut Velocity), With<Shuttle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        movement = -SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement = SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        roll = -ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        roll = ROLL_SPEED * time.delta_seconds();
    }
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
    velocity.value = -transform.forward() * movement;
}
