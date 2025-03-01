use crate::components::*;
use crate::constants::*;
use crate::game_play::Level;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite};
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let fox_texture_handle: Handle<Image> = asset_server.load("Fox/Idle.png");
    let frame_size = UVec2::new(32, 32);
    let fox_atlas_layout = TextureAtlasLayout::from_grid(frame_size, 11, 1, None, None);
    let fox_atlas_layout_handle = texture_atlases.add(fox_atlas_layout);

    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: fox_texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: fox_atlas_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::ONE * 32.0),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::new(-PLAYER_VELOCITY_X, 0.0, 0.0)),
        StandardAnimation::default(),
    ));

    load_level_tiles(Level::FirstGate, &mut commands, &asset_server);
}

fn load_level_tiles(_game_level: Level, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ground_tile_image: Handle<Image> = asset_server.load("Tile.png");
    let platform_image: Handle<Image> = asset_server.load("Platform.png");

    // Main Ground
    draw_ground(
        Vec3::new(-600.0, GROUND_LEVEL, 0.0),
        36,
        commands,
        &ground_tile_image,
    );

    // Jumping grounds
    draw_ground(Vec3::new(-600.0, 100.0, 0.0), 3, commands, &platform_image);
    draw_ground(Vec3::new(-500.0, 180.0, 0.0), 4, commands, &platform_image);
    draw_ground(Vec3::new(-360.0, 140.0, 0.0), 2, commands, &platform_image);
    draw_ground(Vec3::new(-280.0, 60.0, 0.0), 4, commands, &platform_image);
    draw_ground(Vec3::new(-100.0, 80.0, 0.0), 3, commands, &platform_image);
    draw_ground(Vec3::new(20.0, 150.0, 0.0), 5, commands, &platform_image);
    draw_ground(Vec3::new(160.0, 60.0, 0.0), 3, commands, &platform_image);
}

fn draw_ground(location: Vec3, count: usize, commands: &mut Commands, image: &Handle<Image>) {
    for i in 0..count {
        commands.spawn((
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::ONE * 24.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(location.x + (i * 24) as f32, location.y, location.z),
        ));
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x -= velocity.0.x * time.delta_secs();
        transform.scale.x = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.scale.x = -1.0;
    }
    // Jumping
    if keyboard_input.pressed(KeyCode::Space) && transform.translation.y <= GROUND_LEVEL {
        velocity.0.y = JUMP_FORCE;
    }
}

pub fn apply_gravity_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();
    velocity.0.y += GRAVITY * time.delta_secs();
}

pub fn update_player_position_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    transform.translation.y += velocity.0.y * time.delta_secs();

    if transform.translation.y <= GROUND_LEVEL {
        transform.translation.y = GROUND_LEVEL;
        velocity.0.y = 0.0;
    }
}

pub fn spawn_flying_boxes_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<BoxSpawningTimer>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let box_image: Handle<Image> = asset_server.load("Box.png");
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL + (rng.next_u64() % 75) as f32;
        let custom_size = Some(Vec2::ONE * (rng.next_u64() % 50) as f32);
        commands.spawn((
            Box,
            Sprite {
                image: box_image,
                custom_size,
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}

pub fn move_boxes_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Box>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn apply_animation(time: Res<Time>, mut query: Query<(&mut StandardAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 8;
            }
        }
    }
}
