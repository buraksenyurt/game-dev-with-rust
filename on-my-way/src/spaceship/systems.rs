use super::components::Spaceship;
use super::*;
use crate::events::GameOverEvent;
use crate::game::resources::GameState;
use crate::meteor::components::Meteor;
use crate::missile::components::Missile;
use crate::station::components::FuelStation;
use bevy::window::PrimaryWindow;

pub fn spawn_spaceship(
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

pub fn move_spaceship(
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

pub fn fire_missile(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(transform) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        0.,
                    ),
                    texture: asset_server.load("sprites/spaceMissiles_003.png"),
                    ..default()
                },
                Missile {
                    direction: Vec2::new(1., 0.),
                    speed: MISSILE_003_SPEED,
                    width: 51.,
                },
            ));
        }
    }
}

pub fn check_outside_of_the_bounds(
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

pub fn detect_collision_with_meteors(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    meteors_query: Query<(&Transform, &Meteor), With<Meteor>>,
    spaceship_query: Query<(Entity, &Transform), With<Spaceship>>,
) {
    if let Ok((spaceship, spaceship_transform)) = spaceship_query.get_single() {
        for (meteor_transform, meteor) in meteors_query.iter() {
            let distance = spaceship_transform
                .translation
                .distance(meteor_transform.translation);
            if distance < SPACESHIP_001_WIDTH / 2. + meteor.width / 2. {
                commands.entity(spaceship).despawn();
                game_over_event_writer.send(GameOverEvent { current_score: 1 });
                info!("Game Over!");
            }
        }
    }
}
pub fn count_fuel_tick(mut fuel_timer: ResMut<FuelCheckTimer>, time: Res<Time>) {
    fuel_timer.timer.tick(time.delta());
}
pub fn decrease_spaceship_fuel(
    mut commands: Commands,
    fuel_timer: Res<FuelCheckTimer>,
    mut game_state: ResMut<GameState>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    spaceship_query: Query<Entity, With<Spaceship>>,
) {
    if fuel_timer.timer.finished() {
        if game_state.spaceship_fuel_level < 10. {
            info!("Yakıt bitti. Oyun sona ermeli.");
            game_over_event_writer.send(GameOverEvent { current_score: 1 });
            if let Ok(spaceship) = spaceship_query.get_single() {
                commands.entity(spaceship).despawn();
            }
        } else {
            game_state.spaceship_fuel_level -= 10.;
            info!(
                "Güncel yakıt seviyesi...{} galon.",
                game_state.spaceship_fuel_level
            );
        }
    }
}

pub fn detect_connected_with_fuel_station(
    fuel_station_query: Query<(&Transform, &FuelStation), With<FuelStation>>,
    spaceship_query: Query<&Transform, With<Spaceship>>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok(spaceship_transform) = spaceship_query.get_single() {
        for (station_transform, station) in fuel_station_query.iter() {
            let distance = spaceship_transform
                .translation
                .distance(station_transform.translation);
            if distance <= SPACESHIP_001_WIDTH / 2. {
                info!("İstasyondan {} galon yakıt ekleniyor.", station.fuel_amount);
                game_state.spaceship_fuel_level += station.fuel_amount;
            }
        }
    }
}
