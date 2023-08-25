use super::components::Spaceship;
use super::*;
use crate::game::resources::GameState;
use crate::meteor::components::Meteor;
use bevy::window::PrimaryWindow;

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
pub fn fuel_tick_counter_system(mut fuel_timer: ResMut<FuelCheckTimer>, time: Res<Time>) {
    fuel_timer.timer.tick(time.delta());
}
pub fn decrease_spaceship_fuel_system(
    fuel_timer: Res<FuelCheckTimer>,
    mut game_state: ResMut<GameState>,
) {
    if fuel_timer.timer.finished() {
        if game_state.spaceship_fuel_level < 10 {
            info!("Yakıt bitti. Oyun sona ermeli.")
        }
        game_state.spaceship_fuel_level -= 10;
        info!(
            "Güncel yakıt seviyesi...{} galon.",
            game_state.spaceship_fuel_level
        );
    }
}
