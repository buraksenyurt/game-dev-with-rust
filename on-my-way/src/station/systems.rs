use crate::game::resources::GameState;
use crate::station::components::FuelStation;
use crate::station::resources::FuelStationSpawnTimer;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

pub fn get_random_fuel_station() -> String {
    let stations = vec![
        "sprites/spaceStation_018.png",
        "sprites/spaceStation_019.png",
        "sprites/spaceStation_022.png",
    ];
    let idx = rand::thread_rng().gen_range(0..stations.len());
    info!("{idx} - {} kullanılacak", stations[idx]);
    stations[idx].to_string()
}
pub fn spawn_fuel_station(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let y = random::<f32>() * window.height();
    let x = window.width() + random::<f32>() * window.width();

    let fuel_amounts = vec![50, 100, 150];
    let fuel_idx = rand::thread_rng().gen_range(0..fuel_amounts.len());

    let speeds = vec![100., 125., 150.];
    let speed_idx = rand::thread_rng().gen_range(0..speeds.len());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load(get_random_fuel_station()),
            ..default()
        },
        FuelStation {
            direction: Vec2::new(-1., 0.),
            speed: speeds[speed_idx],
            fuel_amount: fuel_amounts[fuel_idx],
        },
    ));
}

pub fn move_fuel_station(mut query: Query<(&mut Transform, &FuelStation)>, time: Res<Time>) {
    for (mut transform, meteor) in query.iter_mut() {
        let direction = Vec3::new(meteor.direction.x, meteor.direction.y, 0.);
        transform.translation += direction * meteor.speed * time.delta_seconds();
    }
}

pub fn check_outside_of_the_bounds(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<FuelStation>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -100. {
            commands.entity(entity).despawn();
            info!("Yakıt istasyonu sınır dışına çıktı.");
        }
    }
}
pub fn count_fuel_station_spawn_tick(
    mut fuel_station_timer: ResMut<FuelStationSpawnTimer>,
    time: Res<Time>,
) {
    fuel_station_timer.timer.tick(time.delta());
}

pub fn spawn_after_time_finished(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    fuel_station_timer: Res<FuelStationSpawnTimer>,
    mut game_state: ResMut<GameState>,
) {
    if fuel_station_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let y = random::<f32>() * window.height();
        let x = window.width() + random::<f32>() * window.width();

        let fuel_amounts = vec![50, 100, 150];
        let fuel_idx = rand::thread_rng().gen_range(0..fuel_amounts.len());

        let speeds = vec![50., 75., 125.];
        let speed_idx = rand::thread_rng().gen_range(0..speeds.len());

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.),
                texture: asset_server.load(get_random_fuel_station()),
                ..default()
            },
            FuelStation {
                direction: Vec2::new(-1., 0.),
                speed: speeds[speed_idx],
                fuel_amount: fuel_amounts[fuel_idx],
            },
        ));

        game_state.spaceship_fuel_level += fuel_amounts[fuel_idx];
    }
}
