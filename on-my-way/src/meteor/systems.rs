use super::components::*;
use super::resources::*;
use super::*;
use crate::game::resources::GameState;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

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
    mut level_units: ResMut<GameState>,
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
    mut level_units: ResMut<GameState>,
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
    mut level_units: ResMut<GameState>,
) {
    if meteor_timer.timer.finished() && level_units.current_meteor_count <= MAX_METEOR_COUNT {
        let window = window_query.get_single().unwrap();
        let y = random::<f32>() * window.height();
        let x = window.width() + random::<f32>() * window.width();
        spawn_meteor(&mut commands, asset_server, y, x);
        level_units.current_meteor_count += 1;
    }
}
