use crate::assets_manager::{load_assets, AssetsResource};
use bevy::prelude::*;
use rand::RngExt;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -40.0..40.0;
const SPAWN_RANGE_Y: Range<f32> = -40.0..40.0;
const SPAWN_RANGE_Z: Range<f32> = -20.0..20.0;
const SCALE_FACTOR: f32 = 1.5;
const MIN_SPAWN_DISTANCE_FROM_SHUTTLE: f32 = 15.0;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet.after(load_assets));
    }
}

fn spawn_planet(mut commands: Commands, assets_resource: Res<AssetsResource>) {
    let mut rng = rand::rng();

    for p in assets_resource.planets.iter() {
        let translation = loop {
            let t = Vec3::new(
                rng.random_range(SPAWN_RANGE_X),
                rng.random_range(SPAWN_RANGE_Y),
                rng.random_range(SPAWN_RANGE_Z),
            );
            if Vec2::new(t.x, t.z).length() >= MIN_SPAWN_DISTANCE_FROM_SHUTTLE {
                break t;
            }
        };
        commands.spawn((
            SceneRoot(p.clone()),
            Transform {
                translation,
                scale: Vec3::splat(SCALE_FACTOR),
                ..default()
            },
        ));
    }
}
