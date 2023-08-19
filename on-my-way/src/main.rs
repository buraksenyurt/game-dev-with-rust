use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const SPACESHIP_001_WIDTH: f32 = 80.;
pub const SPACESHIP_001_HEIGHT: f32 = 106.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera_system, spawn_spaceship_system))
        .run();
}

#[derive(Component)]
pub struct Spaceship {}

pub fn spawn_camera_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

pub fn spawn_spaceship_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(10. + SPACESHIP_001_WIDTH / 2., window.height() / 2., 0.),
            texture: asset_server.load("sprites/spaceShips_001.png"),
            ..default()
        },
        Spaceship {},
    ));
}
