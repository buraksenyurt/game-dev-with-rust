use crate::components::*;
use crate::constants::*;
use crate::game_play::Level;
use crate::level_manager::*;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite};
use bevy_rapier2d::prelude::*;
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::time::Duration;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let fox_idle_texture: Handle<Image> = asset_server.load("fox/Idle.png");
    let fox_running_texture: Handle<Image> = asset_server.load("fox/Run.png");
    let fox_jumping_texture: Handle<Image> = asset_server.load("fox/Jump.png");

    let barrel_idle_texture: Handle<Image> = asset_server.load("barrel/Idle.png");
    let barrel_walking_texture: Handle<Image> = asset_server.load("barrel/Walk.png");

    let fox_atlas = TextureAtlasLayout::from_grid(PLAYER_SIZE.as_uvec2(), 11, 1, None, None);
    let fox_atlas_handle = texture_atlases.add(fox_atlas);

    let barrel_atlas = TextureAtlasLayout::from_grid(BARREL_SIZE.as_uvec2(), 11, 1, None, None);
    let barrel_atlas_handle = texture_atlases.add(barrel_atlas);

    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: fox_idle_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: fox_atlas_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::ONE * PLAYER_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_SIZE.x / 2.1, PLAYER_SIZE.y / 2.1),
        GravityScale(GRAVITY_FORCE),
        Transform::from_xyz(PLAYER_START_X, 0.0, 0.0),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Min,
        },
        StandardAnimation::default(),
        PlayerState::Idle,
        PlayerAnimationHandles {
            idle: fox_idle_texture,
            running: fox_running_texture,
            jumping: fox_jumping_texture,
        },
    ));

    let mut rng = rand::rng();

    // Barrel
    commands.spawn((
        Barrel { power: 2.0 },
        BarrelDriver::default(),
        Sprite {
            image: barrel_idle_texture.clone(),
            anchor: Anchor::Center,
            texture_atlas: Some(TextureAtlas {
                layout: barrel_atlas_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::ONE * BARREL_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(BARREL_SIZE.x / 4.0, BARREL_SIZE.y / 2.0),
        GravityScale(GRAVITY_FORCE),
        Transform::from_translation(
            Vec2::new(
                rng.random_range(-500.0..500.0),
                rng.random_range(400.0..500.0),
            )
            .extend(0.0),
        ),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Min,
        },
        StandardAnimation::default(),
        BarrelState::Idle,
        BarrelAnimationHandles {
            idle: barrel_idle_texture,
            walking: barrel_walking_texture,
        },
    ));

    load_level(Level::FirstGate, &mut commands, &asset_server);
}

fn load_level(level: Level, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ground_tile_image: Handle<Image> = asset_server.load("Tile.png");
    let platform_image: Handle<Image> = asset_server.load("Platform.png");

    if let Some(platforms) = get_level_data(level) {
        for platform_data in platforms {
            let image = match platform_data.platform_type {
                PlatformType::Ground => ground_tile_image.clone(),
                PlatformType::Platform => platform_image.clone(),
                _ => continue,
            };

            spawn_platform(&platform_data, commands, &image);
        }
    }

    spawn_random_crates(8, commands, asset_server);
}

fn spawn_random_crates(count: usize, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let crate_image: Handle<Image> = asset_server.load("Crate.png");
    let mut rng = rand::rng();

    for i in 0..count {
        let location = Vec2::new(
            rng.random_range(-500.0..500.0),
            rng.random_range(400.0..500.0),
        );
        let crate_size = [22.0, 32.0, 36.0, 42.0].choose(&mut rng).unwrap();
        commands.spawn((
            Sprite {
                image: crate_image.clone(),
                custom_size: Some(Vec2::ONE * crate_size),
                ..default()
            },
            Transform::from_xyz(location.x + (i as f32 * crate_size) as f32, location.y, 0.0),
            GravityScale(BOX_GRAVITY_FORCE),
            RigidBody::Dynamic,
            Collider::cuboid(crate_size / 2.0, crate_size / 2.0),
            AdditionalMassProperties::Mass(50.0),
            Friction {
                coefficient: 3.5,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.2,
                combine_rule: CoefficientCombineRule::Average,
            },
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.4,
            },
            Ccd::enabled(),
            Platform {
                platform_type: PlatformType::Crate,
            },
        ));
    }
}

