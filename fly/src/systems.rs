use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite};
use bevy::utils::default;

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image: Handle<Image> = asset_server.load("Fox.png");
    let tile_image: Handle<Image> = asset_server.load("Tile.png");

    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::ONE * 48.0),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::new(-PLAYER_VELOCITY_X, 0.0, 0.0)),
    ));

    // Ground
    for i in 0..36 {
        commands.spawn((
            Sprite {
                image: tile_image.clone(),
                custom_size: Some(Vec2::ONE * 32.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(-600.0 + (i * 32) as f32, GROUND_LEVEL, 0.0),
        ));
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x -= velocity.0.x * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x += velocity.0.x * time.delta_secs();
        }
        // Jumping
        if keyboard_input.pressed(KeyCode::Space) && transform.translation.y <= GROUND_LEVEL {
            velocity.0.y = JUMP_FORCE;
        }
    }
}

pub fn apply_gravity_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

pub fn update_player_position_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}
