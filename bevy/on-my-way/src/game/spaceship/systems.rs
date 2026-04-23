use super::components::Spaceship;
use super::*;
use crate::events::GameOverEvent;
use crate::game::live_data::resources::LiveData;
use crate::game::meteor::components::Meteor;
use crate::game::missile::components::Missile;
use crate::game::station::components::FuelStation;
use bevy::window::PrimaryWindow;

pub fn spawn_spaceship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/spaceShips_001.png"),
            ..default()
        },
        Transform::from_xyz(
            10. + SPACESHIP_001_WIDTH / 2.,
            window.height() / 2.,
            0.,
        ),
        Spaceship {},
    ));
}

pub fn despawn_spaceship(mut commands: Commands, query: Query<Entity, With<Spaceship>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}

pub fn move_spaceship(
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec3::new(0., -1., 0.);
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += direction * SPACESHIP_001_SPEED * time.delta_secs();
    }
}

pub fn fire_missile(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    launch_timer: Res<MissileLaunchCheckTimer>,
    mut live_data: ResMut<LiveData>,
) {
    if keyboard_input.pressed(KeyCode::KeyS) && launch_timer.timer.just_finished() {
        if let Ok(transform) = query.single_mut() {
            let missile = Missile {
                direction: Vec2::new(1., 0.),
                speed: MISSILE_003_SPEED,
                width: 51.,
                fuel_cost: 1.5,
                disposable: false,
                location: transform.translation,
            };
            commands.spawn((
                Sprite {
                    image: asset_server.load("sprites/spaceMissiles_003.png"),
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.,
                ),
                missile.clone(),
            ));
            live_data.spaceship_fuel_level -= missile.fuel_cost;
            live_data.used_missile_count += 1;
        }
    }
}

pub fn check_outside_of_the_bounds(
    mut query: Query<&mut Transform, With<Spaceship>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut spaceship_transform) = query.single_mut() {
        let window = window_query.single().unwrap();
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
    mut game_over_event_writer: MessageWriter<GameOverEvent>,
    meteors_query: Query<(&Transform, &Meteor), With<Meteor>>,
    spaceship_query: Query<(Entity, &Transform), With<Spaceship>>,
    asset_server: Res<AssetServer>,
    live_data: Res<LiveData>,
) {
    if let Ok((spaceship, spaceship_transform)) = spaceship_query.single() {
        for (meteor_transform, meteor) in meteors_query.iter() {
            let distance = spaceship_transform
                .translation
                .distance(meteor_transform.translation);
            if distance < SPACESHIP_001_WIDTH / 2. + meteor.width / 2. {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/explosionCrunch_000.ogg")));

                commands.entity(spaceship).despawn();
                game_over_event_writer.write(GameOverEvent {
                    current_score: live_data.exploded_meteors_count,
                });
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
    mut live_data: ResMut<LiveData>,
    mut game_over_event_writer: MessageWriter<GameOverEvent>,
    spaceship_query: Query<Entity, With<Spaceship>>,
) {
    if fuel_timer.timer.just_finished() {
        if live_data.spaceship_fuel_level < 10. {
            info!("Yakıt bitti. Oyun sona ermeli.");
            game_over_event_writer.write(GameOverEvent {
                current_score: live_data.exploded_meteors_count,
            });
            if let Ok(spaceship) = spaceship_query.single() {
                commands.entity(spaceship).despawn();
            }
        } else {
            live_data.spaceship_fuel_level -= 10.;
        }
    }
}

pub fn detect_connected_with_fuel_station(
    fuel_station_query: Query<(&Transform, &FuelStation), With<FuelStation>>,
    spaceship_query: Query<&Transform, With<Spaceship>>,
    mut live_data: ResMut<LiveData>,
) {
    if let Ok(spaceship_transform) = spaceship_query.single() {
        for (station_transform, station) in fuel_station_query.iter() {
            let distance = spaceship_transform
                .translation
                .distance(station_transform.translation);
            if distance <= SPACESHIP_001_WIDTH / 2. {
                info!("Istasyondan {} galon yakit ekleniyor.", station.fuel_amount);
                live_data.spaceship_fuel_level += station.fuel_amount;
            }
        }
    }
}
pub fn count_launch_time(mut launch_timer: ResMut<MissileLaunchCheckTimer>, time: Res<Time>) {
    launch_timer.timer.tick(time.delta());
}
