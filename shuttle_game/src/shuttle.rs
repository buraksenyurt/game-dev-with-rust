use crate::assets_manager::AssetsResource;
use crate::collision::Collider;
use crate::game_data::Score;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::out_off_boundary::Boundary;
use crate::planner::GameSystemSet;
use crate::state::GameState;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPEED: f32 = 25.0;
const ROCKET_SPEED: f32 = 30.;
const ROCKET_FORWARD_SPAWN: f32 = 2.5;
const ROTATION_SPEED: f32 = 2.5;
const ROLL_SPEED: f32 = 2.5;
const ROCKET_SCALE_FACTOR: f32 = 1. / 4.;
const SHUTTLE_RADIUS: f32 = 5.0;
const ROCKET_RADIUS: f32 = 1.;

#[derive(Component, Debug)]
pub struct Weapon;

#[derive(Component, Debug)]
pub struct Damage {
    pub value: i8,
}
impl Default for Damage {
    fn default() -> Self {
        Self { value: 10 }
    }
}
#[derive(Component)]
pub struct Health {
    pub value: i8,
}
impl Default for Health {
    fn default() -> Self {
        Self { value: 100 }
    }
}

#[derive(Component, Debug)]
pub struct Shuttle;
pub struct ShuttlePlugin;
impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_shuttle)
            .add_systems(
                Update,
                (
                    move_shuttle,
                    fire_at_will,
                    // log_shuttle_translation
                )
                    .chain()
                    .in_set(GameSystemSet::PlayerInput),
            )
            .add_systems(Update, check_health.in_set(GameSystemSet::EntityUpdates))
            .add_systems(
                OnEnter(GameState::GameOver),
                (despawn_shuttle, spawn_shuttle).chain(),
            );
    }
}
fn spawn_shuttle(
    mut commands: Commands,
    assets_resource: Res<AssetsResource>,
    mut score: ResMut<Score>,
) {
    score.total_hit = 0;
    score.player_damage = 100;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SHUTTLE_RADIUS),
            model: SceneBundle {
                scene: assets_resource.shuttle.clone(),
                transform: Transform::from_translation(INITIAL_POSITION),
                ..default()
            },
        },
        Shuttle,
        Damage::default(),
        Health::default(),
    ));
}

fn move_shuttle(
    mut query: Query<(&mut Transform, &mut Velocity), With<Shuttle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
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

fn fire_at_will(
    mut commands: Commands,
    query: Query<&Transform, With<Shuttle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    assets_resource: Res<AssetsResource>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * ROCKET_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(ROCKET_RADIUS),
                model: SceneBundle {
                    scene: assets_resource.rocket.clone(),
                    transform: Transform {
                        translation: transform.translation
                            + -transform.forward() * ROCKET_FORWARD_SPAWN,
                        scale: Vec3::splat(ROCKET_SCALE_FACTOR),
                        rotation: transform.rotation * Quat::from_rotation_y(-FRAC_PI_2),
                    },
                    ..default()
                },
            },
            Weapon,
            Boundary,
        ));
    }
}

fn check_health(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Health, With<Shuttle>>,
) {
    let Ok(health) = query.get_single() else {
        return;
    };
    if health.value == 0 {
        next_state.set(GameState::GameOver);
    }
}

fn despawn_shuttle(mut commands: Commands, query: Query<Entity, With<Shuttle>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

// fn log_shuttle_translation(query: Query<(Entity, &Transform), With<Shuttle>>) {
//     for (entity, transform) in query.iter() {
//         info!("Entity {:?} at {:?}", entity, transform.translation);
//     }
// }
