use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const SPACESHIP_001_WIDTH: f32 = 80.;
pub const SPACESHIP_001_HEIGHT: f32 = 106.;
pub const SPACESHIP_001_SPEED: f32 = 500.;
pub const STANDARD_METEOR_SPEED: f32 = 100.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                spawn_camera_system,
                spawn_spaceship_system,
                spawn_meteor_system,
            ),
        )
        .add_systems(
            Update,
            (
                spaceship_movement_system,
                spaceship_border_check_system,
                meteor_movement_system,
                meteor_outside_of_the_bounds_system
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Spaceship {}

#[derive(Component)]
pub struct Meteor {
    pub direction: Vec2,
}

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
            transform: Transform::from_xyz(
                10. + SPACESHIP_001_WIDTH / 2.,
                window.height() / 2.,
                0.,
            ),
            texture: asset_server.load("sprites/spaceShips_001.png"),
            ..default()
        },
        Spaceship {},
    ));
}

pub fn spaceship_movement_system(
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., -1., 0.);
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += direction * SPACESHIP_001_SPEED * time.delta_seconds();
    }
}

pub fn spaceship_border_check_system(
    mut query: Query<&mut Transform, With<Spaceship>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut spaceship_transform) = query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let (y_min, y_max) = (
            SPACESHIP_001_HEIGHT / 2.,
            window.height() - SPACESHIP_001_HEIGHT / 2.,
        );
        let mut translation = spaceship_transform.translation;

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        spaceship_transform.translation = translation;
    }
}

pub fn spawn_meteor_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let y = random::<f32>() * window.height();
    let x = window.width() + random::<f32>() * window.width();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/spaceMeteors_001.png"),
            ..default()
        },
        Meteor {
            direction: Vec2::new(-1., 0.),
        },
    ));
}

pub fn meteor_movement_system(mut query: Query<(&mut Transform, &Meteor)>, time: Res<Time>) {
    for (mut transform, meteor) in query.iter_mut() {
        let direction = Vec3::new(meteor.direction.x, meteor.direction.y, 0.);
        transform.translation += direction * STANDARD_METEOR_SPEED * time.delta_seconds();
    }
}

pub fn meteor_outside_of_the_bounds_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Meteor>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -50. {
            commands.entity(entity).despawn();
            info!("Meteor sınır dışına çıktı.");
        }
    }
}
