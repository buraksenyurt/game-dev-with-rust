use crate::components::*;
use crate::constants::*;
use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
pub fn spawn_towers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let w_height = window.resolution.height();
        let w_width = window.resolution.width();
        let positions = [
            Vec2::new(-w_width / 2. + TOWER_WIDTH, w_height / 2. - TOWER_HEIGHT), // Upper Left Cornet
            Vec2::new(w_width / 2. - TOWER_WIDTH, w_height / 2. - TOWER_HEIGHT), // Upper Right Corner
            Vec2::new(0., 0.),                                                   // Center
            Vec2::new(-w_width / 2. + TOWER_WIDTH, -w_height / 2. + TOWER_HEIGHT), // Down Left Cornet
            Vec2::new(w_width / 2. - TOWER_WIDTH, -w_height / 2. + TOWER_HEIGHT), // Down Right Corner
        ];

        for position in positions.iter() {
            let shape = Mesh::from(Rectangle::new(TOWER_WIDTH, TOWER_HEIGHT));
            let color = ColorMaterial::from(TOWER_COLOR);

            let mesh_handle = meshes.add(shape);
            let material_handle = materials.add(color);

            commands.spawn((
                TowerBundle::new(position.x, position.y),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let w_height = window.resolution.height();
        let w_width = window.resolution.width();

        let shape = Mesh::from(RegularPolygon::new(TRIANGLE_CIRCUS_RADIUS, 3));
        let color = ColorMaterial::from(TRIANGLE_COLOR);

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        let initial_position = Vec2::new(w_width * 0.25, w_height * 0.25);

        commands
            .spawn((
                PlayerBundle::new(initial_position.x, initial_position.y),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform {
                        translation: Vec3::new(initial_position.x, initial_position.y, 0.0),
                        rotation: Quat::from_rotation_z(0.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(20.0, 1.0))).into(),
                    material: materials.add(ColorMaterial::from(TRIANGLE_COLOR)),
                    transform: Transform {
                        translation: Vec3::new(0.0, TRIANGLE_CIRCUS_RADIUS, 0.0),
                        rotation: Quat::from_rotation_z(PI / 2.0),
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

pub fn handle_player_rotations(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut crate::components::Direction), With<Player>>,
) {
    for (mut transform, mut direction) in &mut player_query {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotate_z(-ROTATION_ANGLE);
            let rotation = Quat::from_rotation_z(-ROTATION_ANGLE);
            let new_direction = rotation * direction.0.extend(0.0);
            direction.0 = new_direction.truncate();
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotate_z(ROTATION_ANGLE);
            let rotation = Quat::from_rotation_z(ROTATION_ANGLE);
            let new_direction = rotation * direction.0.extend(0.0);
            direction.0 = new_direction.truncate();
        }
    }
}

pub fn move_forward_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut crate::components::Direction,
        ),
        With<Player>,
    >,
    timer: Res<Time>,
) {
    for (mut transform, mut velocity, mut direction) in &mut player_query {
        if keyboard_input.pressed(KeyCode::Space) {
            let direction_vector = transform.rotation.mul_vec3(Vec3::Y).truncate();
            direction.0 = direction_vector.normalize();
            velocity.0 = direction.0 * TRIANGLE_SPEED;
        } else {
            velocity.0 = Vec2::ZERO;
        }

        let translation = velocity.0 * timer.delta_seconds();
        transform.translation += Vec3::new(translation.x, translation.y, 0.0);
    }
}

pub fn board_check(
    mut player_query: Query<(&mut Transform, &mut Position, &mut Velocity), With<Player>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let half_width = window.width() * 0.5;
        let half_height = window.height() * 0.5;

        if let Ok((mut transform, mut position, mut velocity)) = player_query.get_single_mut() {
            let mut new_translation = transform.translation;
            if new_translation.x <= -half_width {
                new_translation.x = -half_width;
                velocity.0 = Vec2::ZERO;
            } else if new_translation.x >= half_width {
                new_translation.x = half_width;
                velocity.0 = Vec2::ZERO;
            }

            if new_translation.y <= -half_height {
                new_translation.y = -half_height;
                velocity.0 = Vec2::ZERO;
            } else if new_translation.y >= half_height {
                new_translation.y = half_height;
                velocity.0 = Vec2::ZERO;
            }

            transform.translation = new_translation;
            position.0 = transform.translation.truncate();
        }
    }
}

pub fn wrap_around(
    mut player_query: Query<(&mut Transform, &mut Position), With<Player>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let half_width = window.width() * 0.5;
        let half_height = window.height() * 0.5;

        if let Ok((mut transform, mut position)) = player_query.get_single_mut() {
            let mut new_translation = transform.translation;

            if new_translation.x < -half_width {
                new_translation.x = half_width;
            } else if new_translation.x > half_width {
                new_translation.x = -half_width;
            }

            if new_translation.y < -half_height {
                new_translation.y = half_height;
            } else if new_translation.y > half_height {
                new_translation.y = -half_height;
            }

            transform.translation = new_translation;
            position.0 = transform.translation.truncate();
        }
    }
}
