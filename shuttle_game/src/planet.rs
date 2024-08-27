use crate::assets_manager::AssetsResource;
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Y: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Z: Range<f32> = -10.0..10.0;
const SCALE_FACTOR: f32 = 2.0;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet);
    }
}

#[derive(Bundle)]
struct PlanetBundle {
    model: SceneBundle,
}

fn spawn_planet(mut commands: Commands, assets_resource: Res<AssetsResource>) {
    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        rng.gen_range(SPAWN_RANGE_Y),
        rng.gen_range(SPAWN_RANGE_Z),
    );
    commands.spawn(PlanetBundle {
        model: SceneBundle {
            scene: assets_resource.planet.clone(),
            transform: Transform {
                translation,
                scale: Vec3::splat(SCALE_FACTOR),
                ..default()
            },
            ..default()
        },
    });
}
