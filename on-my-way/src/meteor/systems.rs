use super::components::*;
use super::resources::*;
use super::*;
use crate::game::resources::GameState;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

pub struct MeteorValue {
    pub x: f32,
    pub y: f32,
    pub asset_file: String,
    pub speed: f32,
    pub width: f32,
    pub required_hit_count: u8,
}
pub fn get_random_meteor(window_query: Query<&Window, With<PrimaryWindow>>) -> MeteorValue {
    let meteors = vec![
        "sprites/spaceMeteors_001.png",
        "sprites/spaceMeteors_002.png",
        "sprites/spaceMeteors_003.png",
        "sprites/spaceMeteors_004.png",
    ];
    let idx = rand::thread_rng().gen_range(0..meteors.len());
    info!("{idx} - {} kullanılacak", meteors[idx]);
    let asset_file = meteors[idx].to_string();

    let window = window_query.get_single().unwrap();
    let y = random::<f32>() * window.height();
    let x = window.width() + random::<f32>() * window.width();

    let speeds = vec![50., 100., 150., 200., 250.];
    let idx = rand::thread_rng().gen_range(0..speeds.len());
    let speed = speeds[idx];

    let required_hit_counts = vec![1, 2, 3];
    let idx = rand::thread_rng().gen_range(0..required_hit_counts.len());
    let required_hit_count = required_hit_counts[idx];

    MeteorValue {
        x,
        y,
        asset_file,
        speed,
        width: 40.,
        required_hit_count,
    }
}
pub fn spawn_meteors(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    spawn_one_meteor(&mut commands, window_query, asset_server);
    game_state.current_meteor_count += 1;
}

fn spawn_one_meteor(
    commands: &mut Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let meteor = get_random_meteor(window_query);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(meteor.x, meteor.y, 0.),
            texture: asset_server.load(meteor.asset_file),
            ..default()
        },
        Meteor {
            direction: Vec2::new(-1., 0.),
            speed: meteor.speed,
            width: meteor.width,
            current_hit_count: meteor.required_hit_count,
        },
    ));
}

pub fn move_meteors(mut query: Query<(&mut Transform, &Meteor)>, time: Res<Time>) {
    for (mut transform, meteor) in query.iter_mut() {
        let direction = Vec3::new(meteor.direction.x, meteor.direction.y, 0.);
        transform.translation += direction * meteor.speed * time.delta_seconds();
        transform.rotate_z(f32::to_radians(METEOR_ROTATE_DEGREE) * time.delta_seconds());
    }
}

pub fn check_outside_of_the_bounds(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Meteor>>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -50. {
            commands.entity(entity).despawn();
            game_state.current_meteor_count -= 1;
            game_state.missing_meteors_count += 1;
            info!(
                "Vurulamayan meteor sayısı {}",
                game_state.missing_meteors_count
            );
        }
    }
}

pub fn claim_hitted(
    mut commands: Commands,
    mut query: Query<(Entity, &Meteor)>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, meteor) in query.iter_mut() {
        if meteor.current_hit_count == 0 {
            commands.entity(entity).despawn();
            game_state.current_meteor_count -= 1;
        }
    }
}

pub fn count_meteor_spawn_tick(mut meteor_timer: ResMut<MeteorSpawnTimer>, time: Res<Time>) {
    meteor_timer.timer.tick(time.delta());
}

pub fn spawn_after_time_finished(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteor_timer: Res<MeteorSpawnTimer>,
    mut game_state: ResMut<GameState>,
) {
    if meteor_timer.timer.finished() && game_state.current_meteor_count <= MAX_METEOR_COUNT {
        spawn_one_meteor(&mut commands, window_query, asset_server);
        game_state.current_meteor_count += 1;
    }
}
