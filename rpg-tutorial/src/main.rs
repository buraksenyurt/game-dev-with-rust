use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const RESOLUTION: f32 = 16. / 9.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "RPG Tutorial".to_string(),
                width: 1600.,
                height: 900.,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            top: 1.,
            bottom: -1.,
            right: 1. * RESOLUTION,
            left: -1. * RESOLUTION,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
}