fn spawn_platform(platform_data: &PlatformData, commands: &mut Commands, image: &Handle<Image>) {
    for i in 0..platform_data.tile_count {
        let p_data = platform_data.clone();
        let transform = match p_data.direction {
            PlatformDirection::Horizontal => Vec2::new(
                p_data.position.x + (i * TILE_SIZE as usize) as f32,
                p_data.position.y,
            )
            .extend(0.0),
            PlatformDirection::Vertical => Vec2::new(
                p_data.position.x,
                p_data.position.y + (i * TILE_SIZE as usize) as f32,
            )
            .extend(0.0),
        };
        commands.spawn((
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::ONE * TILE_SIZE),
                ..default()
            },
            Transform::from_translation(transform),
            RigidBody::Fixed,
            Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            Platform {
                platform_type: p_data.platform_type,
            },
        ));
    }
}

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
) {
    for (mut velocity, mut sprite) in query.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowRight) {
            velocity.linvel.x = PLAYER_SPEED;
            sprite.flip_x = false;
        } else if keyboard.pressed(KeyCode::ArrowLeft) {
            velocity.linvel.x = -PLAYER_SPEED;
            sprite.flip_x = true;
        } else {
            velocity.linvel.x = 0.0;
        }

        if keyboard.just_pressed(KeyCode::Space) {
            velocity.linvel.y = JUMP_FORCE;
        }
    }
}
pub fn apply_player_animation(
    time: Res<Time>,
    mut query: Query<(&mut StandardAnimation, &mut Sprite, &PlayerState)>,
) {
    for (mut animation, mut sprite, state) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let max_frames = match state {
                    PlayerState::Idle | PlayerState::Running => 11, // Idle.png 11 kareden oluştuğu için
                    PlayerState::Jumping => 1, // Jump.png sadece 1 kareden oluşuyor
                };

                atlas.index = (atlas.index + 1) % max_frames;
            }
        }
    }
}

pub fn update_player_animation(
    mut query: Query<(&mut Sprite, &PlayerState, &PlayerAnimationHandles)>,
) {
    for (mut sprite, state, handles) in query.iter_mut() {
        match state {
            PlayerState::Idle => {
                sprite.image = handles.idle.clone();
            }
            PlayerState::Running => {
                sprite.image = handles.running.clone();
            }
            PlayerState::Jumping => {
                sprite.image = handles.jumping.clone();
            }
        }
    }
}

pub fn update_player_animation_state(mut query: Query<(&mut PlayerState, &Velocity)>) {
    for (mut state, velocity) in query.iter_mut() {
        if velocity.linvel.y > 1.0 {
            *state = PlayerState::Jumping;
        } else if velocity.linvel.x.abs() > 0.1 {
            *state = PlayerState::Running;
        } else {
            *state = PlayerState::Idle;
        }
    }
}

pub fn play_barrel_animation(
    time: Res<Time>,
    mut query: Query<(&mut StandardAnimation, &mut Sprite, &BarrelState)>,
) {
    for (mut animation, mut sprite, state) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let max_frames = match state {
                    BarrelState::Idle | BarrelState::Walking => 11, // Idle.png 11 kareden oluştuğu için
                };

                atlas.index = (atlas.index + 1) % max_frames;
            }
        }
    }
}

pub fn update_barrel_animation(
    mut query: Query<(&mut Sprite, &BarrelState, &BarrelAnimationHandles)>,
) {
    for (mut sprite, state, handles) in query.iter_mut() {
        match state {
            BarrelState::Idle => {
                sprite.image = handles.idle.clone();
            }
            BarrelState::Walking => {
                sprite.image = handles.walking.clone();
            }
        }
    }
}

pub fn update_barrel_animation_state(mut query: Query<(&mut BarrelState, &Velocity)>) {
    for (mut state, velocity) in query.iter_mut() {
        if velocity.linvel.x.abs() > 0.1 {
            *state = BarrelState::Walking;
        } else {
            *state = BarrelState::Idle;
        }
    }
}

pub fn barrel_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut BarrelDriver, &mut Velocity, &mut Sprite), With<Barrel>>,
) {
    let mut rng = rand::rng();

    for (mut driver, mut velocity, mut sprite) in query.iter_mut() {
        driver.timer.tick(time.delta());

        match driver.state {
            BarrelMoveState::Moving => {
                velocity.linvel.x = BARREL_WALK_SPEED * driver.direction.x;
                sprite.flip_x = driver.direction.x > 0.0;
            }
            BarrelMoveState::Stopped => {
                velocity.linvel.x = 0.0;
            }
        }

        if driver.timer.just_finished() {
            match driver.state {
                BarrelMoveState::Moving => {
                    if rng.random_bool(0.8) {
                        driver.state = BarrelMoveState::Stopped;
                        let stop_time = rng.random_range(1.0..2.0);
                        driver
                            .timer
                            .set_duration(Duration::from_secs_f32(stop_time));
                        driver.timer.reset();
                    } else {
                        if rng.random_bool(0.4) {
                            driver.direction = -driver.direction;
                        }
                        let move_time = rng.random_range(2.0..4.0);
                        driver
                            .timer
                            .set_duration(Duration::from_secs_f32(move_time));
                        driver.timer.reset();
                    }
                }
                BarrelMoveState::Stopped => {
                    driver.state = BarrelMoveState::Moving;

                    driver.direction = if rng.random_bool(0.5) {
                        Vec2::new(1.0, 0.0)
                    } else {
                        Vec2::new(-1.0, 0.0)
                    };

                    let move_time = rng.random_range(2.0..4.0);
                    driver
                        .timer
                        .set_duration(Duration::from_secs_f32(move_time));
                    driver.timer.reset();
                }
            }
        }
    }
}
