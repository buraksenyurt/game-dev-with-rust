use crate::assets_manager::AssetsResource;
use crate::collision::{Collider, CollisionEvent};
use crate::game_data::Score;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::out_off_boundary::Boundary;
use crate::planner::GameSystemSet;
use crate::shuttle::{Damage, Shuttle, Weapon};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = -25.0..25.0;
const VELOCITY_SCALE: f32 = 2.5;
const ACCELERATION_SCALE: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 2.0;
const SCALE_FACTOR: f32 = 3.0;
const RADIUS: f32 = 2.5;
const PLAYER_HIT: u8 = 1;

#[derive(Component, Debug)]
pub struct PickupCrate;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (
                spawn_crate.in_set(GameSystemSet::EntityUpdates),
                hitting_check.in_set(GameSystemSet::CollisionDetections),
            ),
        );
    }
}

fn spawn_crate(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets_resource: Res<AssetsResource>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALE;
    let acceleration = random_unit_vector() * ACCELERATION_SCALE;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            collider: Collider::new(RADIUS),
            model: SceneBundle {
                scene: assets_resource.pickup_crate.clone(),
                transform: Transform {
                    translation,
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..default()
                },
                ..default()
            },
        },
        PickupCrate,
        Boundary,
    ));
}

fn hitting_check(
    mut commands: Commands,
    pickup_query: Query<(Entity, &Collider), With<PickupCrate>>,
    shuttle_query: Query<Entity, With<Shuttle>>,
    weapon_query: Query<Entity, With<Weapon>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    for (entity, collider) in pickup_query.iter() {
        for &collided_entity in collider.entities.iter() {
            if shuttle_query.get(collided_entity).is_ok() {
                collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
                commands.entity(entity).despawn();
            } else if weapon_query.get(collided_entity).is_ok() {
                score.total_hit += PLAYER_HIT;
                commands.entity(entity).despawn();
            }
        }
    }
}
