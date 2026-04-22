use crate::collision::Collider;
use crate::planner::GameSystemSet;
use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub scene: SceneRoot,
    pub transform: Transform,
    pub collider: Collider,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(GameSystemSet::EntityUpdates),
        );
    }
}

const MAX_DELTA_SECS: f32 = 1.0 / 20.0; // cap at 50ms to prevent first-frame spike

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta = time.delta_secs().min(MAX_DELTA_SECS);
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.value * delta;
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    let delta = time.delta_secs().min(MAX_DELTA_SECS);
    for (acceleration, mut vel) in query.iter_mut() {
        vel.value += acceleration.value * delta;
    }
}
