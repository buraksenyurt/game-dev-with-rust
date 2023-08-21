use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

pub const SPACESHIP_001_WIDTH: f32 = 80.;
pub const SPACESHIP_001_HEIGHT: f32 = 106.;
pub const SPACESHIP_001_SPEED: f32 = 500.;
pub const METEOR_SPAWN_TIME: f32 = 5.;
pub const MAX_METEOR_COUNT: u8 = 5;
pub const METEOR_ROTATE_DEGREE: f32 = 30.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MeteorSpawnTimer>()
        .init_resource::<LevelUnits>()
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
                meteor_outside_of_the_bounds_system,
                meteor_spawn_tick_counter_system,
                meteor_spawn_after_timer_system,
                meteors_spaceship_collision_detection_system,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Spaceship {}

#[derive(Component)]
pub struct Meteor {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
}

#[derive(Resource)]
pub struct MeteorSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct LevelUnits {
    pub current_meteor_count: u8,
}

impl Default for MeteorSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(METEOR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
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

pub fn get_random_meteor() -> String {
    let meteors = vec![
        "sprites/spaceMeteors_001.png",
        "sprites/spaceMeteors_002.png",
        "sprites/spaceMeteors_003.png",
        "sprites/spaceMeteors_004.png",
    ];
    let idx = rand::thread_rng().gen_range(0..meteors.len());
    info!("{idx} - {} kullanılacak", meteors[idx]);
    meteors[idx].to_string()
}
pub fn spawn_meteor_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut level_units: ResMut<LevelUnits>,
) {
    let window = window_query.get_single().unwrap();
    let y = random::<f32>() * window.height();
    let x = window.width() + random::<f32>() * window.width();
    spawn_meteor(&mut commands, asset_server, y, x);
    level_units.current_meteor_count += 1;
}

fn spawn_meteor(commands: &mut Commands, asset_server: Res<AssetServer>, y: f32, x: f32) {
    let speeds = vec![50., 100., 150., 200., 250.];
    let idx = rand::thread_rng().gen_range(0..speeds.len());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load(get_random_meteor()),
            ..default()
        },
        Meteor {
            direction: Vec2::new(-1., 0.),
            speed: speeds[idx],
            width: 40.,
        },
    ));
}

pub fn meteor_movement_system(mut query: Query<(&mut Transform, &Meteor)>, time: Res<Time>) {
    for (mut transform, meteor) in query.iter_mut() {
        let direction = Vec3::new(meteor.direction.x, meteor.direction.y, 0.);
        transform.translation += direction * meteor.speed * time.delta_seconds();
        transform.rotate_z(f32::to_radians(METEOR_ROTATE_DEGREE) * time.delta_seconds());
    }
}

pub fn meteor_outside_of_the_bounds_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Meteor>>,
    mut level_units: ResMut<LevelUnits>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -50. {
            commands.entity(entity).despawn();
            level_units.current_meteor_count -= 1;
            info!("Meteor sınır dışına çıktı.");
        }
    }
}

pub fn meteor_spawn_tick_counter_system(
    mut meteor_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>,
) {
    meteor_timer.timer.tick(time.delta());
}

pub fn meteor_spawn_after_timer_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteor_timer: Res<MeteorSpawnTimer>,
    mut level_units: ResMut<LevelUnits>,
) {
    if meteor_timer.timer.finished() && level_units.current_meteor_count <= MAX_METEOR_COUNT {
        let window = window_query.get_single().unwrap();
        let y = random::<f32>() * window.height();
        let x = window.width() + random::<f32>() * window.width();
        spawn_meteor(&mut commands, asset_server, y, x);
        level_units.current_meteor_count += 1;
    }
}

pub fn meteors_spaceship_collision_detection_system(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform), With<Spaceship>>,
    meteors_query: Query<(&Transform, &Meteor), With<Meteor>>,
) {
    if let Ok((spaceship, spaceship_transform)) = spaceship_query.get_single() {
        for (meteor_transform, meteor) in meteors_query.iter() {
            let distance = spaceship_transform
                .translation
                .distance(meteor_transform.translation);
            if distance < SPACESHIP_001_WIDTH / 2. + meteor.width / 2. {
                commands.entity(spaceship).despawn();
                info!("Game Over!");
            }
        }
    }
}
